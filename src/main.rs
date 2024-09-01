/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

pub const COLOR_BLUE: Color = Color::rgb(132.0 / 255.0, 166.0 / 255.0, 199.0 / 255.0);
pub const COLOR_LIGHT_BLUE: Color = Color::rgb(175.0 / 255.0, 188.0 / 255.0, 198.0 / 255.0);
pub const COLOR_GREEN: Color = Color::rgb(175.0 / 255.0, 192.0 / 255.0, 130.0 / 255.0);
pub const COLOR_RED: Color = Color::rgb(159.0 / 255.0, 75.0 / 255.0, 63.0 / 255.0);
pub const COLOR_WHITE: Color = Color::rgb(233.0 / 255.0, 228.0 / 255.0, 217.0 / 255.0);
pub const COLOR_BLACK: Color = Color::rgb(58.0 / 255.0, 59.0 / 255.0, 59.0 / 255.0);

#[derive(Component)]
struct Anchor;

#[derive(Component)]
struct Chain;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct Skin;

#[derive(Component)]
struct Squeleton {
    nodes: Vec<(Vec3, f32)>,
    distance: f32,
    skin: Vec<Vec2>,
}

impl Squeleton {
    fn new(count: usize, distance: f32) -> Self {
        let mut nodes = Vec::<(Vec3, f32)>::new();
        // first 1/6 nodes rises from size to create an head like shape
        let nose = count/6;
        for n in 0..nose {
            nodes.push((
                Vec3::new(0.0, distance * n as f32, 0.0),
                (n + count - nose) as f32 * distance / 8.0,
            ));
        }
        // Then we have the body linearly decreasing in size
        for n in nose..count {
            nodes.push((
                Vec3::new(0.0, distance * n as f32, 0.0),
                (count - n) as f32 * distance / 8.0,
            ));
        }

        Squeleton {
            distance,
            nodes,
            skin: Vec::new(),
        }
    }
}

#[derive(Default)]
enum BodyPartPosition {
    #[default]
    Dorsal,
    Left,
    Right,
}

#[derive(Component, Default)]
struct Fin {
    anchor: usize,
    position: BodyPartPosition,
}

