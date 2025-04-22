use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use perlin2d::PerlinNoise2D;

// Import the Position struct from world.rs
use crate::world::Position;

pub fn chunk_plugin(app: &mut App) {
    //app.add_systems(Startup, chunk_startup);
}

pub enum BlockType {
    Grass,
    Stone,
}

pub fn generate_chunk_data(position: Position) -> [[[bool; 16]; 16]; 16] {
    // define empty chunk
    let mut chunk = [[[false; 16];16];16];

    // perlin noise parameters
    let octaves: i32 = 4; // detail
    let amplitude: f64 = 40.0; // the absolute output value 
    let frequency: f64 = 0.3; //cycles per unit length ???
    let persistence: f64 = 1.0; // determines how the amplitude diminishes
    let lacunarity: f64 = 2.0; // determines frequency increses of octaves
    let scale: (f64, f64) = (100.0, 100.0); // a distance to view the noise map ???
    let bias: f64 = 10.0; // Used to make output positive
    let seed: i32 = 100; // changes the output

    let perlin = PerlinNoise2D::new(octaves, amplitude, frequency, persistence, lacunarity, scale, bias, seed);


    for i in 0..=15 {
        for k in 0..=15 {
            let val = perlin.get_noise((position.x*16) as f64 + i as f64, (position.z*16) as f64 + k as f64).floor();
            let mut height_reached = false;

            for j in 0..= 15 {
                if (position.y*16 + j  <= val as i32) && !height_reached {
                    chunk[i as usize][j as usize][k as usize] = true;

                } else {
                    height_reached = true; 
                }
            }
        }
    }

    // return chunk
    chunk
}

pub fn generate_chunk(position: Position)
    -> [[[bool; 48]; 48]; 48]
{
    let m_x = position.x;
    let m_y = position.y;
    let m_z = position.z;

    // Define chunks
    let mut chunk: [[[bool; 48]; 48]; 48] = [[[false; 48]; 48]; 48];
    let chunk_111 = generate_chunk_data(position);
    let chunk_121 = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z});
    let chunk_101 = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z});
    let chunk_211 = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z});
    let chunk_011 = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z});
    let chunk_112 = generate_chunk_data(Position{x: m_x, y: m_y, z: m_z + 1});
    let chunk_110 = generate_chunk_data(Position{x: m_x, y: m_y, z: m_z - 1});
    // Diagonals
    // i/j
    let chunk_221 = generate_chunk_data(Position{x: m_x + 1, y: m_y + 1, z: m_z});// [i + 1][j + 1][k];
    let chunk_201 = generate_chunk_data(Position{x: m_x + 1, y: m_y - 1, z: m_z});//[i + 1][j - 1][k];
    let chunk_021 = generate_chunk_data(Position{x: m_x - 1, y: m_y + 1, z: m_z});//[i - 1][j + 1][k];
    let chunk_001 = generate_chunk_data(Position{x: m_x - 1, y: m_y - 1, z: m_z});//[i - 1][j - 1][k];
    // k/j
    let chunk_122 = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z + 1});//[i][j + 1][k + 1];
    let chunk_102 = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z + 1});//[i][j - 1][k + 1];
    let chunk_120 = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z - 1});//[i][j + 1][k - 1];
    let chunk_100 = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z - 1});//[i][j - 1][k - 1];
    // i/k
    let chunk_012 = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z + 1});
    let chunk_010 = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z - 1});
    let chunk_212 = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z + 1});
    let chunk_210 = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z - 1});

    // corners
    let chunk_222 = generate_chunk_data(Position{x: m_x + 1, y: m_y + 1, z: m_z + 1});
    let chunk_220 = generate_chunk_data(Position{x: m_x + 1, y: m_y + 1, z: m_z - 1});
    let chunk_202 = generate_chunk_data(Position{x: m_x + 1, y: m_y - 1, z: m_z + 1});
    let chunk_200 = generate_chunk_data(Position{x: m_x + 1, y: m_y - 1, z: m_z - 1});
    let chunk_022 = generate_chunk_data(Position{x: m_x - 1, y: m_y + 1, z: m_z + 1});
    let chunk_020 = generate_chunk_data(Position{x: m_x - 1, y: m_y + 1, z: m_z - 1});
    let chunk_002 = generate_chunk_data(Position{x: m_x - 1, y: m_y - 1, z: m_z + 1});
    let chunk_000 = generate_chunk_data(Position{x: m_x - 1, y: m_y - 1, z: m_z - 1});

    for a in 0..=15 {
        for b in 0..=15 {
            for c in 0..=15 {
                chunk[a][b][c] = chunk_000[a][b][c];
                chunk[a][b][c + 16] = chunk_001[a][b][c];
                chunk[a][b][c + 32] = chunk_002[a][b][c];
                chunk[a][b + 16][c] = chunk_010[a][b][c];
                chunk[a][b + 16][c + 16] = chunk_011[a][b][c];
                chunk[a][b + 16][c + 32] = chunk_012[a][b][c];
                chunk[a][b + 32][c] = chunk_020[a][b][c];
                chunk[a][b + 32][c + 16] = chunk_021[a][b][c];
                chunk[a][b + 32][c + 32] = chunk_022[a][b][c];
                chunk[a + 16][b][c] = chunk_100[a][b][c];
                chunk[a + 16][b][c + 16] = chunk_101[a][b][c];
                chunk[a + 16][b][c + 32] = chunk_102[a][b][c];
                chunk[a + 16][b + 16][c] = chunk_110[a][b][c];
                chunk[a + 16][b + 16][c + 16] = chunk_111[a][b][c];
                chunk[a + 16][b + 16][c + 32] = chunk_112[a][b][c];
                chunk[a + 16][b + 32][c] = chunk_120[a][b][c];
                chunk[a + 16][b + 32][c + 16] = chunk_121[a][b][c];
                chunk[a + 16][b + 32][c + 32] = chunk_122[a][b][c];
                chunk[a + 32][b][c] = chunk_200[a][b][c];
                chunk[a + 32][b][c + 16] = chunk_201[a][b][c];
                chunk[a + 32][b][c + 32] = chunk_202[a][b][c];
                chunk[a + 32][b + 16][c] = chunk_210[a][b][c];
                chunk[a + 32][b + 16][c + 16] = chunk_211[a][b][c];
                chunk[a + 32][b + 16][c + 32] = chunk_212[a][b][c];
                chunk[a + 32][b + 32][c] = chunk_220[a][b][c];
                chunk[a + 32][b + 32][c + 16] = chunk_221[a][b][c];
                chunk[a + 32][b + 32][c + 32] = chunk_222[a][b][c];
            }
        }
    }

    chunk
}


