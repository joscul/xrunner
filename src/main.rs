use macroquad::prelude::*;
use std::path::Path;

mod map;
mod player;
mod command;

// Bring Player and Map into scope
use map::Map;
use player::Player;
use command::Command;

// Helper enum for storing what state the game is in.
enum GameState {
	GamePlay,
	WinScreen,
	Exit,
}

#[macroquad::main(window_conf)]
async fn main() {

	// Initial state
	let mut current_state = GameState::GamePlay;
	let mut current_map: String = String::from("map1.txt");

	// Create a player
	let mut player = Player::new().await;

	// Load a map
	let mut game_map = Map::from_file(map_file(&current_map)).await;

	// Main loop.
	loop {

		// Do different rendering depending on state.
		match current_state {
			GameState::GamePlay => {

				clear_background(SKYBLUE);

				// Update the player
				let mut commands = player.update(&game_map);

				// Draw the map
				game_map.draw();
				player.draw();

				draw_debug(&mut player);

				commands.extend(handle_keyboard_input(&mut player, &game_map));

				// Execute commands.
				for command in commands {
					match command {
						Command::RemoveEntity(ch, tile_x, tile_y) => {
							println!("Command::RemoveEntity");
							game_map.remove_entity(ch, tile_x, tile_y);
						},
						Command::LoadMap(file_name, exit_portal) => {
							println!("Command::LoadMap");
							if !map_exists(&file_name) {
								current_state = GameState::WinScreen;
							} else {
								current_map = file_name;
								game_map = Map::from_file(map_file(&current_map)).await;
								match game_map.find_portal_coordinates(exit_portal) {
									Some((x, y)) => {
										player.set_spawn_pos((x, y));
									},
									None => {
										player.set_spawn_pos((Map::TILE_SIZE, Map::TILE_SIZE));
									}
								}
								// reset player position.
								player.reset();
							}
						},
						Command::ResetMap() => {
							println!("Command::ResetMap");
							game_map = Map::from_file(map_file(&current_map)).await;
							// reset player position.
							player.reset();
						},
						Command::Exit() => {
							println!("Command::Exit");
							current_state = GameState::Exit;
						}
					}
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
			GameState::Exit => {
				break
			}
		}

		// Next frame
		next_frame().await;
	}
}

fn handle_keyboard_input(player: &mut Player, game_map: &Map) -> Vec<Command> {

	let mut ret: Vec<Command> = Vec::new();

	// Handle all keyboard interactions.
	if is_key_down(KeyCode::Escape) {
		ret.push(Command::Exit());
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
	if is_key_down(KeyCode::R) {
		ret.push(Command::ResetMap());
	}

	return ret;
}

fn draw_debug(player: &mut Player) {
	let fps = get_fps();
	// Debug output
	draw_text(
		format!("Coins: {}, Press R to restart level. Press ESC to quit. Gravity: {} vx: {} vy: {} g: {} fps: {}", player.coins(), if player.gravity() > 0.0 { "on" } else { "off" }, player.vx(), player.vy(), player.gravity(), fps).as_str(),
		20.0,
		20.0,
		24.0,
		WHITE,
	);
}

fn map_file(file_name: &String) -> String {
	format!("maps/{}", file_name)
}

fn map_exists(file_name: &String) -> bool {
	let path_string = map_file(file_name);
	return Path::new(&path_string).exists()
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