#[derive(Component)]
struct Eye {
    anchor: usize,
    position: BodyPartPosition,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .init_resource::<MyWorldCoords>()
        .add_systems(Update, my_cursor_system)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, follow_anchor)
        .add_systems(Update, draw_body)
        .add_systems(Update, draw_fin)
        .add_systems(Update, draw_eye)
        .add_systems(Update, enable_gizmos)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn(Camera2dBundle::default());

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
            Squeleton::new(20, 12.0),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(COLOR_BLUE),
        ))
        .with_children(|parent| {
            parent.spawn((
                Fin {
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
                Fin {
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
                Fin {
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
                Fin {
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
                Fin {
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
                Eye {
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
                Eye {
                    anchor: 2,
                    position: BodyPartPosition::Right,
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Ellipse::new(3.0, 6.0))),
                    material: materials.add(COLOR_BLACK),
                    ..default()
                },
            ));
        });

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

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    mut mycoords: ResMut<MyWorldCoords>,
    mut squeleton: Query<&mut Squeleton>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut squeleton = squeleton.single_mut();

        if let Some(head) = squeleton.nodes.first_mut() {
            head.0.x = mycoords.0.x;
            head.0.y = mycoords.0.y;
        }
    }
}

fn follow_anchor(mut squeletons: Query<&mut Squeleton>, mut gizmos: Gizmos) {
    for mut squeleton in squeletons.iter_mut() {
        let node_distance = squeleton.distance;
        let mut iter = squeleton.nodes.iter_mut().peekable();

        let mut skin_left = Vec::<Vec2>::new();
        let mut skin_right = Vec::<Vec2>::new();
        let mut skin_head_tail = Vec::<Vec2>::new();

        loop {
            if let Some(head) = iter.next() {
                if let Some(mut tail) = iter.peek_mut() {
                    debug!("Looking at {:?} {:?}", head, tail);

                    let distance = head.0.distance(tail.0);
                    let ray = Ray2d {
                        origin: head.0.truncate(),
                        direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
                    };

                    if distance > node_distance {
                        let new_position = ray.origin + *ray.direction * node_distance;
                        tail.0.x = new_position.x;
                        tail.0.y = new_position.y;
                    }

                    gizmos.line_2d(
                        ray.origin,
                        ray.origin + *ray.direction * distance,
                        COLOR_WHITE,
                    );
                    gizmos.circle_2d(head.0.truncate(), head.1, COLOR_WHITE);
                    gizmos.circle_2d(tail.0.truncate(), tail.1, COLOR_WHITE);

                    // Compute the skin points
                    let front = ray.origin + -*ray.direction * head.1;
                    let left = ray.origin + ray.direction.perp() * head.1;
                    let right = ray.origin + -ray.direction.perp() * head.1;
                    let back = ray.origin + ray.direction.perp() * head.1;

                    skin_left.push(left);
                    skin_right.push(right);
                    skin_head_tail.push(front);
                    skin_head_tail.push(back);

                    gizmos.circle_2d(front, 5.0, COLOR_WHITE);
                    gizmos.circle_2d(left, 5.0, COLOR_WHITE);
                    gizmos.circle_2d(right, 5.0, COLOR_BLUE);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Compute the last segment by looking at the 2 lasts nodes
        let mut back_iter = squeleton.nodes.iter().rev().peekable();
        if let Some(tail) = back_iter.next() {
            if let Some(head) = back_iter.peek() {
                let ray = Ray2d {
                    origin: tail.0.truncate(),
                    direction: Dir2::new_unchecked((tail.0 - head.0).truncate().normalize()),
                };

                // Compute the skin points
                let left = ray.origin + ray.direction.perp() * tail.1;
                let right = ray.origin + -ray.direction.perp() * tail.1;
                let back = ray.origin + ray.direction * tail.1;

                skin_left.push(left);
                skin_right.push(right);
                skin_head_tail.push(back);

                gizmos.circle_2d(back, 5.0, COLOR_WHITE);
                gizmos.circle_2d(left, 5.0, COLOR_WHITE);
                gizmos.circle_2d(right, 5.0, COLOR_BLUE);
            }
        }

        // Combine all the skin point
        if let Some(tail) = skin_head_tail.last() {
            skin_right.push(*tail);
        }
        skin_right.reverse();
        // push the head front at the bottom of the reversed right, to make it at the top of the vec
        if let Some(head) = skin_head_tail.first() {
            skin_right.push(*head);
        }
        skin_left.append(&mut skin_right);
        squeleton.skin = skin_left;
    }
}

fn draw_body(
    mut gizmos: Gizmos,
    mut squeleton: Query<(&mut Squeleton, &mut Path)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    skins: Query<Entity, With<Skin>>,
) {
    let points = [[
        Vec3::new(-60., -120., 0.),
        Vec3::new(-520., 380., 0.),
        Vec3::new(520., 380., 0.),
        Vec3::new(60., -120., 0.),
    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(points).to_curve();
    gizmos.linestrip(bezier.iter_positions(50), COLOR_BLUE);

    let curve = points[0];
    for p in curve.iter() {
        gizmos.circle_2d(p.truncate(), 5.0, COLOR_BLUE);
    }

    let (squeleton, mut path) = squeleton.single_mut();

    // let t = (time.elapsed_seconds().sin() + 1.) / 2.;
    //     if let Some(head) = squeleton.nodes.first_mut() {
    //         head.0.x = bezier.position(t).x;
    //         head.0.y = bezier.position(t).y;
    //     }

    let mut points = Vec::<Vec2>::new();

    let points = squeleton.skin.clone();

    let shape = shapes::Polygon {
        points: points.into_iter().collect(),
        closed: false,
    };

    *path = GeometryBuilder::build_as(&shape);

    // Clear all the preview skin shapes
    // This is really not efficient as we re-create every circle at every frame....
    let skins = skins.into_iter();

    for skin in skins {
        commands.entity(skin).despawn();
    }

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

fn draw_fin(
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&Squeleton, &mut Children)>,
    mut q_fins: Query<(&Fin, &mut Transform)>,
) {
    for (squeleton, mut children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((fin, mut transform)) = q_fins.get_mut(child) {
                let anchor_node = squeleton.nodes[fin.anchor];
                let anchor_head = squeleton.nodes[fin.anchor - 1];
    
                let distance = anchor_head.0.distance(anchor_node.0);
                let ray = Ray2d {
                    origin: anchor_head.0.truncate(),
                    direction: Dir2::new_unchecked(
                        (anchor_node.0 - anchor_head.0).truncate().normalize(),
                    ),
                };
    
                let left = ray.origin + ray.direction.perp() * anchor_node.1;
                let right = ray.origin + -ray.direction.perp() * anchor_node.1;
    
                gizmos.line_2d(
                    ray.origin,
                    ray.origin + *ray.direction * distance,
                    COLOR_GREEN,
                );
    
                match fin.position {
                    BodyPartPosition::Dorsal => {
                        transform.translation = anchor_node.0;
                        transform.translation.z = 1.0;
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
                    }
                    BodyPartPosition::Left => {
                        transform.translation = left.extend(-1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI / 5.0);
                    }
                    BodyPartPosition::Right => {
                        transform.translation = right.extend(-1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::PI / 5.0);
                    }
                }
            }

        }
    }
}

fn draw_eye (
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&Squeleton, &mut Children)>,
    mut q_eye: Query<(&Eye, &mut Transform)>,
) {
    for (squeleton, mut children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((eye, mut transform)) = q_eye.get_mut(child) {
                let anchor_node = squeleton.nodes[eye.anchor];
                let anchor_head = squeleton.nodes[eye.anchor - 1];
    
                let distance = anchor_head.0.distance(anchor_node.0);
                let ray = Ray2d {
                    origin: anchor_head.0.truncate(),
                    direction: Dir2::new_unchecked(
                        (anchor_node.0 - anchor_head.0).truncate().normalize(),
                    ),
                };
    
                let left = ray.origin + ray.direction.perp() * anchor_node.1 * 0.75;
                let right = ray.origin + -ray.direction.perp() * anchor_node.1 *0.75;
    
                gizmos.line_2d(
                    ray.origin,
                    ray.origin + *ray.direction * distance,
                    COLOR_GREEN,
                );

                match eye.position {
                    BodyPartPosition::Dorsal => {
                        transform.translation = anchor_node.0;
                        transform.translation.z = 1.0;
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2);
                    }
                    BodyPartPosition::Left => {
                        transform.translation = left.extend(1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2 + 0.2);
                    }
                    BodyPartPosition::Right => {
                        transform.translation = right.extend(1.0);
                        let angle = ray.direction.to_angle();
                        transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2 - 0.2);
                    }
                }
            }

        }
    }
}

fn enable_gizmos(mut config_store: ResMut<GizmoConfigStore>, keyboard: Res<ButtonInput<KeyCode>>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    if keyboard.just_pressed(KeyCode::KeyD) {
        config.enabled ^= true;
    }
}
