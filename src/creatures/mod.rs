/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::corbusier_colors::*;

pub mod kinematic_chain;

use crate::creatures::kinematic_chain::{KinematicChain, reach_target};

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, reach_target);
        app.add_systems(Update, draw_body);
        app.add_systems(Update, draw_fin);
        app.add_systems(Update, draw_eye);
        app.add_systems(Update, draw_leg);
    }
}

#[derive(Component)]
struct Skin;


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

#[derive(Component)]
struct Leg {
    anchor: usize,
    position: BodyPartPosition,
    nodes: Vec<(Vec3, f32)>,
}

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
        KinematicChain::new(20, 12.0),
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
        parent.spawn((
            Leg {
                anchor: 10,
                position: BodyPartPosition::Right,
                nodes: vec![(Vec3::new(0.0, 0.0, 0.0), 2.0),
                        (Vec3::new(4.0, 4.0, 0.0), 2.0),]
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Ellipse::new(30.0, 10.0))),
                material: materials.add(COLOR_GREEN),
                ..default()
            },
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
    mut q_squeleton: Query<(&KinematicChain, &mut Children)>,
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
    mut q_squeleton: Query<(&KinematicChain, &mut Children)>,
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

fn draw_leg(
    mut gizmos: Gizmos,
    mut q_squeleton: Query<(&KinematicChain, &mut Children)>,
    mut q_legs: Query<(&mut Leg, &mut Transform)>,
) {
    for (squeleton, mut children) in q_squeleton.iter_mut() {
        for &child in children.iter() {
            if let Ok((mut leg, mut transform)) = q_legs.get_mut(child) {
                let anchor_node = squeleton.nodes[leg.anchor];
                let anchor_head = squeleton.nodes[leg.anchor - 1];

                *transform = get_attachment_position(anchor_node, anchor_head, &leg.position);

                gizmos.circle_2d(transform.translation.truncate(), 5.0, COLOR_GREEN);

                let foot_direction = get_perpendicular_body_ray(anchor_node, anchor_head);

                // calculate the position of the foot
                for node in leg.nodes.iter_mut() {
                    node.0.x = transform.translation.x;
                    node.0.y = transform.translation.y;

                    let middle_position = node.0.truncate() + foot_direction.direction * 30.0;
                    let top_position = middle_position + foot_direction.direction.perp() * 20.0;
                    let bottom_position = middle_position - foot_direction.direction.perp() * 20.0;
                    gizmos.circle_2d(middle_position, 5.0 , COLOR_GREEN);
                    gizmos.circle_2d(top_position, 5.0 , COLOR_RED);
                    gizmos.circle_2d(bottom_position, 5.0 , COLOR_RED);

                }

            }

        }
    }
}

fn get_attachment_position(node: (Vec3, f32), head: (Vec3, f32), part_type: &BodyPartPosition) -> Transform
{
    // let distance = head.0.distance(node.0);
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // let ray = get_body_direction(node, head);

    let left = ray.origin + ray.direction.perp() * node.1;
    let right = ray.origin + -ray.direction.perp() * node.1;

    let mut transform = Transform::default();
    match part_type {
        BodyPartPosition::Dorsal => {
            transform.translation = node.0;
            transform.translation.z = 1.0;
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
        }
        BodyPartPosition::Left => {
            transform.translation = left.extend(-1.0);
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::PI/2.0);
        }
        BodyPartPosition::Right => {
            transform.translation = right.extend(-1.0);
            let angle = ray.direction.to_angle();
            transform.rotation = Quat::from_rotation_z(angle + std::f32::consts::PI/2.0);
        }
    }

    return transform;
}

fn get_body_direction(node: (Vec3, f32), head: (Vec3, f32)) -> Ray2d {
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // Left side
    return Ray2d {
        origin: ray.origin + ray.direction.perp() * node.1,
        direction: Dir2::new_unchecked(ray.direction.perp())
    }
}

fn get_perpendicular_body_ray(node: (Vec3, f32), head: (Vec3, f32)) -> Ray2d {
    let ray = Ray2d {
        origin: head.0.truncate(),
        direction: Dir2::new_unchecked(
            (node.0 - head.0).truncate().normalize(),
        ),
    };

    // Left side
    return Ray2d {
        origin: ray.origin + ray.direction.perp() * node.1,
        direction: Dir2::new_unchecked(-ray.direction.perp())
    }
}