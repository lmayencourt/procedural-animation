/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::corbusier_colors::*;

use crate::MyWorldCoords;

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, follow_path);
        app.add_systems(Update, add_points);
    }
}

#[derive(Component)]
struct Path(CubicCurve<Vec3>);

#[derive(Component)]
struct PathComponents(Vec<Vec3>);

#[derive(Component)]
struct PathProgress(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let points = vec![[
        Vec3::new(-60., -120., 0.),
        Vec3::new(-350., 150., 0.),
        Vec3::new(350., 150., 0.),
        Vec3::new(60., -120., 0.),
    ]];

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(30.0, 30.0))),
            material: materials.add(COLOR_LIGHT_BLUE),
            ..default()
        },
        // Path(bezier),
        PathComponents(vec![Vec3::ZERO]),
        PathProgress(0.0),
    ));
}

fn follow_path (
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PathComponents, &mut PathProgress)>,
) {    
    for (mut transform, mut points, mut progress) in &mut query {

        // Draw the points for reference
        for point in &points.0 {
            gizmos.circle_2d(point.truncate(), 5.0, COLOR_GREEN);
        }

        let target: f32 = (points.0.len() -1) as f32;
        if progress.0 < target {
            progress.0 += time.delta_seconds() * 0.4;
        }

        if progress.0 > target {
            // We reached the last point. Clear the path
            points.0.clear();
            points.0.push(transform.translation);
        }

        if points.0.len() > 1 {

            println!("progress is {}/{}", progress.0, target);

            let bezier = CubicCardinalSpline::new(0.5, points.0.clone()).to_curve();
            gizmos.linestrip(bezier.iter_positions(points.0.len() * 50), COLOR_WHITE);

            // This version needs at least 3 points. It is smoother, but doesn't pass
            // through the control point position.
            // let bezier = CubicBSpline::new(points.0.clone()).to_curve();
            // gizmos.linestrip(bezier.iter_positions(points.0.len() * 50), COLOR_RED);

            // position takes a point from the curve where 0 is the initial point
            // and 1 is the last point
            // transform.translation = bezier.position(progress.0 * bezier.segments().len() as f32);
            transform.translation = bezier.position(progress.0);
        }
    }
}

fn add_points (
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Transform, &mut PathComponents, &mut PathProgress)>,
    mut mycoords: ResMut<MyWorldCoords>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (pos, mut points, mut progress) = query.single_mut();
        points.0.push(mycoords.0.extend(0.0));
        // change the first point to be the current position
        let first = points.0.first_mut().unwrap();
        *first = pos.translation;
        progress.0 = 0.0;
    }

    if buttons.just_pressed(MouseButton::Right) {
        let (pos, mut points, mut progress) = query.single_mut();
        points.0.clear();
        points.0.push(pos.translation);
        progress.0 = 0.0;
    }
}

    // This draw a heart based on mesh !

    // let mut path_builder = PathBuilder::new();
    // path_builder.move_to(Vec2::new(0., 0.));
    // path_builder.cubic_bezier_to(
    //     Vec2::new(70., 70.),
    //     Vec2::new(175., -35.),
    //     Vec2::new(0., -140.),
    // );
    // path_builder.cubic_bezier_to(
    //     Vec2::new(-175., -35.),
    //     Vec2::new(-70., 70.),
    //     Vec2::new(0., 0.),
    // );
    // path_builder.close();
    // let path = path_builder.build();

    // commands.spawn((
    //     ShapeBundle {
    //         path,
    //         spatial: SpatialBundle {
    //             transform: Transform::from_xyz(0., 75., 0.),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Stroke::new(COLOR_BLACK, 10.0),
    //     Fill::color(COLOR_RED),
    // ));