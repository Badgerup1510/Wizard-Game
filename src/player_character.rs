use std::{f32::consts::FRAC_PI_2, fmt::Debug};
use bevy::{render::mesh::CylinderAnchor, window::{CursorGrabMode, PrimaryWindow}};

use avian3d::prelude::*;

use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster,
    prelude::*, render::view::RenderLayers,
};

use crate::world::{self, Player};

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;



pub struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initial_grab_cursor);
        app.add_systems(Update, (rotate_player, move_player, player_interaction));
        app.add_systems(Update, escape_handle);
    }
}

#[derive(Component)]
pub struct PlayerCharacterComponent;

#[derive(Component)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

pub fn spawn_view_model(
mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 10.0, 0.0),
            Visibility::default(),
            RigidBody::Dynamic,
            //Collider::capsule(0.3, 2.0),
            Collider::cuboid(0.9,1.8,0.9),
            LockedAxes::ROTATION_LOCKED,
            //Friction::new(0.0),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),


        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                CameraSensitivity::default(),
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 60.0_f32.to_radians(), // originally 90 deg
                    ..default()
                }),
                DistanceFog {
                    color: Color::srgba(0.35, 0.35, 0.35, 1.0),
                    directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
                    directional_light_exponent: 30.0,
                    falloff: FogFalloff::from_visibility_colors(
                        500.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                        Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                        Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
                    ),
                },
            ));

            // Spawn view model camera.
            parent.spawn((
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));

            // Spawn the player's right arm.        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
            parent.spawn((
                DirectionalLight {
                    color: Color::srgb(0.98, 0.95, 0.82),
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-1.0, -1.0, -1.0), Dir3::NEG_Y),
            ));
        });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0,1.0,1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
}

fn escape_handle(
    keys: Res<ButtonInput<KeyCode>>,
    mut window_q: Query<&mut Window>
) {
    let Ok(mut window) = window_q.get_single_mut() else {
        return
    };
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(&mut window);
    }
}




fn rotate_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<WorldModelCamera>>,
) {
    let Ok((mut transform, camera_sensitivity)) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.0.x;
        let delta_pitch = -delta.y * camera_sensitivity.0.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

const MOVE_SPEED: f32 = 1.0;
const DECCELERATION_PERCENT: f32 = 0.9;

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut LinearVelocity, Entity), With<Player>>,
    camera: Query<&Transform, With<WorldModelCamera>>,
    spatial_query: SpatialQuery,
) {
    let Ok((mut velocity, player_entity)) = player.get_single_mut() else {
        return;
    };
    let Ok(transform) = camera.get_single() else {
        return;
    };

    velocity.x *= DECCELERATION_PERCENT;
    velocity.z *= DECCELERATION_PERCENT;
    velocity.y *= DECCELERATION_PERCENT;

    velocity.y -= 2.0;
    
    let forward = transform.forward();
    //let horizontal = Vec2::new(forward.x, forward.z).normalize_or(Vec2::new(1.0, 0.0));
    let forward_2d = Vec2::new(forward.x, forward.z).normalize_or(Vec2::X);
    let right_2d = Vec2::new(forward_2d.y, -forward.x);


    let max_distance = 2.0;
    let solid = true;
    let filter = SpatialQueryFilter::default();
    if let Some(first_hit) = spatial_query.cast_ray(transform.translation, Dir3::NEG_Y, max_distance, solid, &filter) {
        //println!("First hit: {:?}", first_hit);
    }

    let mut velocity_buffer: Vec2 = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        velocity_buffer += forward_2d;
        //velocity.x = MOVE_SPEED * horizontal.x;
        //velocity.z = MOVE_SPEED * horizontal.y;
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity_buffer -= right_2d;
        //velocity.x = -MOVE_SPEED * horizontal.y;
        //velocity.z = MOVE_SPEED * horizontal.x;
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity_buffer -= forward_2d;
        //velocity.x = -MOVE_SPEED * horizontal.x;
        //velocity.z = -MOVE_SPEED * horizontal.y;
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity_buffer += right_2d;
        //velocity.x = MOVE_SPEED * horizontal.y;
        //velocity.z = -MOVE_SPEED * horizontal.x;
    }
    if keys.just_pressed(KeyCode::Space) {
        velocity.y += MOVE_SPEED * 30.0;
    }
    //if keys.pressed(KeyCode::ShiftLeft) {
    //    velocity.y = -10.0 * MOVE_SPEED;
    //}
    if keys.pressed(KeyCode::ArrowRight) {
        velocity.x = MOVE_SPEED;
    }
    if velocity_buffer.length_squared() > 0.0 {
        let move_dir = velocity_buffer.normalize();
        velocity.x += move_dir.x * MOVE_SPEED;
        velocity.z += move_dir.y * MOVE_SPEED;
    }
    //velocity.x = MOVE_SPEED * velocity_buffer.x;
    //velocity.z = MOVE_SPEED * velocity_buffer.y;
}

fn player_interaction(
    buttons: Res<ButtonInput<MouseButton>>,
    player: Query<(&Transform, Entity), With<Player>>,
    camera: Query<&Transform, With<WorldModelCamera>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((player_transform, player_entity)) = player.get_single() else {
        return
    };
    
    let Ok(camera_transform) = camera.get_single() else {
        return
    };

    let vec_forward = camera_transform.forward();

    if let Some(ray_hit) = spatial_query.cast_ray(player_transform.translation, vec_forward, 10.0, true, &SpatialQueryFilter::default().with_excluded_entities([player_entity])) {
        //println!("Exluding {}, hit {}, at {}", player_entity, ray_hit.entity, ray_hit.distance);
        if buttons.just_pressed(MouseButton::Left) {
            println!("left mouse click at {:?}, looking at: {:?}", cube_from_ray_data(player_transform.translation, Some(vec_forward), ray_hit.distance + 0.1), vec_forward);
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_translation(player_transform.translation + (ray_hit.distance * Vec3::from(vec_forward)))
            ));
        }

    }

}

fn cube_from_ray_data(
    transform: Vec3,
    direction: Option<Dir3>,
    distance: f32,
) -> (i32, i32, i32) {
    let dir = direction.unwrap_or(Dir3::X);
    let hit_location = transform + (distance * Vec3::from(dir));
    (hit_location.x as i32, hit_location.y as i32, hit_location.z as i32)
}

/*
fn chunk_from_translation(
    translation: Vec3,
) -> (u32, u32) {

}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_from_translation() {
        let transform = Vec3::ZERO;
        let direction: Option<Dir3> = Some(Dir3::X);
        let distance = 1.0;
        let result = cube_from_ray_data(transform, direction, distance);
        assert_eq!(result, (1, 0, 0));
        let transform1 = Vec3::ZERO;
        let direction1: Option<Dir3> = Some(Dir3::Y);
        let distance1 = 1.45;
        let result1 = cube_from_ray_data(transform1, direction1, distance1);
        assert_eq!(result1, (0, 1, 0));
        let transform2 = Vec3::ZERO;
        let direction2: Option<Dir3> = Some(Dir3::Y);
        let distance2 = 1.55;
        let result2 = cube_from_ray_data(transform2, direction2, distance2);
        assert_eq!(result2, (0, 2, 0));

    }
}
