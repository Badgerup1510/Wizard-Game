use bevy::prelude::*;
use std::collections::HashMap;


// Need to make VoxelWorld Use hashsets somehow for performance.

// Define constants:
const WORLD_SIZE: usize = 2<<16;
const WORLD_HALF_SIZE: usize = WORLD_SIZE / 2;
const CHUNK_SIZE: usize = 32;

#[derive(Component)]
pub struct PlayerComponent;

pub struct VoxelWorldPlugin;

impl Plugin for VoxelWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_voxel_world);
    }
}

#[derive(Component)]
struct VoxelWorld {
    //chunks: [[[Chunk; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE],
    chunks: HashMap<Chunk, Vec3> 
}

impl VoxelWorld {
    fn new() -> Self {
        VoxelWorld{
            chunks: HashMap::new(),
        }
    }
    /*
    fn render_chunks(&self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<Assets<StandardMaterial>>>,
        player_query: Query<&Transform, With<PlayerComponent>>
        ) {
        const RENDER_DISTANCE: u32 = 3;
    }
    */
}

struct Chunk {
    render_state: RenderState,
    load_state: LoadState,
}

enum RenderState  {
    NotSpawned,
    Spawned,
}

enum LoadState {
    NotLoaded,
    Loaded {
        cubes: [[[Cube;  CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    },
}

struct Cube {
    cube_type: CubeType,
}

enum CubeType {
    Air,
    Dirt,
    Grass,
}

fn setup_voxel_world(mut commands: Commands) {
    commands.spawn((SpatialBundle{
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }, VoxelWorld::new()));
    
}

