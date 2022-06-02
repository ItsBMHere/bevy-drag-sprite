use bevy::prelude::*;
use bevy::core_pipeline::ClearColor;
use bevy::window::CursorMoved;

const SPRITE_SIZE: f32 = 55.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.0,
            height: 1000.0,
            resizable: false,
            title: "Bevy: drag sprite".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
            ..Default::default()
        },
        texture: asset_server.load("sprites/bevy-icon.png"),
        ..Default::default()
    });
}

#[derive(Default, Component)]
struct State {
    cursor_pos: Vec2,
    sprite: Option<(Entity, Vec3)>,
}

fn sprite_system(
    mut state: Local<State>,
    windows: Res<Windows>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut sprites: Query<(Entity, &Sprite)>,
    mut transforms: Query<&mut Transform>,
) {
    let window = windows.get_primary().unwrap();
    let half_window = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    if let Some(cursor_event) = cursor_moved_event_reader.iter().last() {
        state.cursor_pos = cursor_event.position - half_window;
    };

    if mouse_button_input.just_released(MouseButton::Left) {
        state.sprite = None;
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) && state.sprite.is_some() {
        let sprite = state.sprite.unwrap();

        let mut sprite_pos = transforms.get_mut(sprite.0).unwrap();

        info!("Sprite position old: {:?}", sprite_pos.translation);
        sprite_pos.translation.x = state.cursor_pos.x + sprite.1.x;
        sprite_pos.translation.y = state.cursor_pos.y + sprite.1.y;
        info!("Sprite position new: {:?}", sprite_pos.translation);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, sprite) in sprites.iter_mut() {
            let sprite_pos = transforms.get_mut(entity).unwrap().translation;
            let diff = cursor_to_sprite_diff(&state.cursor_pos, &sprite_pos);
            let sprite_size = sprite.custom_size.unwrap();
            if diff.length() < (sprite_size.x / 2.0) {
                state.sprite = Some((entity, diff));
            }
        }
    }
}

fn cursor_to_sprite_diff(cursor_pos: &Vec2, sprite_pos: &Vec3) -> Vec3 {
    Vec3::new(
        sprite_pos.x - cursor_pos.x,
        sprite_pos.y - cursor_pos.y,
        0.0,
    )
}