pub fn generate_chunk_mesh(chunk: [[[bool; 48]; 48]; 48]) -> (Mesh, bool) {
    // init empty triangle list mesh
    
    // define mesh attribute vectors
    let mut atr_pos: Vec<[f32; 3]> = vec![];
    let mut atr_uv: Vec<[f32; 2]> = vec![];
    let mut atr_norm: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];

    let mut indices_counter = 0;

    // loop through each position in chunk
    for mut i in 15..31 {
        for mut j in 15..31 {
            for mut k in 15..31 {
                // if current cube exists
                //print!("[{}, {}, {}]", i, j, k);
                

                if chunk[i][j][k] {
                    // define other cubes                // normals
                    let cube_right = !chunk[i + 1][j][k]; // [1, 0, 0]
                    let cube_left  = !chunk[i - 1][j][k]; // [-1, 0, 0]
                    let cube_above = !chunk[i][j + 1][k]; // [0, 1, 0]
                    let cube_below = !chunk[i][j - 1][k]; // [0, -1, 0]
                    let cube_front = !chunk[i][j][k + 1]; // [0, 0, 1]
                    let cube_hind  = !chunk[i][j][k - 1]; // [0, 0, -1]



                    let v0_0_0: [f32; 3] = [(i as f32), (j as f32), (k as f32)];
                    let v0_0_1: [f32; 3] = [(i as f32), (j as f32), (k as f32 + 1.0)];
                    let v0_1_0: [f32; 3] = [(i as f32), (j as f32 + 1.0), (k as f32)];
                    let v0_1_1: [f32; 3] = [(i as f32), (j as f32 + 1.0), (k as f32 + 1.0)];
                    let v1_0_0: [f32; 3] = [(i as f32 + 1.0), (j as f32), (k as f32)];
                    let v1_0_1: [f32; 3] = [(i as f32 + 1.0), (j as f32), (k as f32 + 1.0)];
                    let v1_1_0: [f32; 3] = [(i as f32 + 1.0), (j as f32 + 1.0), (k as f32)];
                    let v1_1_1: [f32; 3] = [(i as f32 + 1.0), (j as f32 + 1.0), (k as f32 + 1.0)];

                    /*
                    if cube_right || cube_left || cube_above || cube_below || cube_front || cube_hind {
                        atr_uv.extend(uv_from_block_type(BlockType::Grass));
                    }
                    */
                    

                    // check each touching face
                    if cube_below {
                        atr_pos.push(v0_0_1);
                        atr_pos.push(v0_0_0);
                        atr_pos.push(v1_0_0);
                        atr_pos.push(v1_0_1);
                        
                        atr_uv.extend(uv_from_block_type(BlockType::Grass));


                        atr_norm.push([0.0, -1.0, 0.0]);
                        atr_norm.push([0.0, -1.0, 0.0]);
                        atr_norm.push([0.0, -1.0, 0.0]);
                        atr_norm.push([0.0, -1.0, 0.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;
                    }
                    if cube_above {
                        // i/k plane 
                        atr_pos.push(v1_1_1); //111
                        atr_pos.push(v1_1_0); //110
                        atr_pos.push(v0_1_0); //010
                        atr_pos.push(v0_1_1); //011

                        atr_uv.extend(uv_from_block_type(BlockType::Grass));

                        atr_norm.push([0.0, 1.0, 0.0]);
                        atr_norm.push([0.0, 1.0, 0.0]);
                        atr_norm.push([0.0, 1.0, 0.0]);
                        atr_norm.push([0.0, 1.0, 0.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;

                    } 
                    if cube_right {
                        atr_pos.push(v1_1_1); 
                        atr_pos.push(v1_0_1); 
                        atr_pos.push(v1_0_0); 
                        atr_pos.push(v1_1_0);

                        atr_uv.extend(uv_from_block_type(BlockType::Grass));


                        atr_norm.push([1.0, 0.0, 0.0]);
                        atr_norm.push([1.0, 0.0, 0.0]);
                        atr_norm.push([1.0, 0.0, 0.0]);
                        atr_norm.push([1.0, 0.0, 0.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;

                    }
                    if cube_left {
                        atr_pos.push(v0_1_0); 
                        atr_pos.push(v0_0_0); 
                        atr_pos.push(v0_0_1); 
                        atr_pos.push(v0_1_1);

                        atr_uv.extend(uv_from_block_type(BlockType::Grass));


                        atr_norm.push([-1.0, 0.0, 0.0]);
                        atr_norm.push([-1.0, 0.0, 0.0]);
                        atr_norm.push([-1.0, 0.0, 0.0]);
                        atr_norm.push([-1.0, 0.0, 0.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;

                    }
                    if cube_front {
                        atr_pos.push(v0_1_1); 
                        atr_pos.push(v0_0_1); 
                        atr_pos.push(v1_0_1); 
                        atr_pos.push(v1_1_1);

                        atr_uv.extend(uv_from_block_type(BlockType::Grass));


                        atr_norm.push([0.0, 0.0, 1.0]);
                        atr_norm.push([0.0, 0.0, 1.0]);
                        atr_norm.push([0.0, 0.0, 1.0]);
                        atr_norm.push([0.0, 0.0, 1.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;
                    }

                    if cube_hind {
                        atr_pos.push(v1_1_0); 
                        atr_pos.push(v1_0_0); 
                        atr_pos.push(v0_0_0); 
                        atr_pos.push(v0_1_0);

                        atr_uv.extend(uv_from_block_type(BlockType::Grass));


                        atr_norm.push([0.0, 0.0, -1.0]);
                        atr_norm.push([0.0, 0.0, -1.0]);
                        atr_norm.push([0.0, 0.0, -1.0]);
                        atr_norm.push([0.0, 0.0, -1.0]);

                        indices.push(indices_counter);
                        indices.push(indices_counter + 1);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 2);
                        indices.push(indices_counter + 3);
                        indices.push(indices_counter);

                        indices_counter += 4;
                    }
                }
            }
        }
    }


    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            atr_pos
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            atr_uv
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            atr_norm
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(indices));
    let result = indices_counter != 0;

    (mesh, result)
}

fn uv_from_block_type(
    block_type: BlockType,
) -> Vec<[f32; 2]> {
    match block_type {
        BlockType::Grass => {
            vec![[0.51, 1.0], [0.51, 0.0], [1.0, 0.0], [1.0, 1.0]]
}
        BlockType::Stone => {
            vec![[0.0, 1.0], [0.0, 0.0], [0.49, 0.0], [0.49, 1.0]]

}
    }

}


/*
fn create_simple_parallelogram() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, 2.0, 0.0], [1.0, 0.0, 0.0]]
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 1.0], [0.5, 0.0], [1.0, 0.0], [0.5, 1.0]]
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(vec![
            // First triangle
            0, 3, 1,
            // Second triangle
            1, 3, 2
        ]))
}
*/

/*
fn create_chunk_mesh(positions: [[[bool; 18]; 18]; 18], mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: Resmut<Assets<Material>>) {
    // position is 18 x 18 x 18 due to the extra position needed at all ends of the chunk
    let mut chunk_tops = [[[false; 16]; 16]; 16];
    for i in 1..16 {
        for j in 1..16 {
            for k in 1..16 {
                // check for empty voxel above
                if positions[i][j + 1][k] == false {
                    commands.spawn(PbrBundle {
                       mesh: meshes.add(create_simple_parallelogram()),
                       transform: Transform::from_xyz(0.0, 1.0, 0.0),
                       material: materials.add(Color::WHITE),
                       ..default()
                    });

                } 
            }
        }
    }

    /*
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[]]
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[]]
        )
        .with_inserted_attribute(
            Indices::U32(vec![

            ])
        )
    */
}
*/

/*
fn create_chunk_mesh() -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
        .with_inserted_attribute(
            vec![
                [-0.5, -0.5, -0.5]
            ]
        )

}
*/
