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
    app.add_systems(Startup, chunk_startup);
}

pub fn generate_chunk_data(position: Position) -> [[[bool; 16]; 16]; 16] {
    // define empty chunk
    let mut chunk = [[[false; 16];16];16];

    // perlin noise parameters
    let octaves: i32 = 6; // detail
    let amplitude: f64 = 20.0; // the absolute output value 
    let frequency: f64 = 0.5; //cycles per unit length ???
    let persistence: f64 = 1.0; // determines how the amplitude diminishes
    let lacunarity: f64 = 2.0; // determines frequency increses of octaves
    let scale: (f64, f64) = (100.0, 100.0); // a distance to view the noise map ???
    let bias: f64 = 0.0; // Used to make output positive
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
    -> [[[bool; 18]; 18]; 18]
{
    let m_x = position.x;
    let m_y = position.y;
    let m_z = position.z;

    // Define chunks
    let mut chunk: [[[bool; 18]; 18]; 18] = [[[false; 18]; 18]; 18];
    let main_chunk = generate_chunk_data(position);
    let top_chunk = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z});
    let bottom_chunk = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z});
    let right_chunk = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z});
    let left_chunk = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z});
    let front_chunk = generate_chunk_data(Position{x: m_x, y: m_y, z: m_z + 1});
    let back_chunk = generate_chunk_data(Position{x: m_x, y: m_y, z: m_z - 1});
    // Diagonals
    // i/j
    let cube_right_up = generate_chunk_data(Position{x: m_x + 1, y: m_y + 1, z: m_z});// [i + 1][j + 1][k];
    let cube_right_down = generate_chunk_data(Position{x: m_x + 1, y: m_y - 1, z: m_z});//[i + 1][j - 1][k];
    let cube_left_up = generate_chunk_data(Position{x: m_x - 1, y: m_y + 1, z: m_z});//[i - 1][j + 1][k];
    let cube_left_down = generate_chunk_data(Position{x: m_x - 1, y: m_y - 1, z: m_z});//[i - 1][j - 1][k];
    // k/j
    let cube_front_up = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z + 1});//[i][j + 1][k + 1];
    let cube_front_down = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z + 1});//[i][j - 1][k + 1];
    let cube_hind_up = generate_chunk_data(Position{x: m_x, y: m_y + 1, z: m_z - 1});//[i][j + 1][k - 1];
    let cube_hind_down = generate_chunk_data(Position{x: m_x, y: m_y - 1, z: m_z - 1});//[i][j - 1][k - 1];
    // i/k
    let cube_left_front = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z + 1});
    let cube_left_hind = generate_chunk_data(Position{x: m_x - 1, y: m_y, z: m_z - 1});
    let cube_right_front = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z + 1});
    let cube_right_hind = generate_chunk_data(Position{x: m_x + 1, y: m_y, z: m_z - 1});

    // Set chunk
    // Main chunk

    for i in 0..16 {
        for j in 0..16 {
            for k in 0..16 {
                // Centre
                chunk[i + 1][j + 1][k + 1] = main_chunk[i][j][k];
            }
        }
    }



    // Left and Right
    for k in 1..16 {
        for j in 1..16 {
            // Left
            chunk[0][j][k] = left_chunk[15][j][k];

            // Right
            chunk[17][j][k] = right_chunk[0][j][k];
        }
    }

    // Top and Bottom
    for i in 1..16 {
        for k in 1..16 {
            // Bottom
            chunk[i][0][k] = bottom_chunk[i][15][k];
            // Top
            chunk[i][17][k] = top_chunk[i][0][k];

        }
    }

    // Front and Back
    for i in 1..16 {
        for j in 1..16 {
            // Back
            chunk[i][j][0] = back_chunk[i][j][15];
            // Front
            chunk[i][j][17] = front_chunk[i][j][0];
        }
    }


    for a in 0..16 {
        // i/j
        chunk[0][17][a + 1] = cube_left_up[15][0][a];
        chunk[0][0][a + 1] = cube_left_down[15][15][a];
        chunk[17][17][a + 1] = cube_right_up[0][0][a];
        chunk[17][0][a + 1] = cube_right_down[0][15][a];
        // k/j
        chunk[a + 1][17][0] = cube_hind_up[a][0][15];
        chunk[a + 1][0][0] = cube_hind_down[a][15][15];
        chunk[a + 1][0][17] = cube_front_down[a][15][0];
        chunk[a + 1][17][17] = cube_front_up[a][0][0];
        // i/k
        chunk[0][a + 1][17] = cube_left_front[15][a][0];
        chunk[0][a + 1][0] = cube_left_hind[15][a][15];
        chunk[17][a + 1][0] = cube_right_hind[0][a][15];
        chunk[17][a + 1][17] = cube_right_front[0][a][0];
    }
    println!("{}", chunk[0][0][0]);

    chunk

    /*
    // spawn chunk
    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_chunk_mesh(chunk)),
        transform: Transform::from_xyz(m_x as f32 * 16.0, m_y as f32 * 16.0, m_z as f32 * 16.0),
        material: materials.add(Color::WHITE),
        ..default()
    });
    */
}

