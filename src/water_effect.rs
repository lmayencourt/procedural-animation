/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

/// Custom 2d material for water shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {}

impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://shaders/water_foam.wgsl".into()
    }
}

/// Plugin for easy integration in application
pub struct WaterEffectPlugin;

impl Plugin for WaterEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<WaterMaterial>::default());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_material: ResMut<Assets<WaterMaterial>>,
    windows: Query<&Window>,
) {
    let windows = windows.single();

    // Create a material mesh which is on top of all the other element in the scene
    // and with the full size of the screen. The transparency of the effect should allow
    // to see below.
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(windows.width(), windows.height()))),
        material: water_material.add(WaterMaterial {}),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..default()
    });
}
