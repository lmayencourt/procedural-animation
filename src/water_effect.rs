/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
    render::view::RenderLayers,
    render::camera::ScalingMode,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

/// Custom 2d material for water shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://shaders/water.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaterFloorMaterial {}

impl Material2d for WaterFloorMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://shaders/water_floor.wgsl".into()
    }
}

/// Plugin for easy integration in application
pub struct WaterEffectPlugin;

impl Plugin for WaterEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<WaterMaterial>::default());
        app.add_plugins(Material2dPlugin::<WaterFloorMaterial>::default());
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct TextureCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut water_material: ResMut<Assets<WaterMaterial>>,
    mut water_floor_material: ResMut<Assets<WaterFloorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    let windows = windows.single();

    let size = Extent3d {
        width: windows.width() as u32,
        height: windows.height() as u32,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
 
    // Camera that capture the screen to the image
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                target: image_handle.clone().into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // water floor
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: Mesh2dHandle(meshes.add(Rectangle::new(windows.width(), windows.height()))),
    //     transform: Transform::from_xyz(0.0, 0.0, -1.0),
    //     material: water_floor_material.add(WaterFloorMaterial {}),
    //     ..default()
    // });

    // Second camera that display the texture shader
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(780.0);
    commands.spawn((
        camera,
        TextureCamera,
        RenderLayers::layer(1),
    ));

    // Create a material mesh which is on top of all the other element in the scene
    // and with the full size of the screen. The transparency of the effect should allow
    // to see below.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(windows.width(), windows.height()))),
            material: water_material.add(WaterMaterial {
                color_texture: Some(image_handle),
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RenderLayers::layer(1),
    ));
}
