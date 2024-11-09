pub mod rect;
pub mod window;

use bevy::{color::palettes::basic::PURPLE, prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window::set_window_plugin()))
        .insert_resource(rect::RectanglePosition(Vec3::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut rectangle_pos: ResMut<rect::RectanglePosition>,
    mut query: Query<&mut Transform, With<rect::RectangleMarker>>,
) {
    if keys.pressed(KeyCode::ArrowLeft) {
        rect::handle_move_left(&mut rectangle_pos);
    } else if keys.pressed(KeyCode::ArrowUp) {
        rect::handle_move_up(&mut rectangle_pos);
    } else if keys.pressed(KeyCode::ArrowRight) {
        rect::handle_move_right(&mut rectangle_pos);
    } else if keys.pressed(KeyCode::ArrowDown) {
        rect::handle_move_down(&mut rectangle_pos);
    }
    rect::update_rectangle_pos(&rectangle_pos, &mut query);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rectangle_pos: Res<rect::RectanglePosition>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_translation(rectangle_pos.0).with_scale(Vec3::new(
                rect::RECT_WIDTH,
                rect::RECT_HEIGHT,
                0.0,
            )),
            material: materials.add(Color::from(PURPLE)),
            ..default()
        },
        rect::RectangleMarker,
    ));
}
