/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
    diagnostic::FrameTimeDiagnosticsPlugin,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_prototype_lyon::prelude::*;

mod audio;
mod corbusier_colors;
mod creatures;
mod fps_counter;
mod water_effect;
mod path;

use audio::AudioPlugin;
use creatures::{kinematic_chain::KinematicChain, Creature, Playable, CreaturesPlugin};
use water_effect::{WaterEffectPlugin, TextureCamera};
use fps_counter::FpsDisplay;
use corbusier_colors::*;
use path::*;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(FpsDisplay)
        .add_plugins(ShapePlugin)
        .add_plugins(CreaturesPlugin)
        .add_plugins(WaterEffectPlugin)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup)
        .init_resource::<MyWorldCoords>()
        .add_systems(Update, my_cursor_system)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, enable_gizmos)
        .add_systems(Update, adapt_windows_size)
        // .add_systems(Update, follow_circle)
        .add_plugins(PathPlugin)
        .run();
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = false;

    for i in 0..10 {
        let fish = creatures::species::fish::Fish::random();
        let entity = fish.spawn(&mut commands, &mut meshes, &mut materials);
        let x_size = 600.0;
        let y_size = 300.0;
        commands.entity(entity).insert((
            Creature,
            PathComponents::new(vec![Vec3::ZERO]),
            PathLoop::random(5, -x_size..x_size, -y_size..y_size),
        ));
    }
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), Without<TextureCamera>>,
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
    mut q_squeleton: Query<&mut KinematicChain, With<Playable>>,
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut gizmos: Gizmos,
) {
    if !buttons.pressed(MouseButton::Left) {
        let circle_radius = 150.0;
        gizmos.circle_2d(Vec2::ZERO, circle_radius, COLOR_WHITE);

        if let Ok(mut squeleton) = q_squeleton.get_single_mut() {
            let t = time.elapsed_seconds();
            squeleton.target = Vec3::new(f32::cos(t), f32::sin(t), 0.0);
            squeleton.target *= circle_radius;
            squeleton.target.x += f32::cos(t*5.0) * 10.0;
            squeleton.target.y += f32::sin(t*5.0) * 10.0;
        }
    }
}

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut mycoords: ResMut<MyWorldCoords>,
    mut squeleton: Query<&mut KinematicChain, With<Playable>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Ok(mut squeleton) = squeleton.get_single_mut() {
            squeleton.target = mycoords.0.extend(0.0);

            // if let Some(head) = squeleton.nodes.first_mut() {
                //     head.0.x = mycoords.0.x;
                //     head.0.y = mycoords.0.y;
                // }
            }
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
