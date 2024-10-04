/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::corbusier_colors::*;
use crate::creatures::body_parts::*;
use crate::creatures::Creature;

/// Fix point eye
#[derive(Component)]
pub struct Eye {
    pub anchor: usize,
    pub position: BodyPartPosition,
}

pub fn draw_eye(
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&KinematicChain, &mut Children)>,
    mut q_eye: Query<(&Eye, &mut Transform)>,
) {
    for (squeleton, mut children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((eye, mut transform)) = q_eye.get_mut(child) {
                let anchor_node = squeleton.nodes[eye.anchor];
                let anchor_head = squeleton.nodes[eye.anchor - 1];

                let distance = anchor_head.0.distance(anchor_node.0);
                let ray = Ray2d {
                    origin: anchor_head.0.truncate(),
                    direction: Dir2::new_unchecked(
                        (anchor_node.0 - anchor_head.0).truncate().normalize(),
                    ),
                };

                let left = ray.origin + ray.direction.perp() * anchor_node.1 * 0.75;
                let right = ray.origin + -ray.direction.perp() * anchor_node.1 * 0.75;

                gizmos.line_2d(
                    ray.origin,
                    ray.origin + *ray.direction * distance,
                    COLOR_GREEN,
                );

                match eye.position {
                    BodyPartPosition::Dorsal => {
                        transform.translation = anchor_node.0;
                        transform.translation.z = 1.0;
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2);
                    }
                    BodyPartPosition::Left => {
                        transform.translation = left.extend(1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2 + 0.2);
                    }
                    BodyPartPosition::Right => {
                        transform.translation = right.extend(1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation =
                            Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2 - 0.2);
                    }
                }
            }
        }
    }
}
