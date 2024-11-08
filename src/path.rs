/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::creatures::{kinematic_chain::KinematicChain, Creature};
use crate::corbusier_colors::*;

use crate::MyWorldCoords;

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup);
        app.add_systems(Update, follow_path);
        app.add_systems(Update, add_points);
        app.add_systems(Update, loop_path);
    }
}

#[derive(Component)]
struct Path(CubicCurve<Vec3>);

#[derive(Component)]
pub struct PathComponents(pub Vec<Vec3>);

#[derive(Component)]
pub struct PathProgress(pub f32);

#[derive(Component)]
pub struct PathLoop {
    pub points: Vec<Vec3>,
    pub next_idx: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let points = vec![
        Vec3::new(-60., -120., 0.),
        Vec3::new(-350., 150., 0.),
        Vec3::new(350., 150., 0.),
        Vec3::new(60., -120., 0.),
    ];

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(30.0, 30.0))),
            material: materials.add(COLOR_LIGHT_BLUE),
            ..default()
        },
        // Path(bezier),
        PathComponents(vec![Vec3::ZERO]),
        PathProgress(0.0),
        PathLoop{points, next_idx: 0},
    ));
}

fn follow_path (
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut query: Query<(&mut KinematicChain, &mut PathComponents, &mut PathProgress), With<Creature>>,
) {
    for (mut squeleton, mut points, mut progress) in &mut query {

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
            points.0.push(squeleton.target);
            progress.0 = 0.0;
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
            squeleton.target = bezier.position(progress.0);
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
        // Add the new final destination
        points.0.push(mycoords.0.extend(0.0));

        // Remove the already reached points and
        // change the first point to be the current position
        if progress.0 != 0.0 {
            points.0.drain(0..progress.0 as usize);
            let first = points.0.first_mut().unwrap();
            *first = pos.translation;
            progress.0 = 0.0;
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        let (pos, mut points, mut progress) = query.single_mut();
        points.0.clear();
        points.0.push(pos.translation);
        progress.0 = 0.0;
    }
}

fn loop_path (
    mut query: Query<(&mut PathLoop, &mut PathComponents, &mut PathProgress)>,
) {
    let (mut path_loop, mut points, mut progress) = query.single_mut();
    if progress.0 == 0.0 {
        // Initiate the path the the loop points
        points.0 = path_loop.points.clone();
    } else if progress.0 > 2.0 {
        // Remove the first point, after reaching the second,
        // in order to keep the curve smooth.
        points.0.drain(0..1);
        progress.0 = 1.0;

        // add new target point
        points.0.push(path_loop.points[path_loop.next_idx]);

        if path_loop.next_idx < path_loop.points.len()-1 {
            path_loop.next_idx += 1;
        } else {
            path_loop.next_idx = 0;
        }
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