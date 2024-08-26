use avian3d::prelude::*;
use bevy::prelude::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_units);
    }
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
