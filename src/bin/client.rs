use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

use realkings::{
    camera::CameraPlugin, player::PlayerPlugin, terrain::TerrainPlugin, units::UnitsPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, TerrainPlugin, PlayerPlugin, UnitsPlugin))
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins((
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ))
        .add_plugins(PhysicsPlugins::default())
        .run();
}
