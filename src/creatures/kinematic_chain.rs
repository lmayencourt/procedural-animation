/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::corbusier_colors::*;

#[derive(Component, Debug)]
pub struct KinematicChain {
    pub anchor: Option<Vec3>,
    pub target: Vec3,
    pub nodes: Vec<(Vec3, f32)>,
    pub distance: f32,
    pub skin: Vec<Vec2>,
}

impl KinematicChain {
    pub fn new(count: usize, distance: f32, anchor: Option<Vec3>) -> Self {
        let mut nodes = Vec::<(Vec3, f32)>::new();
        for _ in 0..count {
            nodes.push((Vec3::new(0.0, distance, 0.0), distance));
        }
        KinematicChain {
            anchor,
            target: Vec3::new(0.0, distance*count as f32, 0.0),
            distance,
            nodes,
            skin: Vec::new(),
        }
    }

    pub fn fish_like(count: usize, distance: f32, anchor: Option<Vec3>) -> Self {
        let mut nodes = Vec::<(Vec3, f32)>::new();
        // first 1/6 nodes rises from size to create an head like shape
        let nose = count / 6;
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
            anchor,
            target: Vec3::new(200.0, 0.0, 0.0),
            distance,
            nodes,
            skin: Vec::new(),
        }
    }
}

pub fn reach_target(
    mut squeletons: Query<(&mut KinematicChain, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (mut squeleton, t_global) in squeletons.iter_mut() {
        // perform a few iteration to stabilize before drawing body
        for i in 0..10 {
            let target = squeleton.target;
            if let Some(head) = squeleton.nodes.first_mut() {
                head.0 = target;
            }

            forward_kinematics(&mut squeleton, &mut gizmos);

            if let Some(anchor) = squeleton.anchor {
                if let Some(tail) = squeleton.nodes.last_mut() {
                    tail.0 = anchor;
                }
                backward_kinematics(&mut squeleton);
            }

            if squeleton_angles_are_ok(&mut squeleton, &mut gizmos) {
                if i > 0 {
                    debug!("All good at {}", i);
                }
                break;
            } else {
                correct_angle(&mut squeleton, &mut gizmos);
                debug!("Correcting angle {}", i);
            }
        }
        compute_skin(&mut squeleton, &t_global.translation(), &mut gizmos);
    }
}

fn forward_kinematics(squeleton: &mut KinematicChain, gizmos: &mut Gizmos) {
    let node_distance = squeleton.distance;
    let mut iter = squeleton.nodes.iter_mut().peekable();

    loop {
        if let Some(head) = iter.next() {
            if let Some(tail) = iter.peek_mut() {
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

                gizmos.circle_2d(head.0.truncate(), head.1, COLOR_WHITE);
                gizmos.circle_2d(tail.0.truncate(), tail.1, COLOR_WHITE);
                gizmos.line(head.0, tail.0, COLOR_WHITE);

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

fn backward_kinematics(squeleton: &mut KinematicChain) {
    let node_distance = squeleton.distance;
    let mut iter = squeleton.nodes.iter_mut().rev().peekable();

    loop {
        if let Some(head) = iter.next() {
            if let Some(tail) = iter.peek_mut() {
                debug!("Looking at {:?} {:?}", head, tail);

                let distance = head.0.distance(tail.0);
                if let Ok(direction) = Dir2::new((tail.0 - head.0).truncate().normalize()) {
                    let ray = Ray2d {
                        origin: head.0.truncate(),
                        direction,
                    };

                    if distance > node_distance {
                        let new_position = ray.origin + *ray.direction * node_distance;
                        tail.0.x = new_position.x;
                        tail.0.y = new_position.y;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}
use std::cell::Cell;

fn squeleton_angles_are_ok(squeleton: &mut KinematicChain, gizmos: &mut Gizmos) -> bool {
    for (i, nodes) in squeleton.nodes.windows(3).enumerate() {
        let n0 = nodes[0].0;
        let n1 = nodes[1].0;
        let n2 = nodes[2].0;

        let segment_1 = n1 - n0;
        let segment_2 = n2 - n1;

        let angle = segment_1.angle_between(segment_2).to_degrees();
        let cross_product = segment_1.cross(segment_2);
        let clockwise = Vec3::Z.angle_between(cross_product) > 0.0;

        let max_angle: f32 = 20.0;

        if angle > max_angle {
            debug!("angle {} is {}, {}", i, angle, clockwise);
            return false;
        }
    }

    true
}

fn correct_angle(squeleton: &mut KinematicChain, gizmos: &mut Gizmos) {
    let mut points = Vec::new();
    for point in &mut squeleton.nodes {
        points.push(point.0);
    }
    let slice = &mut points[..];
    let slice_of_cells: &[Cell<Vec3>] = Cell::from_mut(slice).as_slice_of_cells();
    for (i, nodes) in slice_of_cells.windows(3).enumerate() {
        let n0 = nodes[0].get();
        let n1 = nodes[1].get();
        let n2 = nodes[2].get();

        let segment_1 = n1 - n0;
        let segment_2 = n2 - n1;

        let angle = segment_1.angle_between(segment_2).to_degrees();
        let cross_product = segment_1.cross(segment_2);
        let clockwise = Vec3::Z.angle_between(cross_product) > 0.0;
        debug!("angle {} is {}, {}", i, angle, clockwise);

        // gizmos.arrow_2d(Vec2::ZERO, segment_1.truncate(), COLOR_GREEN);
        // gizmos.arrow_2d(Vec2::ZERO, segment_2.truncate(), COLOR_GREEN);

        let max_angle: f32 = 20.0;

        if angle > max_angle {
            // gizmos.arrow_2d(n1.truncate(), segment_1.truncate() + n1.truncate(), COLOR_BLUE);
            // gizmos.arrow_2d(n1.truncate(), segment_2.truncate() + n1.truncate(), COLOR_RED);

            let quat = if clockwise {
                Quat::from_axis_angle(Vec3::Z, -max_angle.to_radians())
            } else {
                Quat::from_axis_angle(Vec3::Z, max_angle.to_radians())
            };
            let rotated_segment = quat * segment_1.normalize() * segment_2.length();
            let point = rotated_segment + n1;
            nodes[2].set(point);

            // gizmos.arrow_2d(Vec2::ZERO, rotated_segment.truncate(), COLOR_BLUE);
            gizmos.arrow_2d(n1.truncate(), rotated_segment.truncate() + n1.truncate(), COLOR_BLUE);
        }
    }

    // Resign the points to the squeleton
    for (i, point) in points.iter().enumerate() {
        squeleton.nodes[i].0 = *point;
    }
}

fn compute_skin(squeleton: &mut KinematicChain, t_global: &Vec3, gizmos: &mut Gizmos) {
    let node_distance = squeleton.distance;
    let mut iter = squeleton.nodes.iter_mut().enumerate().peekable();

    let mut skin_left = Vec::<Vec2>::new();
    let mut skin_right = Vec::<Vec2>::new();
    let mut skin_head_tail = Vec::<Vec2>::new();

    let offset: Vec3;
    if let Some((_, head)) = iter.peek() {
        if head.0 != *t_global {
            offset = *t_global;
        } else {
            offset = Vec3::ZERO;
        }
    } else {
        offset = Vec3::ZERO;
    }

    loop {
        if let Some((i, head)) = iter.next() {
            if let Some((_, tail)) = iter.peek_mut() {
                debug!("Looking at {:?} {:?}", head, tail);

                let ray = Ray2d {
                    origin: head.0.truncate() - offset.truncate(),
                    direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
                };

                // Compute the skin points
                let front = ray.origin + -*ray.direction * head.1;
                let left = ray.origin + ray.direction.perp() * head.1;
                let right = ray.origin + -ray.direction.perp() * head.1;
                let back = ray.origin + ray.direction * head.1;

                if i == 0 {
                    for angle in (15..=60).step_by(15) {
                        let rotation_angle = (angle as f32).to_radians();
                        let rotation_vec = Vec2::new(rotation_angle.cos(), rotation_angle.sin());

                        let point = ray.origin + (-ray.direction.rotate(rotation_vec)).normalize() * head.1;
                        // gizmos.line_2d(ray.origin, point, COLOR_GREEN);
                        // gizmos.circle_2d(point, 4.0, COLOR_GREEN);
                        skin_right.push(point);
                    }

                    for angle in (-60..=-15).rev().step_by(15) {
                        let rotation_angle = (angle as f32).to_radians();
                        let rotation_vec = Vec2::new(rotation_angle.cos(), rotation_angle.sin());

                        let point = ray.origin + (-ray.direction.rotate(rotation_vec)).normalize() * head.1;
                        // gizmos.line_2d(ray.origin, point, COLOR_RED);
                        // gizmos.circle_2d(point, 4.0, COLOR_RED);
                        skin_left.push(point);
                    }
                }

                skin_left.push(left);
                skin_right.push(right);
                skin_head_tail.push(front);
                skin_head_tail.push(back);

                // gizmos.circle_2d(left, 2.0, COLOR_RED);
                // gizmos.circle_2d(right, 2.0, COLOR_GREEN);
                // gizmos.circle_2d(front, 2.0, COLOR_RED);
                // gizmos.circle_2d(back, 2.0, COLOR_RED);
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
                origin: tail.0.truncate() - offset.truncate(),
                direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
            };

            // Compute the skin points
            let left = ray.origin + ray.direction.perp() * tail.1;
            let right = ray.origin + -ray.direction.perp() * tail.1;
            let back = ray.origin + ray.direction * tail.1;

            skin_left.push(left);
            skin_right.push(right);
            skin_head_tail.push(back);

            // gizmos.circle_2d(back + offset.truncate(), 5.0, COLOR_RED);
            // gizmos.circle_2d(left, 5.0, COLOR_RED);
            // gizmos.circle_2d(right, 5.0, COLOR_BLUE);
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
