use bevy::prelude::*;
use pixelate_mesh::prelude::*;

use crate::MainCamera;

use crate::chunk::{generate_chunk, generate_chunk_mesh, generate_chunk_data};

pub fn world_plugin(app: &mut App) {
    app.add_systems(Update, render_chunks);
    app.add_plugins(PixelateMeshPlugin::<MainCamera>::default());
}
#[derive(Component)]
pub struct Chunk{
    pub position: Position,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Player;


fn render_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,        
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(Entity, &Chunk)>,
    ) {

    // Define Immutable
    const RENDER_DISTANCE: u32 = 4;                         // distance of 4 around the player 
    const RENDER_DISTANCE_HALF: u32 = RENDER_DISTANCE /2;    // half render distance
    //const RENDER_RANGE: u32 = (2 * RENDER_DISTANCE) + 1;    // 9 x 9 grid
    const ARRAY_LENGTH: u32 = 65;                           // 33 x 33 chunk grid
    const ARRAY_HALF: i32 = 32; 

    // Define Mutable
    let mut chunks_should_exist = [[[false; ARRAY_LENGTH as usize]; ARRAY_LENGTH as usize]; ARRAY_LENGTH as usize];
    let mut chunks_does_exist = [[[false; ARRAY_LENGTH as usize]; ARRAY_LENGTH as usize]; ARRAY_LENGTH as usize];

    // get player position in chunk
    let player_transform = player_query.single();

    let player_x: i32 = ((player_transform.translation[0] + 8.0) / 16.0).floor() as i32;
    let player_y: i32 = ((player_transform.translation[1] + 8.0) / 16.0).floor() as i32;
    let player_z: i32 = ((player_transform.translation[2] + 8.0) / 16.0).floor() as i32;        

    // loop through the positions around the player
    for i in (player_x - RENDER_DISTANCE as i32)..=(player_x + RENDER_DISTANCE as i32) {
        for j in (player_y - RENDER_DISTANCE as i32)..=(player_y + RENDER_DISTANCE as i32) {
            for k in (player_z - RENDER_DISTANCE as i32)..=(player_z + RENDER_DISTANCE as i32) {
                // The chunk should exist
                chunks_should_exist[(ARRAY_HALF + i) as usize][(ARRAY_HALF + j) as usize][(ARRAY_HALF + k) as usize] = true;
                //println!("Chunk: {i}, {j}, {k} should exist");
            }
        }

    }

    // Loop through existing chunks
    for (entity, chunk) in chunk_query.iter() {
        // chunk 0-65
        let chunk_x: i32 = chunk.position.x;
        let chunk_y: i32 = chunk.position.y;
        let chunk_z: i32 = chunk.position.z;
         

        if chunk_x <= 65 && chunk_y <= 65 && chunk_z <= 65 {
            // The chunk does exist
            chunks_does_exist[(ARRAY_HALF + chunk_x) as usize][(ARRAY_HALF + chunk_y) as usize][(ARRAY_HALF + chunk_z) as usize] = true;

            //Despawns if it shouldnt exist
            if !chunks_should_exist[(ARRAY_HALF + chunk_x) as usize][(ARRAY_HALF + chunk_y) as usize][(ARRAY_HALF + chunk_z)as usize] {
                commands.entity(entity).despawn_recursive();
                //println!("Despawning: {chunk_x}, {chunk_y}, {chunk_z}");
            } 
        }
    }
    let cube_mesh_handle = meshes.add(Cuboid::new(15.9, 1.0, 15.9));
    let material_handle = materials.add(Color::WHITE);

    // spawn chunk if necassary
    for i in 0..ARRAY_LENGTH {
        for j in 0..ARRAY_LENGTH {
            for k in 0..ARRAY_LENGTH {
                //println!("{}, {}", chunks_should_exist[i as usize][j as usize][k as usize], chunks_does_exist[i as usize][j as usize][k as usize]);
                if chunks_should_exist[i as usize][j as usize][k as usize] && !chunks_does_exist[i as usize][j as usize][k as usize] {
                    //println!("{}, {}, {}", i as f32 - 32.0, j as f32 - 32.0, k as f32 - 32.0);
                    let chunk_entity = commands
                        .spawn((
                            Chunk {
                                position: Position {
                                    x: (i as isize - 32) as i32,
                                    y: (j as isize - 32) as i32,
                                    z: (k as isize - 32) as i32,
                                }
                            },
                            SpatialBundle {
                                transform: Transform::from_translation(Vec3::new(
                                    (i as f32 - 32.0) * 1.0,
                                    (j as f32 - 32.0) * 1.0,
                                    (k as f32 - 32.0) * 1.0,
                                )),
                                ..Default::default()
                            }
                        ))
                    .id();


                    //println!("{}, {}, {}", (i as isize - 32) as i32, (j as isize -32) as i32, (k as isize -32) as i32);
                    //println!("{}, {}, {}", i as f32 - 32.0, j as f32 - 32.0, k as f32 - 32.0);


                    let cube = commands.spawn(( 
                        PbrBundle {
                            mesh: meshes.add(generate_chunk_mesh(generate_chunk(Position{x: i as i32 - 32, y: j as i32 - 32, z: k as i32 - 32}))), 
                            //mesh: meshes.add(generate_chunk_mesh(temp)),
                            transform: Transform::from_xyz(
                                (i as f32 - 32.0) * 15.0 - (RENDER_DISTANCE_HALF as f32 * 15.0), 
                                (j as f32 - 32.0) * 15.0 - (RENDER_DISTANCE_HALF as f32 * 15.0),
                                (k as f32 - 32.0) * 15.0 - (RENDER_DISTANCE_HALF as f32 * 15.0),
                                ),
                            material: material_handle.clone(),
                            ..default()
                        }, )).id();
                    commands.entity(chunk_entity).push_children(&[cube]);
                    


                }
            }
        }
    }
}


/*
                        let chunk_entity = commands.spawn(Chunk {
                            position: Position{
                                x: (i as isize - offset) as i32,
                                y: (j as isize - offset) as i32,
                                z: (k as isize - offset) as i32,
                            }
                        }).id();   

                        let cube = commands.spawn(( 
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                                transform: Transform::from_xyz(
                                    i as f32 * 16.0,
                                    j as f32 * 16.0,
                                    k as f32 * 16.0,
                                    ),
                                material: materials.add(Color::WHITE),
                                ..default()
                            },  )).id();
                        commands.entity(chunk_entity).push_children(&[cube]);
*/
