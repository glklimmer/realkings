use avian3d::prelude::*;
use bevy::{
    pbr::{
        CascadeShadowConfigBuilder,
        // NotShadowCaster
    },
    prelude::*,
};
use bevy_third_person_camera::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins((
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ))
        .add_plugins(PhysicsPlugins::default())
        .add_systems(
            Startup,
            (setup_camera, setup_terrain_scene, setup_player, setup_units),
        )
        .add_systems(
            Update,
            (
                apply_rotation,
                apply_controls.in_set(TnuaUserControlsSystemSet),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            //transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            aim_enabled: true,
            aim_button: MouseButton::Right,
            aim_speed: 3.0,
            aim_zoom: 0.7,
            cursor_lock_key: KeyCode::KeyQ,
            cursor_lock_toggle_enabled: true,
            cursor_lock_active: true,
            sensitivity: Vec2::new(2.0, 2.0),
            mouse_orbit_button_enabled: false,
            offset_enabled: false,
            offset: Offset::new(0.5, 0.4),
            offset_toggle_enabled: false,
            offset_toggle_speed: 5.0,
            offset_toggle_key: KeyCode::KeyE,
            zoom: Zoom::new(10., 10.),
            zoom_sensitivity: 1.0,
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

#[derive(Component)]
struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Capsule
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::hsl(237., 0.75, 0.5)),
            transform: Transform::from_xyz(5.0, 1., 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
        Player,
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        TnuaControllerBundle::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
    ));
}

fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Capsule
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::default()),
            material: materials.add(Color::hsl(237., 0.75, 0.5)),
            transform: Transform::from_xyz(-5.0, 1., 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
    ));
}

fn apply_rotation(
    mut player_query: Query<&mut Transform, With<TnuaController>>,
    cam_query: Query<&Transform, (With<Camera3d>, Without<TnuaController>)>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let Ok(cam) = cam_query.get_single() else {
        return;
    };

    let mut direction = *cam.forward();
    direction.y = 0.0;
    player_transform.look_to(direction, Vec3::Y);
}

fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TnuaController, &Transform)>,
) {
    let Ok((mut controller, transform)) = query.get_single_mut() else {
        return;
    };

    let speed = 10.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction += *transform.back();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction += *transform.left();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction.normalize_or_zero() * speed,
        float_height: 0.1,
        ..Default::default()
    });

    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            ..Default::default()
        });
    }
}
