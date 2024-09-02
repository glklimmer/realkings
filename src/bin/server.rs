use avian3d::prelude::*;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_systems(Update, hello_world_system)
        .add_plugins(PhysicsPlugins::default())
        .run();
}

fn hello_world_system() {
    println!("hello world");
}

// use std::time::Duration;
//
// use avian3d::prelude::*;
// use bevy::{app::ScheduleRunnerPlugin, prelude::*};
//
// fn main() {
//     App::new()
//         .add_plugins(
//             MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
//                 1.0 / 60.0,
//             ))),
//         )
//         .add_plugins(PhysicsPlugins::default())
//         .add_systems(Update, counter)
//         .run();
// }
//
// #[derive(Default)]
// struct CounterState {
//     count: u32,
// }
//
// fn counter(mut state: Local<CounterState>) {
//     if state.count % 60 == 0 {
//         println!("{}", state.count);
//     }
//     state.count += 1;
// }
