use bevy::prelude::*;
use crate::HashSet;

pub fn world_plugin(app: &mut App) {
    app.add_systems(Update, render_chunks);
}
#[derive(Component)]
pub struct Chunk{
    pub position: Position
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
        let player_transform = player_query.single();
        let x: isize = (player_transform.translation[0]) as isize/ 16;
        let y: isize = (player_transform.translation[1]) as isize/ 16;
        let z: isize = (player_transform.translation[2]) as isize/ 16;


        let mut current_chunks = [[[false; 65]; 65]; 65];
        let mut chunks_to_spawn = [[[false; 65]; 65]; 65]; 
        let mut chunks_to_despawn = [[[false; 65]; 65]; 65];
        let offset = 32;


        //println!("{}, {}, {}", x + offset, y + offset ,z + offset);

        // Collect existing chunks into a HashSet for quick lookup
        for (_entity, chunk) in chunk_query.iter() {
            let a: isize = (chunk.position.x / 16) as isize;
            let b: isize = (chunk.position.y / 16) as isize;
            let c: isize = (chunk.position.z / 16) as isize;

            // if within bounds, assign it as a current chunk 
            if a.abs() <= 32 && b.abs() <= 32 && c.abs() <= 32 {
                current_chunks[(a + offset) as usize][(b + offset) as usize][(c + offset) as usize ] = true; 
                println!("Chunk: {a}, {b}, {c} is {}", current_chunks[(a) as usize][(b) as usize][(c) as usize ]);
            }
        }

        for i in (x - 3)..=(x + 3) {
            for j in (y - 3)..=(y + 3) {
                for k in (z - 3)..=(z + 3) {
                    if i.abs() <= 32 && j.abs() <= 32 && k.abs() <= 32{
                        println!("{i}, {j}, {k} => {}", current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize]);
                        //println!("{i}, {j}, {k} => {}", current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize]);
                        // spawn a chunk if it does not exist
                        if !current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] {
                            chunks_to_spawn[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] = true;
                            /*
                            chunks_to_spawn[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] = true;
                                let chunk_entity = commands.spawn((
                                Chunk {
                                    position: Vec3::new((i + offset) as f32 * 16.0, (j + offset) as f32 * 16.0, (k + offset) as f32 * 16.0),
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
                        else {
                            println!("A chunk exists")
                        }
                    }

                    
                    //valid_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] = true;
                    //println!("{}", valid_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize])
                }
            }
        }
        
        let mut num_chunks_to_spawn = 0;
        // loop through all chunk positions
        for i in 0..65 {
            for j in 0..65 {
                for k in 0..65 {
                    if chunks_to_spawn[i][j][k] {
                        num_chunks_to_spawn += 1;
                    }
                }
            }
        }
        //println!("{}", num_chunks_to_spawn);
}
