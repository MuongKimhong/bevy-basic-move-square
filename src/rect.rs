use crate::window;
use bevy::prelude::*;

pub const RECT_WIDTH: f32 = 128.0;
pub const RECT_HEIGHT: f32 = 128.0;

#[derive(Resource)]
pub struct RectanglePosition(pub Vec3);

#[derive(Component)]
pub struct RectangleMarker;

pub fn update_rectangle_pos(
    rectangle_pos: &ResMut<RectanglePosition>,
    query: &mut Query<&mut Transform, With<RectangleMarker>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation = rectangle_pos.0;
    }
}

pub fn handle_move_left(rectangle_pos: &mut ResMut<RectanglePosition>) {
    if rectangle_pos.0.x > -window::WINDOW_WIDTH / 2.0 + RECT_WIDTH / 2.0 {
        rectangle_pos.0.x -= 1.0;
    }
}

pub fn handle_move_right(rectangle_pos: &mut ResMut<RectanglePosition>) {
    if rectangle_pos.0.x + RECT_WIDTH / 2.0 < window::WINDOW_WIDTH / 2.0 {
        rectangle_pos.0.x += 1.0;
    }
}

pub fn handle_move_up(rectangle_pos: &mut ResMut<RectanglePosition>) {
    if rectangle_pos.0.y + RECT_HEIGHT / 2.0 < window::WINDOW_HEIGHT / 2.0 {
        rectangle_pos.0.y += 1.0;
    }
}

pub fn handle_move_down(rectangle_pos: &mut ResMut<RectanglePosition>) {
    if rectangle_pos.0.y > -window::WINDOW_HEIGHT / 2.0 + RECT_HEIGHT / 2.0 {
        rectangle_pos.0.y -= 1.0;
    }
}
