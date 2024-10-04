/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::corbusier_colors::*;
use crate::creatures::body_parts::*;
use crate::creatures::Creature;

/// Moving leg following body position
#[derive(Component)]
pub struct Leg {
    pub anchor: usize,
    pub position: BodyPartPosition,
}

pub fn clear_leg_rotation(mut q_legs: Query<&mut Transform, With<Leg>>) {
    for mut leg in q_legs.iter_mut() {
        leg.rotation = Quat::from_rotation_x(0.);
    }
}

pub fn draw_leg(
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&KinematicChain, &mut Children), With<Creature>>,
    mut q_legs: Query<(&mut Leg, &mut KinematicChain, &mut Transform), Without<Creature>>,
) {
    for (squeleton, children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((leg, mut chain, mut transform)) = q_legs.get_mut(child) {
                let anchor_node = squeleton.nodes[leg.anchor];
                let anchor_head = squeleton.nodes[leg.anchor - 1];

                *transform = get_attachment_position(anchor_node, anchor_head, &leg.position);

                gizmos.circle_2d(transform.translation.truncate(), 5.0, COLOR_GREEN);

                // calculate the position of the foot
                chain.anchor = Some(transform.translation);

                let leg_length = chain.distance * chain.nodes.len() as f32;
                let foot_direction = get_perpendicular_body_ray(anchor_node, anchor_head);
                let middle_position;
                match leg.position {
                    BodyPartPosition::Left => {
                        middle_position = anchor_node.0.truncate()
                            - foot_direction.direction * leg_length / std::f32::consts::SQRT_2
                    }
                    BodyPartPosition::Right => {
                        middle_position = anchor_node.0.truncate()
                            + foot_direction.direction * leg_length / std::f32::consts::SQRT_2
                    }
                    BodyPartPosition::Dorsal => middle_position = anchor_node.0.truncate(),
                }

                let top_position = middle_position
                    - foot_direction.direction.perp() * leg_length / std::f32::consts::SQRT_2;
                let bottom_position = middle_position
                    + foot_direction.direction.perp() * leg_length / std::f32::consts::SQRT_2;
                // gizmos.circle_2d(middle_position, 5.0 , COLOR_GREEN);
                // gizmos.circle_2d(top_position, 5.0 , COLOR_RED);
                // gizmos.circle_2d(bottom_position, 5.0 , COLOR_RED);

                let distance = chain.target.distance(anchor_head.0);
                if distance > leg_length {
                    chain.target = top_position.extend(0.0);
                }
            }
        }
    }
}
