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
pub struct PathComponents{
    points: Vec<Vec3>,
    progress: f32,
}

impl PathComponents {
    pub fn new(points: Vec<Vec3>) -> Self {
        Self {
            points,
            progress: 0.0,
        }
    }
}

#[derive(Component)]
pub struct PathLoop {
    pub points: Vec<Vec3>,
    pub next_idx: usize,
}

impl PathLoop {
    pub fn new(points: Vec<Vec3>) -> Self {
        Self {
            points,
            next_idx: 0,
        }
    }
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
        PathComponents::new(vec![Vec3::ZERO]),
        PathLoop{points, next_idx: 0},
    ));
}

fn follow_path (
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut query: Query<(&mut KinematicChain, &mut PathComponents), With<Creature>>,
) {
    for (mut squeleton, mut path) in &mut query {

        // Draw the points for reference
        for point in &path.points {
            gizmos.circle_2d(point.truncate(), 5.0, COLOR_GREEN);
        }

        let target: f32 = (path.points.len() -1) as f32;
        if path.progress < target {
            path.progress += time.delta_seconds() * 0.4;
        }

        if path.progress > target {
            // We reached the last point. Clear the path
            path.points.clear();
            path.points.push(squeleton.target);
            path.progress = 0.0;
        }

        if path.points.len() > 1 {

            let bezier = CubicCardinalSpline::new(0.5, path.points.clone()).to_curve();
            gizmos.linestrip(bezier.iter_positions(path.points.len() * 50), COLOR_WHITE);

            // This version needs at least 3 points. It is smoother, but doesn't pass
            // through the control point position.
            // let bezier = CubicBSpline::new(path.points.clone()).to_curve();
            // gizmos.linestrip(bezier.iter_positions(path.points.len() * 50), COLOR_RED);

            // position takes a point from the curve where 0 is the initial point
            // and 1 is the last point
            // transform.translation = bezier.position(path.progress * bezier.segments().len() as f32);
            squeleton.target = bezier.position(path.progress);
        }
    }
}

fn add_points (
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&Transform, &mut PathComponents)>,
    mut mycoords: ResMut<MyWorldCoords>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (pos, mut path) = query.single_mut();
        // Add the new final destination
        path.points.push(mycoords.0.extend(0.0));

        // Remove the already reached points and
        // change the first point to be the current position
        if path.progress != 0.0 {
            let progress = path.progress;
            path.points.drain(0..progress as usize);
            let first = path.points.first_mut().unwrap();
            *first = pos.translation;
            path.progress = 0.0;
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        let (pos, mut path) = query.single_mut();
        path.points.clear();
        path.points.push(pos.translation);
        path.progress = 0.0;
    }
}

fn loop_path (
    mut query: Query<(&mut PathLoop, &mut PathComponents)>,
) {
    let (mut path_loop, mut path) = query.single_mut();
    if path.progress == 0.0 {
        // Initiate the path the the loop points
        path.points = path_loop.points.clone();
    } else if path.progress > 2.0 {
        // Remove the first point, after reaching the second,
        // in order to keep the curve smooth.
        path.points.drain(0..1);
        path.progress = 1.0;

        // add new target point
        path.points.push(path_loop.points[path_loop.next_idx]);

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