/*
fn shift_right(arr: &mut [[[f32; 18]; 18]; 18]) {
    // Flatten the 3D array into a 1D iterator
    let mut flat_arr: Vec<bool> = arr.iter().flatten().flatten().copied().collect();

    // Shift the flat array
    let last = flat_arr.pop().unwrap();  // Remove the last element
    flat_arr.insert(0, last);  // Insert the last element at the start

    // Reshape the flat array back into the 3D structure
    let mut iter = flat_arr.into_iter();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                arr[i][j][k] = iter.next().unwrap();
            }
        }
    }
}
*/

pub fn generate_chunk_mesh(chunk: [[[bool; 18]; 18]; 18]) -> Mesh {
    // init empty triangle list mesh
    
    // define mesh attribute vectors
    let mut atr_pos: Vec<[f32; 3]> = vec![];
    let mut atr_uv: Vec<[f32; 2]> = vec![];
    let mut atr_norm: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];

    let mut indices_counter = 0;

    // loop through each position in chunk
    for i in 1..17 {
        for j in 1..17 {
            for k in 1..17 {
                // if current cube exists
                //println!("Chunk: {}", chunk[i][j][k]);
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


                    // check each touching face
                    if cube_below {
                        atr_pos.push(v0_0_1);
                        atr_pos.push(v0_0_0);
                        atr_pos.push(v1_0_0);
                        atr_pos.push(v1_0_1);
                        
                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0,0.0]);
                        atr_uv.push([1.0, 1.0]);

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

                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0, 0.0]);
                        atr_uv.push([1.0, 1.0]);

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

                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0, 0.0]);
                        atr_uv.push([1.0, 1.0]);

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

                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0, 0.0]);
                        atr_uv.push([1.0, 1.0]);

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

                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0, 0.0]);
                        atr_uv.push([1.0, 1.0]);

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

                        atr_uv.push([0.0, 1.0]);
                        atr_uv.push([0.0, 0.0]);
                        atr_uv.push([1.0, 0.0]);
                        atr_uv.push([1.0, 1.0]);

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
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
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
        .with_inserted_indices(Indices::U32(indices))

}

/*

*/
/*
                       atr_pos.push([
                            [, , ], [, , ], [, , ], 
                            [[], [], []],
                            [[], [], []],
                            [[], [], []]

                            ]);
                        atr_uv.push([
                            [, ], [, ], [, ], 
                            [[], [], []],
                            [[], [], []],
                            [[], [], []]

                            ]);
                        atr_norm.push([
                            [, , ], [, , ], [, , ], 
                            [[], [], []],
                            [[], [], []],
                            [[], [], []]

                            ]);

*/

fn chunk_startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let temp: Position  = Position {
       x: 0,
       y: 0,
       z: 0,
    };

    //println!("{:?} -> {:?}", generate_chunk(Position{x: 0, y: -2, z: 0}), generate_chunk(Position{x: 1, y: -2, z: 0}))

    //generate_chunk(temp);
    //println!("spawned chunk");
    /*
    generate_chunk_data(temp);


    commands.spawn(PbrBundle {
       mesh: meshes.add(create_simple_parallelogram()),
       transform: Transform::from_xyz(0.0, 1.0, 0.0),
       material: materials.add(Color::WHITE),
       ..default()
    });
    */
}

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
