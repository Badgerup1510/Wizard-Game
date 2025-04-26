use bevy::prelude::*;
use avian3d::prelude::*;
use std::time::Instant;

use crate::chunk::{generate_chunk, generate_chunk_mesh, GameChunk, CHUNK_SIZE, BlockType, load_chunk_data};
use crate::chunk::{generate_chunk_new, generate_chunk_mesh_new};

pub fn world_plugin(app: &mut App) {
    app.add_systems(Update, render_chunks_new);
    //app.add_systems(Startup, test_spawn_chunk);
}

const RENDER_DISTANCE: usize = 10;

#[derive(Component)]
pub struct Chunk{
    pub position: Position,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Player;





fn render_chunks_new(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    mut params: ParamSet<(
        Query<(Entity, &GameChunk)>,       // p0: read-only
        Query<&mut GameChunk>,             // p1: mutable
    )>,    
    asset_server: Res<AssetServer>,
) {
    const CHUNKS_PER_TICK: usize = 10;

    //let chunk_query = params.p0();           // read-only
    //let mut game_chunk_query = params.p1();  // mutable
    let start = Instant::now();


    let Ok(player_transform) = player_query.get_single() else {
        return
    };
    //Define texture atlas
    let texture_atlas = asset_server.load("textures/Sprite-Mini-Spritesheat.png");

    let x_start = (player_transform.translation.x as i32 / CHUNK_SIZE as i32) - ( RENDER_DISTANCE as i32);
    let x_end = (player_transform.translation.x as i32 / CHUNK_SIZE as i32) + (RENDER_DISTANCE as i32);
    let y_start = (player_transform.translation.y as i32 / CHUNK_SIZE as i32) - (RENDER_DISTANCE as i32);
    let y_end = (player_transform.translation.y as i32 / CHUNK_SIZE as i32) + (RENDER_DISTANCE as i32);
    let z_start = (player_transform.translation.z as i32 / CHUNK_SIZE as i32) - (RENDER_DISTANCE as i32);
    let z_end = (player_transform.translation.z as i32 / CHUNK_SIZE as i32) + (RENDER_DISTANCE as i32);

    //println!("{:?}, {:?}, {:?}", x_start..x_end, y_start..y_end, z_start..z_end);
    



    //let y_range = (player_transform.translation.y - (CHUNK_SIZE as f32 * RENDER_DISTANCE as f32))..(player_transform.translation.y + (CHUNK_SIZE as f32 * RENDER_DISTANCE as f32));
    //let z_range = (player_transform.translation.z - (CHUNK_SIZE as f32 * RENDER_DISTANCE as f32))..(player_transform.translation.z + (CHUNK_SIZE as f32 * RENDER_DISTANCE as f32));

    let mut chunks_should_exist: Vec<(Position, f32)> = Vec::new();
    let mut chunks_that_exist: Vec<Position> = Vec::new();
    let mut chunks_to_spawn: Vec<Position> = Vec::new();

    let mut debug_num_of_chunks: f32 = 0.0;

    for (entity, chunk) in params.p0().iter() {

        debug_num_of_chunks += 1.0;

        let x_range = x_start..x_end;
        let y_range = y_start..y_end;
        let z_range = z_start..z_end;
        if x_range.contains(&(chunk.position.x)) && y_range.contains(&(chunk.position.y)) && z_range.contains(&(chunk.position.z)) {
            chunks_that_exist.push(chunk.position)

        }
        else {
            commands.entity(entity).despawn_recursive();
            //println!("Despawned Chunk");

        }
    }

    //println!("{}", debug_num_of_chunks);


    let x_range = x_start..x_end;

    for i in x_range {
        let y_range = y_start..y_end;

        for j in y_range {
            let z_range = z_start..z_end;

            for k in z_range {
                let position = Position{x: i, y: j, z: k};
                if !chunks_that_exist.contains(&position) {
                    let distance = Vec3::new(16.0 * i as f32 - player_transform.translation.x, 16.0 * j as f32 - player_transform.translation.y, 16.0 * k as f32 - player_transform.translation.z).length();
                    chunks_should_exist.push((position, distance));

                }

            }
        }
    }
    chunks_should_exist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    chunks_should_exist.truncate(CHUNKS_PER_TICK);

    //println!("after sorting: {:?}", start.elapsed());
    for chunk in chunks_should_exist.iter() {
        /*
        //println!("Spawning Chunk");
        //println!("{:?}", Transform::from_translation(Vec3::new(
                                    chunk.0.x as f32 * 16.0,
                                    chunk.0.y as f32 * 16.0,
                                    chunk.0.z as f32 * 16.0,
                                )));
        */
        let chunk_entity = commands
                        .spawn((
                            GameChunk {
                                position: chunk.0,
                                data: [[[BlockType::Air; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
                                data_loaded: false,
                                mesh_loaded: false
                            },
                            Transform::from_translation(Vec3::new(
                                    chunk.0.x as f32 * CHUNK_SIZE as f32,
                                    chunk.0.y as f32 * CHUNK_SIZE as f32,
                                    chunk.0.z as f32 * CHUNK_SIZE as f32,
                                )),
                            Visibility::default(),
                            ))
                    .id();
            
                    let material_handle = materials.add(StandardMaterial {
                        base_color_texture: Some(texture_atlas.clone()),
                        ..default()
                    });

                    //println!("Before chunk_data: {:?}", start.elapsed());

                    //let chunk_data = generate_chunk_new(chunk.0, &mut params.p1());
                    let chunk_data = generate_chunk_new(chunk.0, &mut params.p1());

                    //println!("Before mesh: {:?}", start.elapsed());

                    let chunk_mesh = generate_chunk_mesh_new(chunk_data);
                    //println!("After mesh: {:?}", start.elapsed());

                    //println!("{:?}", chunk_mesh.1);

                    if chunk_mesh.1 {

                        let chunk = commands.spawn(( 
                            Mesh3d(meshes.add(chunk_mesh.0.clone())),
                            MeshMaterial3d(material_handle.clone()),
                            Transform::default(),
                            GlobalTransform::default(),
                            Visibility::default(),
                            RigidBody::Static,
                            Collider::trimesh_from_mesh(&chunk_mesh.0.clone()).unwrap(),
                            //CollisionMargin(0.05),
                        )).id();
                        commands.entity(chunk_entity).add_children(&[chunk]);

                    }
    }
}

/*
fn test_spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    mut params: ParamSet<(
        Query<(Entity, &GameChunk)>,       // p0: read-only
        Query<&mut GameChunk>,             // p1: mutable
    )>, 
    asset_server: Res<AssetServer>,
) {
    let chunk = (Position {
        x: -2,
        y: -4,
        z: -2,
    }, true);
    let texture_atlas = asset_server.load("textures/Sprite-Mini-Spritesheat.png");

let chunk_entity = commands
                        .spawn((
                            GameChunk {
                                position: chunk.0,
                                data: [[[BlockType::Air; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
                                data_loaded: false,
                                mesh_loaded: false
                            },
                            Transform::from_translation(Vec3::ZERO),
                            /*
                            Transform::from_translation(Vec3::new(
                                    chunk.0.x as f32 * 16.0,
                                    chunk.0.y as f32 * 16.0,
                                    chunk.0.z as f32 * 16.0,
                                )),
                            */
                            Visibility::default(),
                            ))
                    .id();
            
                    let material_handle = materials.add(StandardMaterial {
                        base_color_texture: Some(texture_atlas.clone()),
                        ..default()
                    });
                    //load_chunk_new();
                    //let chunk_data = load_chunk_new(chunk.0, &mut params.p1());
                    let test_data = [[[BlockType::Grass; 48]; 48]; 48];

                    let chunk_mesh = generate_chunk_mesh_new(chunk_data);
                    //println!("{:?}", chunk_mesh.1);

                    //println!("{:?}", chunk_mesh.1);

                    if chunk_mesh.1 {
                        println!("Mesh");
                        let chunk = commands.spawn(( 
                            Mesh3d(meshes.add(chunk_mesh.0.clone())),
                            MeshMaterial3d(material_handle.clone()),
                            Transform::default(),
                            GlobalTransform::default(),
                            Visibility::default(),
                            RigidBody::Static,
                            Collider::trimesh_from_mesh(&chunk_mesh.0.clone()).unwrap(),
                        )).id();
                        commands.entity(chunk_entity).add_children(&[chunk]);

                    }

}
*/
fn render_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,        
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(Entity, &Chunk)>,
    mut game_chunk_query: Query<&mut GameChunk>,
    asset_server: Res<AssetServer>,
    ) {

    // Define texture atlas
    let texture_atlas = asset_server.load("textures/Sprite-Mini-Spritesheat.png");

    // Define Immutable
    //const RENDER_DISTANCE: u32 = 5;                         // distance of 4 around the player 
    //const RENDER_DISTANCE_HALF: u32 = RENDER_DISTANCE / 2;    // half render distance
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

    let mut debug_chunks_count = 0;

    // Loop through existing chunks
    for (entity, chunk) in chunk_query.iter() {
        debug_chunks_count +=1;
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

    for chunk in game_chunk_query.iter() {

    }

    println!("{}", debug_chunks_count);

    let cube_mesh_handle = meshes.add(Cuboid::new(15.9, 1.0, 15.9));
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_atlas.clone()),
        ..default()
    });

    let chunks_to_load: u32 = 10;
    let mut chunks_loading: u32 = 0;

    // spawn chunk if necassary
    for i in 0..ARRAY_LENGTH {
        for j in 0..ARRAY_LENGTH {
            for k in 0..ARRAY_LENGTH {
                //println!("{}, {}", chunks_should_exist[i as usize][j as usize][k as usize], chunks_does_exist[i as usize][j as usize][k as usize]);
                if chunks_should_exist[i as usize][j as usize][k as usize] && !chunks_does_exist[i as usize][j as usize][k as usize] {
                    //println!("{}, {}, {}", i as f32 - 32.0, j as f32 - 32.0, k as f32 - 32.0);
                    if chunks_loading <= chunks_to_load {
                        let chunk_entity = commands
                            .spawn((
                                GameChunk {
                                    position: Position {
                                        x: (i as isize - 32) as i32,
                                        y: (j as isize - 32) as i32,
                                        z: (k as isize - 32) as i32,
                                    },
                                    data: [[[BlockType::Air; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
                                    data_loaded: false,
                                    mesh_loaded: false
                                },
                                Transform::from_translation(Vec3::new(
                                        (i as f32 - 32.0) * 16.0,
                                        (j as f32 - 32.0) * 16.0,
                                        (k as f32 - 32.0) * 16.0,
                                    )),
                                Visibility::default(),
                                ))
                        .id();

                        let chunk_data = generate_chunk_new(Position{x: i as i32 - 32, y: j as i32 - 32, z: k as i32 - 32}, &mut game_chunk_query);

                        let chunk_mesh = generate_chunk_mesh_new(chunk_data);

                        if chunk_mesh.1 {
                            let chunk = commands.spawn(( 
                                Mesh3d(meshes.add(chunk_mesh.0.clone())),
                                MeshMaterial3d(material_handle.clone()),
                                Transform::default(),
                                GlobalTransform::default(),
                                Visibility::default(),
                                RigidBody::Static,
                                Collider::trimesh_from_mesh(&chunk_mesh.0.clone()).unwrap(),
                            )).id();
                            commands.entity(chunk_entity).add_children(&[chunk]);

                        }
                        chunks_loading += 1;
                    }
                    else {
                        break
                    }
                }
            }
        }
    }
}

