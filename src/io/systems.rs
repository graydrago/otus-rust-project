use bevy::prelude::*;
use crate::components::{Player, Speed};
use crate::io::Control;

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &Speed, &mut Transform)>
) {
    let mut direction = Vec3::ZERO;
    let (_, Speed(speed), mut player_transform) = query.single_mut();
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    player_transform.translation += direction.normalize_or_zero() * *speed;
}

pub fn mouse_input_system(
    mut control: ResMut<Control>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    mut camera: Query<(&Camera, &GlobalTransform, &Transform)>,
) {
    let (camera, camera_global_transform, &camera_transform) = camera.single_mut();
    if let Some(cursor_movement) = mouse_motion_events.iter().last() {
        let usign_screen_size = camera.physical_viewport_size().unwrap();
        let screen_size = Vec2::new(usign_screen_size.x as f32, usign_screen_size.y as f32);
        let screen_ndc_2d = (cursor_movement.position / screen_size) * 2.0 - Vec2::ONE;

        let ndc_to_world_matrix = camera_global_transform.compute_matrix() * camera.projection_matrix().inverse();

        control.cursor_ray.origin = camera_transform.translation;
        control.cursor_ray.normal = (camera_transform.translation - ndc_to_world_matrix.project_point3(screen_ndc_2d.extend(-1.0))).normalize();
    }
}
