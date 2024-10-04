/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
};

use crate::corbusier_colors::*;
use crate::creatures::Creature;
use crate::creatures::kinematic_chain::{KinematicChain};

pub mod eye;
pub mod fin;
pub mod leg;

use eye::*;
use fin::*;
use leg::*;

#[derive(Default)]
pub enum BodyPartPosition {
    #[default]
    Dorsal,
    Left,
    Right,
}

fn get_attachment_position(node: (Vec3, f32), head: (Vec3, f32), part_type: &BodyPartPosition) -> Transform
{
    // let distance = head.0.distance(node.0);
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // let ray = get_body_direction(node, head);

    let left = ray.origin + ray.direction.perp() * node.1;
    let right = ray.origin + -ray.direction.perp() * node.1;

    let mut transform = Transform::default();
    match part_type {
        BodyPartPosition::Dorsal => {
            transform.translation = node.0;
            transform.translation.z = 1.0;
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
        }
        BodyPartPosition::Left => {
            transform.translation = left.extend(-1.0);
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI/2.0);
        }
        BodyPartPosition::Right => {
            transform.translation = right.extend(-1.0);
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::PI/2.0);
        }
    }

    return transform;
}

fn get_body_direction(node: (Vec3, f32), head: (Vec3, f32)) -> Ray2d {
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // Left side
    return Ray2d {
        origin: ray.origin + ray.direction.perp() * node.1,
        direction: Dir2::new_unchecked(ray.direction.perp())
    }
}

fn get_perpendicular_body_ray(node: (Vec3, f32), head: (Vec3, f32)) -> Ray2d {
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // Left side
    return Ray2d {
        origin: ray.origin + ray.direction.perp() * node.1,
        direction: Dir2::new_unchecked(-ray.direction.perp())
    }
}