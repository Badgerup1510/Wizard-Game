use bevy::{prelude::*, 
    ecs::prelude::Commands, 
};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use avian3d::prelude::*;

mod day_night;
//use crate::day_night::day_night_plugin;

mod world;
use crate::world::world_plugin;

mod chunk;
use crate::chunk::chunk_plugin;

mod player_character;
use crate::player_character::{spawn_view_model, PlayerCharacterPlugin};

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins, 
                PhysicsPlugins::default(),
                world_plugin,
                chunk_plugin,
                PlayerCharacterPlugin,
                ))
        //.insert_resource(Gravity(Vec3::NEG_Y * 0.0))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    
    // spawns the main camera
    spawn_view_model(commands, meshes, materials);

    /*
    commands.spawn((
        Camera3d::default(),
        Player,
        MainCamera,
        PlayerCharacterComponent,
        RigidBody::Dynamic,
        Collider::capsule(1.0, 3.0),
        LockedAxes::ROTATION_LOCKED,
    ));
    */

    // spawn debug ball for collision testing
    /*
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        RigidBody::Dynamic,
        Collider::sphere(0.9),

    ));
    */
} 

