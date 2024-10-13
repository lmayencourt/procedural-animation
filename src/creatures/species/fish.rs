/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::corbusier_colors::*;
use crate::creatures::body_parts::*;
use crate::creatures::body_parts::eye::Eye;
use crate::creatures::body_parts::fin::Fin;
use crate::creatures::kinematic_chain::KinematicChain;

#[derive(Bundle)]
pub struct Fish{
    spine: KinematicChain,
    skin: ShapeBundle,
    color: Fill,
}

impl Default for Fish {
    fn default() -> Self {
        let shape = shapes::RoundedPolygon {
            points: Vec::<Vec2>::new(),
            radius: 1.,
            closed: false,
        };

        Fish {
            spine: KinematicChain::new(20, 12.0, None),
            skin: ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
            color: Fill::color(COLOR_BLUE),
        }
    }
}

impl Fish {
    pub fn new(size: usize, color: Color) -> Fish {
        let shape = shapes::RoundedPolygon {
            points: Vec::<Vec2>::new(),
            radius: 1.,
            closed: false,
        };

        Fish {
            spine: KinematicChain::new(20, 12.0, None),
            skin: ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
            color: Fill::color(color),
        }
    }

    pub fn spawn (
        self,
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Entity {
        commands.spawn(self)
        .with_children(|parent| {
            parent.spawn((
                Fin,
                BodyPartAnchor {
                    anchor: 5,
                    position: BodyPartPosition::Left,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(15.0, 30.0))),
                    material: materials.add(COLOR_LIGHT_BLUE),
                    ..default()
                },
            ));
            parent.spawn((
                Fin,
                BodyPartAnchor {
                    anchor: 5,
                    position: BodyPartPosition::Right,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(15.0, 30.0))),
                    material: materials.add(COLOR_LIGHT_BLUE),
                    ..default()
                },
            ));
            parent.spawn((
                Fin,
                BodyPartAnchor {
                    anchor: 7,
                    position: BodyPartPosition::Dorsal,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(6.0, 25.0))),
                    material: materials.add(COLOR_LIGHT_BLUE),
                    ..default()
                },
            ));
            parent.spawn((
                Fin,
                BodyPartAnchor {
                    anchor: 18,
                    position: BodyPartPosition::Left,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(6.0, 12.0))),
                    material: materials.add(COLOR_LIGHT_BLUE),
                    ..default()
                },
            ));
            parent.spawn((
                Fin,
                BodyPartAnchor {
                    anchor: 18,
                    position: BodyPartPosition::Right,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(6.0, 12.0))),
                    material: materials.add(COLOR_LIGHT_BLUE),
                    ..default()
                },
            ));
            // Draw Eye
            parent.spawn((
                Eye,
                BodyPartAnchor {
                    anchor: 2,
                    position: BodyPartPosition::Left,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(3.0, 6.0))),
                    material: materials.add(COLOR_BLACK),
                    ..default()
                },
            ));
            parent.spawn((
                Eye,
                BodyPartAnchor {
                    anchor: 2,
                    position: BodyPartPosition::Right,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(3.0, 6.0))),
                    material: materials.add(COLOR_BLACK),
                    ..default()
                },
            ));
        }).id()
    }
}