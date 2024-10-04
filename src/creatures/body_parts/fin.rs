/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::corbusier_colors::*;
use crate::creatures::body_parts::*;

/// Fix point Fin
#[derive(Component, Default)]
pub struct Fin {
    pub anchor: usize,
    pub position: BodyPartPosition,
}

pub fn draw_fin(
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&KinematicChain, &mut Children)>,
    mut q_fins: Query<(&Fin, &mut Transform)>,
) {
    for (squeleton, children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((fin, mut transform)) = q_fins.get_mut(child) {
                let anchor_node = squeleton.nodes[fin.anchor];
                let anchor_head = squeleton.nodes[fin.anchor - 1];

                let distance = anchor_head.0.distance(anchor_node.0);
                let ray = Ray2d {
                    origin: anchor_head.0.truncate(),
                    direction: Dir2::new_unchecked(
                        (anchor_node.0 - anchor_head.0).truncate().normalize(),
                    ),
                };

                let left = ray.origin + ray.direction.perp() * anchor_node.1;
                let right = ray.origin + -ray.direction.perp() * anchor_node.1;

                gizmos.line_2d(
                    ray.origin,
                    ray.origin + *ray.direction * distance,
                    COLOR_GREEN,
                );

                match fin.position {
                    BodyPartPosition::Dorsal => {
                        transform.translation = anchor_node.0;
                        transform.translation.z = 1.0;
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                    }
                    BodyPartPosition::Left => {
                        transform.translation = left.extend(-1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle - std::f32::consts::PI / 5.0);
                    }
                    BodyPartPosition::Right => {
                        transform.translation = right.extend(-1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle + std::f32::consts::PI / 5.0);
                    }
                }
            }
        }
    }
}
