use bevy::prelude::*;
use crate::HashSet;
use crate::Cube;

pub fn world_plugin(app: &mut App) {
    app.add_systems(Update, render_chunks);
}
#[derive(Component)]
pub struct Chunk{
    position: Vec3,
}

#[derive(Component)]
pub struct Player;



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
            let a: isize = (transform.translation.x) as isize;
            let b: isize = (transform.translation.y) as isize;
            let c: isize = (transform.translation.z) as isize;

            let chunk_pos = (a,b,c);
            existing_chunks.insert(chunk_pos);
            current_chunks[(a + offset) as usize][(b + offset) as usize][(c + offset) as usize ] = true; 
            println!("{}, {}, {}", a, b, c);
            chunk_count += 1;
        }
        println!("{}", chunk_count);

        for i in (x - 3)..=(x + 3) {
            for j in (y - 3)..=(y + 3) {
                for k in (z - 3)..=(z + 3) {
                    if i.abs() <= 32 && j.abs() <= 32 && k.abs() <= 32{
                        // spawn a chunk if it does not exist
                        if !current_chunks[(i + offset) as usize][(j + offset) as usize][(k + offset) as usize] {
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
                            
                        }

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
}
