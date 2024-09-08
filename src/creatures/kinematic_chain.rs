/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::corbusier_colors::*;

#[derive(Component)]
pub struct KinematicChain {
    pub target: Vec3,
    pub nodes: Vec<(Vec3, f32)>,
    pub distance: f32,
    pub skin: Vec<Vec2>,
}

impl KinematicChain {
    pub fn new(count: usize, distance: f32) -> Self {
        let mut nodes = Vec::<(Vec3, f32)>::new();
        // first 1/6 nodes rises from size to create an head like shape
        let nose = count/6;
        for n in 0..nose {
            nodes.push((
                Vec3::new(0.0, distance * n as f32, 0.0),
                (n + count - nose) as f32 * distance / 8.0,
            ));
        }
        // Then we have the body linearly decreasing in size
        for n in nose..count {
            nodes.push((
                Vec3::new(0.0, distance * n as f32, 0.0),
                (count - n) as f32 * distance / 8.0,
            ));
        }

        KinematicChain {
            target: Vec3::splat(0.0),
            distance,
            nodes,
            skin: Vec::new(),
        }
    }
}

pub fn reach_target(
    mut squeletons: Query<&mut KinematicChain>,
    mut gizmos: Gizmos,
) {
    // for now directly assign target to head
    for mut squeleton in squeletons.iter_mut() {
        let target = squeleton.target;
        if let Some(head) = squeleton.nodes.first_mut() {
            head.0 = target;
        }

        forward_kinematics(&mut squeleton, &mut gizmos);
        compute_skin(&mut squeleton, &mut gizmos);
    }
}

fn forward_kinematics(squeleton: &mut KinematicChain, gizmos: &mut Gizmos) {
    let node_distance = squeleton.distance;
    let mut iter = squeleton.nodes.iter_mut().peekable();

    loop {
        if let Some(head) = iter.next() {
            if let Some(mut tail) = iter.peek_mut() {
                debug!("Looking at {:?} {:?}", head, tail);

                let distance = head.0.distance(tail.0);
                let ray = Ray2d {
                    origin: head.0.truncate(),
                    direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
                };

                if distance > node_distance {
                    let new_position = ray.origin + *ray.direction * node_distance;
                    tail.0.x = new_position.x;
                    tail.0.y = new_position.y;
                }

                gizmos.line_2d(
                    ray.origin,
                    ray.origin + *ray.direction * distance,
                    COLOR_WHITE,
                );
                gizmos.circle_2d(head.0.truncate(), head.1, COLOR_WHITE);
                gizmos.circle_2d(tail.0.truncate(), tail.1, COLOR_WHITE);

                // gizmos.circle_2d(front, 5.0, COLOR_WHITE);
                // gizmos.circle_2d(left, 5.0, COLOR_WHITE);
                // gizmos.circle_2d(right, 5.0, COLOR_BLUE);
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn compute_skin(squeleton: &mut KinematicChain, gizmos: &mut Gizmos) {
    let node_distance = squeleton.distance;
    let mut iter = squeleton.nodes.iter_mut().peekable();

    let mut skin_left = Vec::<Vec2>::new();
    let mut skin_right = Vec::<Vec2>::new();
    let mut skin_head_tail = Vec::<Vec2>::new();

    loop {
        if let Some(head) = iter.next() {
            if let Some(mut tail) = iter.peek_mut() {
                debug!("Looking at {:?} {:?}", head, tail);

                let ray = Ray2d {
                    origin: head.0.truncate(),
                    direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
                };

                // Compute the skin points
                let front = ray.origin + -*ray.direction * head.1;
                let left = ray.origin + ray.direction.perp() * head.1;
                let right = ray.origin + -ray.direction.perp() * head.1;
                let back = ray.origin + ray.direction.perp() * head.1;

                skin_left.push(left);
                skin_right.push(right);
                skin_head_tail.push(front);
                skin_head_tail.push(back);

            } else {
                break;
            }
        } else {
            break;
        }
    }

    // Compute the last segment by looking at the 2 lasts nodes
    let mut back_iter = squeleton.nodes.iter().rev().peekable();
    if let Some(tail) = back_iter.next() {
        if let Some(head) = back_iter.peek() {
            let ray = Ray2d {
                origin: tail.0.truncate(),
                direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
            };

            // Compute the skin points
            let left = ray.origin + ray.direction.perp() * tail.1;
            let right = ray.origin + -ray.direction.perp() * tail.1;
            let back = ray.origin + ray.direction * tail.1;

            skin_left.push(left);
            skin_right.push(right);
            skin_head_tail.push(back);

            gizmos.circle_2d(back, 5.0, COLOR_WHITE);
            gizmos.circle_2d(left, 5.0, COLOR_WHITE);
            gizmos.circle_2d(right, 5.0, COLOR_BLUE);
        }
    }

    // Combine all the skin point
    if let Some(tail) = skin_head_tail.last() {
        skin_right.push(*tail);
    }
    skin_right.reverse();
    // push the head front at the bottom of the reversed right, to make it at the top of the vec
    if let Some(head) = skin_head_tail.first() {
        skin_right.push(*head);
    }
    skin_left.append(&mut skin_right);
    squeleton.skin = skin_left;
}