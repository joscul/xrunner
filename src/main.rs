use macroquad::prelude::*;

mod map;
mod player;
mod command;

// Bring Player and Map into scope
use map::Map;
use player::Player;
use command::Command;

enum GameState {
	GamePlay,
	WinScreen,
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut current_screen = GameState::GamePlay;
	// For demonstration, create a player
	let mut player = Player::new(32.0, 32.0);

	let mut game_map = Map::from_file("maps/map1.txt").await;

	loop {
		match current_screen {
			GameState::GamePlay => {
				clear_background(SKYBLUE);

				// Update the player
				let commands = player.update(&game_map);

				// We own the map object so we need to change it.
				for command in commands {
					match command {
						Command::RemoveEntity(ch, tile_x, tile_y) => {
							game_map.remove_entity(ch, tile_x, tile_y);
						},
						Command::DisplayWinScreen(_tile_x, _tile_y) => {
							current_screen = GameState::WinScreen;
						}
					}
				}
				
				// Draw the map
				game_map.draw(32.0);

				let fps = get_fps();

				// Example draw for the player:
				draw_rectangle(player.x, player.y, 32.0, 32.0, YELLOW);

				draw_text(
					format!("Toggle gravity using G key. Press ESC to quit. Gravity: {} vx: {} vy: {} g: {} fps: {}", if player.gravity() > 0.0 { "on" } else { "off" }, player.vx(), player.vy(), player.gravity(), fps).as_str(),
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
				if is_key_down(KeyCode::Up) {
					player.move_up(&game_map)
				}
				if is_key_down(KeyCode::Down) {
					player.move_down()
				}
			}
			GameState::WinScreen => {
				// Win screen logic and rendering here
				clear_background(GREEN);
				draw_text("You Won! Press ESC to Exit", 100.0, 100.0, 30.0, BLACK);

				if is_key_pressed(KeyCode::Escape) {
					break;
				}
			}
		}

		// Next frame
		next_frame().await;
	}
}

/// Optional: window configuration function
fn window_conf() -> Conf {
	Conf {
		window_title: "XRunner".to_owned(),
		window_width: 32 * 40,
		window_height: 32 * 20,
		window_resizable: false,
		..Default::default()
	}
}
