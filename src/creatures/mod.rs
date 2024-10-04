/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::corbusier_colors::*;

mod body_parts;
pub mod kinematic_chain;

use crate::creatures::kinematic_chain::{reach_target, KinematicChain};
use body_parts::eye::Eye;
use body_parts::fin::Fin;
use body_parts::leg::Leg;
use body_parts::*;

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, reach_target);
        app.add_systems(Update, draw_body);
        app.add_systems(Update, body_parts::fin::draw_fin);
        app.add_systems(Update, body_parts::eye::draw_eye);
        app.add_systems(Update, body_parts::leg::draw_leg);
        app.add_systems(Update, body_parts::leg::clear_leg_rotation);
    }
}

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
struct Skin;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let points = [
        Vec2::new(-1.0, -0.3),
        Vec2::new(1.5, 0.0),
        Vec2::new(0.0, 1.0),
    ]
    .map(|x| x * 100.);

    let shape = shapes::RoundedPolygon {
        points: points.into_iter().collect(),
        radius: 10.,
        closed: false,
    };

    commands
        .spawn((
            Creature,
            KinematicChain::new(20, 12.0, None),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(COLOR_BLUE),
        ))
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
            // Spawn 4 legs
            parent.spawn((
                Leg,
                BodyPartAnchor {
                    anchor: 3,
                    position: BodyPartPosition::Right,
                },
                KinematicChain::new(3, 20.0, None),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(COLOR_GREEN),
            ));
            parent.spawn((
                Leg,
                BodyPartAnchor {
                    anchor: 3,
                    position: BodyPartPosition::Left,
                },
                KinematicChain::new(3, 20.0, None),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(COLOR_GREEN),
            ));
            parent.spawn((
                Leg,
                BodyPartAnchor {
                    anchor: 13,
                    position: BodyPartPosition::Right,
                },
                KinematicChain::new(3, 20.0, None),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(COLOR_GREEN),
            ));
            parent.spawn((
                Leg,
                BodyPartAnchor {
                    anchor: 13,
                    position: BodyPartPosition::Left,
                },
                KinematicChain::new(3, 20.0, None),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(COLOR_GREEN),
            ));
        });
}

fn draw_body(
    mut gizmos: Gizmos,
    mut squeleton: Query<(&mut KinematicChain, &mut Path)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    skins: Query<Entity, With<Skin>>,
) {
    // let points = [[
    //     Vec3::new(-60., -120., 0.),
    //     Vec3::new(-520., 380., 0.),
    //     Vec3::new(520., 380., 0.),
    //     Vec3::new(60., -120., 0.),
    // ]];

    // Make a CubicCurve
    // let bezier = CubicBezier::new(points).to_curve();
    // gizmos.linestrip(bezier.iter_positions(50), COLOR_BLUE);

    // let curve = points[0];
    // for p in curve.iter() {
    //     gizmos.circle_2d(p.truncate(), 5.0, COLOR_BLUE);
    // }

    // Clear all the preview skin shapes
    // This is really not efficient as we re-create every circle at every frame....
    let skins = skins.into_iter();

    for skin in skins {
        commands.entity(skin).despawn();
    }

    for (squeleton, mut path) in squeleton.iter_mut() {
        // let t = (time.elapsed_seconds().sin() + 1.) / 2.;
        //     if let Some(head) = squeleton.nodes.first_mut() {
        //         head.0.x = bezier.position(t).x;
        //         head.0.y = bezier.position(t).y;
        //     }

        let points = Vec::<Vec2>::new();

        let points = squeleton.skin.clone();

        for point in &points {
            gizmos.circle_2d(*point, 5.0, COLOR_GREEN);
        }

        let shape = shapes::Polygon {
            points: points.into_iter().collect(),
            closed: false,
        };

        *path = GeometryBuilder::build_as(&shape);

        for node in &squeleton.nodes {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle { radius: node.1 })),
                    material: materials.add(COLOR_BLUE),
                    transform: Transform::from_translation(node.0),
                    ..default()
                },
                Skin,
            ));
        }
    }
}
