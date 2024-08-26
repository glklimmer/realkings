use avian3d::prelude::*;
use bevy::{
    pbr::{
        CascadeShadowConfigBuilder,
        // NotShadowCaster
    },
    prelude::*,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terrain_scene);
    }
}

#[derive(Component)]
struct Ground;

fn setup_terrain_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Configure a properly scaled cascade shadow map for this scene (defaults are too large, mesh units are in km)
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });

    // Terrain
    // commands.spawn(SceneBundle {
    //     scene: asset_server
    //         .load(GltfAssetLabel::Scene(0).from_asset("models/terrain/Mountains.gltf")),
    //     ..default()
    // });
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Sky
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(2.0, 1.0, 1.0)),
    //         material: materials.add(StandardMaterial {
    //             base_color: Srgba::hex("888888").unwrap().into(),
    //             unlit: true,
    //             cull_mode: None,
    //             ..default()
    //         }),
    //         transform: Transform::from_scale(Vec3::splat(20.0)),
    //         ..default()
    //     },
    //     NotShadowCaster,
    // ));
}
