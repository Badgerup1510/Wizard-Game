use bevy::{prelude::*, ecs::prelude::Commands};
use bevy_flycam::prelude::*;
use bevy_atmosphere::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use std::collections::HashMap;
use std::collections::HashSet;
//use pixelate_mesh::prelude::*;

mod day_night;
use crate::day_night::day_night_plugin;

mod world;
use crate::world::world_plugin;
 


#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins, 
                NoCameraPlayerPlugin, 
                AtmospherePlugin, 
                day_night_plugin,
                world_plugin,
                ))
        .insert_resource(MovementSettings {
            sensitivity: 0.00006,
            speed: 12.0,
        })
        //.add_plugins(PixelateMeshPlugin::<MainCamera>::default())
        .add_systems(Startup, (setup))
        //.add_systems(Update, (render_chunks))
        .run();
}




#[derive(Component)]
pub struct Chunk{
    position: Vec3,
}

#[derive(Component)]
struct Cube;

/*
#[derive(Component)]
pub struct Player;
*/

use world::Player;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    
    // spawns the main camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.5),
            ..default()
        },
        AtmosphereCamera::default(),
        FlyCam,
        Player,
        MainCamera,

    ));

    // spawns a directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    

    let perlin = Perlin::new(1);
    let scale = 0.2;
    // debug ==> spawns a random chunk 
    let mut voxel: [[[bool; 16]; 16]; 16] = [[[false; 16]; 16]; 16];
    for i in 0..16 {
        for j in 0..16 {
            for k in 0..16 { 
                let val = perlin.get([i as f64 * scale, j as f64 * scale, k as f64 * scale]);
                voxel[i][j][k] = val >= 0.0; 
            }
        }
    }
    //spawn_cubes(voxel, [0.0,0.0,0.0], commands, meshes, materials);
} 



fn spawn_cubes(voxels: [[[bool; 16]; 16]; 16], position: [f32; 3], mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let chunk_entity = commands.spawn((
        Chunk {
            position: Vec3::new(position[0], position[1], position[2]),
        },
        SpatialBundle {..default()}
    )).id();
 
    for i in 0..15 {
        for j in 0..15 {
            for k in 0..15 {
                if voxels[i][j][k] == true {
                    let cube = commands.spawn(( 
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                                transform: Transform::from_xyz(
                                    i as f32 + position[0] - 7.0,
                                    j as f32 + position[1] - 7.0,
                                    k as f32 + position[2] - 7.0,
                                    ),
                                material: materials.add(Color::WHITE),
                                ..default()
                            }, Cube, )).id();
                    commands.entity(chunk_entity).push_children(&[cube]);
                };
            }
        }
    }
}

/*
fn render_chunks(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<StandardMaterial>>,        
        player_query: Query<&Transform, With<Player>>,
        chunk_query: Query<(Entity, &Chunk, &Transform)>,
    ) {
        let player_transform = player_query.single();
        let x: isize = (player_transform.translation[0]) as isize/ 16;
        let y: isize = (player_transform.translation[1]) as isize/ 16;
        let z: isize = (player_transform.translation[2]) as isize/ 16;

        let mut current_chunks = [[[false; 65]; 65]; 65];
        let mut chunks_to_spawn = [[[false; 65]; 65]; 65]; 
        let mut invalid_chunks = [[[false; 65]; 65]; 65];
        let offset = 32;

        let mut chunk_count = 0;

        // Collect existing chunks into a HashSet for quick lookup
        let mut existing_chunks = HashSet::new();
        for ( _, _chunk, transform) in chunk_query.iter() {
            let a: isize = (transform.translation.x / 16.0).floor() as isize;
            let b: isize = (transform.translation.y / 16.0).floor() as isize;
            let c: isize = (transform.translation.z / 16.0).floor() as isize;

            let chunk_pos = (a,b,c);
            existing_chunks.insert(chunk_pos);
            current_chunks[(a + offset) as usize][(b + offset) as usize][(c + offset) as usize ] = true; 
            chunk_count += 1;
        }
        println!("{}", chunk_count);

        for i in (x - 3)..=(x + 3) {
            for j in (y - 3)..=(y + 3) {
                for k in (z - 3)..=(z + 3) {
                    // spawn a chunk if it does not exist
                    if !current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] {
                        chunks_to_spawn[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] = true;
                        /*
                        let chunk_entity = commands.spawn((
                        Chunk {
                            position: Vec3::new(i as f32 * 16.0, j as f32 * 16.0, k as f32 * 16.0),
                        },
                        SpatialBundle{..default()},
                            
                        )).id();
                        
                        let cube = commands.spawn((
                                PbrBundle {
                                    mesh: meshes.add(Cuboid::new(14.0, 1.0, 14.0)),
                                    transform: Transform::from_xyz(
                                        i as f32 * 16.0,
                                        j as f32 * 16.0, 
                                        k as f32 * 16.0,
                                        ),
                                    material: materials.add(Color::WHITE),
                                    ..default()
                                },
                                Cube,
                        )).id();
                        commands.entity(chunk_entity).push_children(&[cube]);
                        */
                    }

                    
                    //valid_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] = true;
                    //println!("{}", valid_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize])
                }
            }
        }
        
        // loop through all chunk positions
        for i in 0..65 {
            for j in 0..65 {
                for k in 0..65 {
                    //
                }
            }
        }

        /*
        // iterate through the nearby world and spawn non-exising chunks 
        for i in (x - 3)..=(x + 3) {
            for j in (y - 3)..=(y + 3) {
                for k in (z - 3)..=(z + 3) {
                    
                    if !current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize]  {
                        let chunk_entity = commands.spawn((
                            Chunk {
                                position: Vec3::new(i as f32 * 16.0, j as f32 * 16.0, k as f32 * 16.0),
                            },
                            SpatialBundle{..default()},
                            
                        )).id();
                    
                    let cube = commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(14.0, 1.0, 14.0)),
                                transform: Transform::from_xyz(
                                    i as f32 * 16.0,
                                    j as f32 * 16.0, 
                                    k as f32 * 16.0,
                                    ),
                                material: materials.add(Color::WHITE),
                                ..default()
                            },
                            Cube,
                    )).id();
                    commands.entity(chunk_entity).push_children(&[cube]);
                    }
                }

            }
        }

        for (chunk_entity, chunk, transform) in chunk_query.iter() {
            let chunk_pos = (
                (transform.translation.x / 16.0).floor() as isize,
                (transform.translation.y / 16.0).floor() as isize,
                (transform.translation.z / 16.0).floor() as isize,
            );

            if (chunk_pos.0 < x - 3 || chunk_pos.0 > x + 3) ||
               (chunk_pos.1 < y - 3 || chunk_pos.1 > y + 3) ||
               (chunk_pos.2 < z - 3 || chunk_pos.2 > z + 3) {
                commands.entity(chunk_entity).despawn_recursive();
            }
        }
        */
}
*/
