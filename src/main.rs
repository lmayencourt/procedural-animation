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

#[derive(Component)]
struct Squeleton {
    nodes: Vec<Vec3>,
    distance: f32,
}

impl Squeleton {
    fn new(count: usize, distance: f32) -> Self {
        let mut nodes = Vec::<Vec3>::new();
        for n in 0..count {
            nodes.push(Vec3::new(0.0, distance * n as f32, 0.0));
        }

        Squeleton {
            distance,
            nodes
        }
    }
}

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

    commands.spawn(Squeleton::new(5, 50.0));
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
    mut squeleton: Query<&mut Squeleton>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut squeleton = squeleton.single_mut();

        if let Some(head) = squeleton.nodes.first_mut() {
            head.x = mycoords.0.x;
            head.y = mycoords.0.y;
        }

    }
}

fn follow_anchor(
    mut squeletons: Query<&mut Squeleton>,
    mut gizmos: Gizmos,
){
    for mut squeleton in squeletons.iter_mut() {
        
        let node_distance = squeleton.distance;
        let mut iter = squeleton.nodes.iter_mut().peekable();

        loop {
            if let Some(head) = iter.next() {
                if let Some(mut tail) = iter.peek_mut() {
                    info!("Looking at {:?} {:?}", head, tail);
                    let distance = head.distance(**tail);
                    let ray = Ray2d {
                        origin: head.truncate(),
                        direction: Dir2::new_unchecked((**tail - *head).truncate().normalize()),
                    };

                    if distance > node_distance {
                        let new_position = ray.origin + *ray.direction * node_distance;
                        tail.x = new_position.x;
                        tail.y = new_position.y;
                    }

                    gizmos.line_2d(ray.origin, ray.origin + *ray.direction * distance, COLOR_WHITE);
                    gizmos.circle(*head, Dir3::Z, 10.0, COLOR_WHITE);
                    gizmos.circle(**tail, Dir3::Z, 10.0, COLOR_WHITE);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}