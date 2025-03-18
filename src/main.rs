use macroquad::prelude::*;
use std::io;

const TILE_SIZE: f32 = 32.0; // each tile will be drawn at 32x32 pixels

mod map;
mod player;

// Bring Player and Map into scope
use map::Map;
use player::Player;

#[macroquad::main(window_conf)]
async fn main() {
    // For demonstration, create a player
    let mut player = Player::new(100.0, 100.0);

    let game_map = Map::from_file("maps/map1.txt");

    loop {
        clear_background(BLACK);

        // Update the player
        player.update(&game_map);
        
        // Draw the map
        game_map.draw(32.0);

        // Example draw for the player:
        draw_rectangle(player.x, player.y, 32.0, 32.0, YELLOW);

        draw_text(
            "Map loaded from map1.txt. Press ESC to quit.",
            20.0,
            20.0,
            24.0,
            WHITE,
        );

        if is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Space) {
            player.jump(&game_map);
        }

        if is_key_down(KeyCode::Left) {
            player.move_left()
        }
        if is_key_down(KeyCode::Right) {
            player.move_right()
        }

        // Next frame
        next_frame().await;
    }
}

/// Optional: window configuration function
fn window_conf() -> Conf {
    Conf {
        window_title: "My Game".to_owned(),
        window_width: 32 * 40,
        window_height: 32 * 20,
        window_resizable: false,
        ..Default::default()
    }
}
