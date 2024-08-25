/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy::window::PrimaryWindow;

pub const COLOR_BLUE: Color = Color::rgb(132.0/255.0, 166.0/255.0, 199.0/255.0);
pub const COLOR_WHITE: Color = Color::rgb(233.0/255.0, 228.0/255.0, 217.0/255.0);

#[derive(Component)]
struct Anchor;

#[derive(Component)]
struct Chain;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_resource::<MyWorldCoords>()
        .add_systems(Update, my_cursor_system)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, follow_anchor)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(meshes.add(Circle{radius: 10.0})),
        material: materials.add(COLOR_BLUE),
        ..default()
    })
        .insert(Anchor)
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle{
                mesh: Mesh2dHandle(meshes.add(Annulus::new(98.0, 100.0))),
                material: materials.add(COLOR_WHITE),
                ..default()
            });
        })
    ;

    commands.spawn(MaterialMesh2dBundle{
        mesh: Mesh2dHandle(meshes.add(Circle{radius: 10.0})),
        material: materials.add(COLOR_WHITE),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    }).insert(Chain);
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    mut mycoords: ResMut<MyWorldCoords>,
    mut anchor: Query<&mut Transform, With<Anchor>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut anchor = anchor.single_mut();

        anchor.translation.x = mycoords.0.x;
        anchor.translation.y = mycoords.0.y;

    }
}

fn follow_anchor(
    mut anchor: Query<&mut Transform, With<Anchor>>,
    mut chains: Query<&mut Transform, (With<Chain>, Without<Anchor>)>,
    mut gizmos: Gizmos,
){
    let anchor = anchor.single();
    for mut chain in chains.iter_mut() {
        let distance = anchor.translation.distance(chain.translation);

        let ray = Ray2d {
            origin: anchor.translation.truncate(),
            direction: Dir2::new_unchecked((chain.translation - anchor.translation).truncate().normalize()),
        };

        gizmos.line_2d(ray.origin, ray.origin + *ray.direction * distance, COLOR_WHITE);

        if distance > 100.0 {
            let new_position = ray.origin + *ray.direction * 100.0;
            chain.translation = Vec3::new(new_position.x, new_position.y, 0.0);
        }
    }
}