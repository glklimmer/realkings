use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (
                apply_rotation,
                apply_controls.in_set(TnuaUserControlsSystemSet),
            ),
        );
    }
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
