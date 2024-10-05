/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_prototype_lyon::prelude::*;

mod corbusier_colors;
mod creatures;
mod water_effect;

use creatures::{kinematic_chain::KinematicChain, Creature, CreaturesPlugin};
use water_effect::WaterEffectPlugin;
use corbusier_colors::*;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(CreaturesPlugin)
        .add_plugins(WaterEffectPlugin)
        .add_systems(Startup, setup)
        .init_resource::<MyWorldCoords>()
        .add_systems(Update, my_cursor_system)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, enable_gizmos)
        .add_systems(Update, adapt_windows_size)
        .add_systems(Update, follow_circle)
        .run();
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2dBundle::default());

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = false;
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
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

fn follow_circle(
    time: Res<Time>,
    mut q_squeleton: Query<&mut KinematicChain, With<Creature>>,
    mut gizmos: Gizmos,
) {
    let circle_radius = 150.0;
    gizmos.circle_2d(Vec2::ZERO, circle_radius, COLOR_WHITE);

    let mut squeleton = q_squeleton.single_mut();

    let t = time.elapsed_seconds();
    squeleton.target = Vec3::new(f32::cos(t), f32::sin(t), 0.0);
    squeleton.target *= circle_radius;
    squeleton.target.x += f32::cos(t*5.0) * 10.0;
    squeleton.target.y += f32::sin(t*5.0) * 10.0;
}

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut mycoords: ResMut<MyWorldCoords>,
    mut squeleton: Query<&mut KinematicChain, With<Creature>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut squeleton = squeleton.single_mut();

        squeleton.target = mycoords.0.extend(0.0);

        // if let Some(head) = squeleton.nodes.first_mut() {
        //     head.0.x = mycoords.0.x;
        //     head.0.y = mycoords.0.y;
        // }
    }

    for finger in touches.iter() {
        // if touches.just_pressed(finger.id()) {
        //     println!("A new touch with ID {} just began.", finger.id());
        // }

        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            mycoords.0 = world_position;
        }

        if let Some(position) = camera
            .viewport_to_world(camera_transform, finger.position())
            .map(|ray| ray.origin.truncate())
        {
            let mut squeleton = squeleton.single_mut();

            squeleton.target = position.extend(0.0);
        }
    }
}

fn enable_gizmos(mut config_store: ResMut<GizmoConfigStore>, keyboard: Res<ButtonInput<KeyCode>>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    if keyboard.just_pressed(KeyCode::KeyD) {
        config.enabled ^= true;
    }
}

fn adapt_windows_size(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for event in resize_reader.read() {
        let mut window = q_window.single_mut();
        window.resolution.set(event.width, event.height);
    }
}
