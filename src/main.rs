use bevy::{
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_terrain_scene, setup_units))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            //transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FogSettings {
            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                200.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));
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
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Srgba::hex("888888").unwrap().into(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Capsule
    commands.spawn(PbrBundle {
        mesh: meshes.add(Capsule3d::default()),
        material: materials.add(Color::hsl(237., 0.75, 0.5)),
        transform: Transform::from_xyz(5.0, 1., 0.0),
        ..default()
    });

    // Capsule
    commands.spawn(PbrBundle {
        mesh: meshes.add(Capsule3d::default()),
        material: materials.add(Color::hsl(237., 0.75, 0.5)),
        transform: Transform::from_xyz(-5.0, 1., 0.0),
        ..default()
    });
}
