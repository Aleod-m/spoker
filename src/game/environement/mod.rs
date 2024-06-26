use bevy::render::{
    mesh::PlaneMeshBuilder,
    texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
};

use super::player::components::SpawnPlayerCmd;
use crate::{
    common::physics::{col_layers, prelude::*}, prelude::*, AppState
};

pub struct EnvironementPlugin;

impl Plugin for EnvironementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Lighting.
    ambient_light.brightness = 400.0;
    ambient_light.color = Color::WHITE;
    let mut light_dir = Transform::default();
    light_dir.look_to(Vec3::NEG_Y + Vec3::X, Vec3::Y);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: light_dir,
        ..default()
    });

    // Ground.
    let ground_mesh = meshes.add(
        PlaneMeshBuilder {
            half_size: Vec2::splat(1000.0),
            plane: Plane3d::new(Vec3::Y),
        }
        .build()
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [-500.0, -500.0],
                [500.0, -500.0],
                [500.0, 500.0],
                [-500.0, 500.0],
            ],
        ),
    );
    let ground_texture = asset_server.load_with_settings(
        "textures/Black/tex_7.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::default()
            });
        },
    );

    let ground_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(ground_texture),
        reflectance: 0.0,
        perceptual_roughness: 1.0,
        ..default()
    });
    commands
        .spawn((
            PbrBundle {
                mesh: ground_mesh,
                material: ground_mat,
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            RigidBody::Fixed,
            CollisionGroups {
                memberships: col_layers::ENVIRONEMENT,
                filters: col_layers::PLAYERS,
            },
        ))
        .with_children(|children| {
            children.spawn((
                Collider::halfspace(Vec3::Y).unwrap(),
                TransformBundle::from(Transform::from_translation(Vec3::ZERO)),
            ));
        });

    // Little green cube.
    let cube_mesh = meshes.add(Cuboid {
        half_size: Vec3::splat(0.5),
    });
    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load("textures/Green/tex_7.png")),
        ..default()
    });

    commands.spawn((PbrBundle {
        mesh: cube_mesh,
        material: cube_mat,
        transform: Transform::from_translation(Vec3::Y * 0.5)
            .with_rotation(Quat::from_axis_angle(Vec3::Y, PI / 6.0)),
        ..default()
    },));

    // Wall
    let wall_mesh = meshes.add(Cuboid {
        half_size: Vec3::new(1.0, 10.0, 10.0),
    });
    let wall_mat = materials.add(StandardMaterial {
        base_color: Color::YELLOW,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: wall_mesh,
            material: wall_mat,
            transform: Transform::from_translation(Vec3::Y * 5.0 + Vec3::X * 10.0),
            ..default()
        },
        RigidBody::Fixed,
        CollisionGroups {
            memberships: col_layers::ENVIRONEMENT,
            filters: col_layers::PLAYERS,
        },
        Collider::cuboid(1.0, 10.0, 10.0),
    ));

    commands.add(SpawnPlayerCmd {
        transform: Transform::default(),
        cam_offset: Vec3::Y,
    });
}
