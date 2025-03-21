
use crate::map::Map;
use crate::command::Command;

use macroquad::prelude::*;
pub struct Player {
	pub x: f32,
	pub y: f32,
	pub vx: f32,
	pub vy: f32,
	pub g: f32,
	sprite_bg1: Texture2D,
}

impl Player {

	// new player with x and y coords.
	pub async fn new() -> Self {

		let sprite_bg1 = load_texture("sprites/bg1.png").await.unwrap();
		sprite_bg1.set_filter(FilterMode::Nearest);

		Player {
			x: Map::TILE_SIZE, y: Map::TILE_SIZE, vx: 0.0, vy: 0.0, g: 0.1, sprite_bg1: sprite_bg1,
		}
	}

	// update player position on map and draw it.
	pub fn update(&mut self, map: &Map) -> Vec<Command> {

		// player is falling more and more as default. This is gravity.
		if self.has_gravity() {
			self.update_with_gravity(map);
		} else {
			self.update_without_gravity(map);
		}

		let mut commands = Vec::new();

		// check if we have a collision with an entity.
		match map.raycast_any((self.center_x(), self.center_y()), (0.0, 1.0), 100.0, 'g') {
			Some((dist, tile_x, tile_y)) => {
				if dist <= 16.0 {
					self.gravity_toggle();
					commands.push(Command::RemoveEntity('g', tile_x, tile_y));
				}
			}
			None => {
			}
		}

		match map.raycast_any((self.center_x(), self.center_y()), (0.0, 1.0), 100.0, 'p') {
			Some((dist, tile_x, tile_y)) => {
				if dist <= 16.0 {
					commands.push(Command::LoadNextMap(tile_x, tile_y));
				}
			}
			None => {
			}
		}

		return commands;
	}

	// draw the player tile.
	pub fn draw(&self) {
		// Example draw for the player:
		//draw_rectangle(self.x, self.y, 32.0, 32.0, YELLOW);
		self.texture_rot(self.sprite_bg1, self.x, self.y, 11, 7, 0.0);
	}

	pub fn reset(&mut self) {
		self.x = Map::TILE_SIZE;
		self.y = Map::TILE_SIZE;
		self.vx = 0.0;
		self.vy = 0.0;
		self.g = 0.1;
	}

	fn update_with_gravity(&mut self, map: &Map) {
		let delta = get_frame_time() * 100.0;
		self.vy += self.gravity() * delta;

		let search_distance = 100.0;

		let mut y_distance = f32::INFINITY;

		// check if we are standing on something.
		if self.vy > 0.0 {
			// moving down.
			match map.raycast((self.right() - 1.0, self.bottom()), (0.0, 1.0), search_distance) {
				Some(dist) => {
					y_distance = f32::min(dist, y_distance);
				}
				None => {
				}
			}
			match map.raycast((self.left() + 1.0, self.bottom()), (0.0, 1.0), search_distance) {
				Some(dist) => {
					y_distance = f32::min(dist, y_distance);
				}
				None => {
				}
			}
		} else if self.vy < 0.0 {
			// moving up.
			match map.raycast((self.left() + 1.0, self.top()), (0.0, -1.0), search_distance) {
				Some(dist) => {
					y_distance = f32::min(dist, y_distance);
				}
				None => {
				}
			}
			match map.raycast((self.right() - 1.0, self.top()), (0.0, -1.0), search_distance) {
				Some(dist) => {
					y_distance = f32::min(dist, y_distance);
				}
				None => {
				}
			}
		}

		// check if we are moving right or left. if so, we can only move some distance.
		let mut distance = f32::INFINITY;
		if self.vx > 0.0 {
			// moving right.
			match map.raycast((self.right(), self.bottom() - 1.0), (1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.right(), self.top() + 1.0), (1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		} else if self.vx < 0.0 {
			// moving left.
			match map.raycast((self.left(), self.bottom() - 1.0), (-1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.left(), self.top() + 1.0), (-1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		}

		let mut vy = self.vy * self.vy.abs().sqrt();

		if vy > 15.0 {
			vy = 15.0;
		}

		if distance < f32::abs(self.vx * delta) {
			self.vx = (self.vx / f32::abs(self.vx)) * distance / delta;
			if f32::abs(self.vx) < f32::EPSILON {
				self.vx = 0.0;
			}
		}

		if y_distance < f32::abs(vy * delta) {
			vy = (vy / f32::abs(vy)) * y_distance / delta;
			if f32::abs(vy) < f32::EPSILON {
				self.vy = 0.0;
				vy = 0.0;
			}
		}

		self.x += self.vx * delta;
		self.y += vy * delta;

		// reset
		self.vx = 0.0;
	}

	fn update_without_gravity(&mut self, map: &Map) {

		// ray cast in the same direction we are moving.
		let search_distance = 100.0;
		let mut distance = f32::INFINITY;
		if self.vy > 0.0 {
			// moving down.
			match map.raycast((self.right() - 1.0, self.bottom()), (0.0, 1.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.left() + 1.0, self.bottom()), (0.0, 1.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		} else if self.vy < 0.0 {
			// moving up.
			match map.raycast((self.left() + 1.0, self.top()), (0.0, -1.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.right() - 1.0, self.top()), (0.0, -1.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		}

		if self.vx > 0.0 {
			// moving right.
			match map.raycast((self.right(), self.bottom() - 1.0), (1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.right(), self.top() + 1.0), (1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		} else if self.vx < 0.0 {
			// moving left.
			match map.raycast((self.left(), self.bottom() - 1.0), (-1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
			match map.raycast((self.left(), self.top() + 1.0), (-1.0, 0.0), search_distance) {
				Some(dist) => {
					distance = f32::min(dist, distance);
				}
				None => {
				}
			}
		}

		let delta = get_frame_time() * 100.0;

		if distance < f32::abs(self.vx * delta) {
			self.vx = (self.vx / f32::abs(self.vx)) * distance / delta;
			if f32::abs(self.vx) < f32::EPSILON {
				self.vx = 0.0;
			}
		}

		if distance < f32::abs(self.vy * delta) {
			self.vy = (self.vy / f32::abs(self.vy)) * distance / delta;
			if f32::abs(self.vy) < f32::EPSILON {
				self.vy = 0.0;
			}
		}

		self.x += self.vx * delta;
		self.y += self.vy * delta;

	}

	pub fn jump(&mut self, map: &Map) {
		// can only jump if standing on a solid, i.e distance down is zero.
		if self.gravity() > 0.0 {
			let search_distance = 100.0;
			let mut distance = f32::INFINITY;
			if self.vy >= 0.0 {
				// moving down.
				match map.raycast((self.right() - 1.0, self.bottom()), (0.0, 1.0), search_distance) {
					Some(dist) => {
						distance = f32::min(dist, distance);
					}
					None => {
					}
				}
				match map.raycast((self.left() + 1.0, self.bottom()), (0.0, 1.0), search_distance) {
					Some(dist) => {
						distance = f32::min(dist, distance);
					}
					None => {
					}
				}
			}
			if distance == 0.0 {
				self.vy = -4.1;
			}
		}
	}

	pub fn move_right(&mut self) {
		if self.has_gravity() {
			self.vx = 3.0;
		} else {
			if self.vy == 0.0 && self.vx == 0.0 {

				self.vx = 6.0;

			}
		}
	}

	pub fn move_left(&mut self) {
		if self.gravity() > 0.0 {
			self.vx = -3.0;
		} else {
			if self.vy == 0.0 && self.vx == 0.0 {
				self.vx = -6.0;
			}
		}
	}

	pub fn move_up(&mut self, map: &Map) {
		if self.gravity() > 0.0 {
			// jump.
			self.jump(map);
		} else {
			if self.vy == 0.0 && self.vx == 0.0 {
				self.vy = -6.0;
			}
		}
	}

	pub fn move_down(&mut self) {
		if self.gravity() > 0.0 {
		} else {
			if self.vy == 0.0 && self.vx == 0.0 {
				self.vy = 6.0;
			}
		}
	}

	pub fn center_x(&self) -> f32 {
		return self.x + 16.0;
	}

	pub fn center_y(&self) -> f32 {
		return self.y - 16.0;
	}

	pub fn left(&self) -> f32 {
		return self.x;
	}

	pub fn right(&self) -> f32 {
		return self.x + 32.0;
	}

	pub fn bottom(&self) -> f32 {
		return self.y + 32.0;
	}

	pub fn top(&self) -> f32 {
		return self.y;
	}

	pub fn gravity(&self) -> f32 {
		return self.g;
	}

	pub fn vy(&self) -> f32 {
		return self.vy;
	}

	pub fn vx(&self) -> f32 {
		return self.vx;
	}

	pub fn gravity_toggle(&mut self) {
		if self.g > 0.0 {
			self.g = 0.0;
		} else {
			self.g = 0.1;
		}
	}

	pub fn has_gravity(&mut self) -> bool {
		if self.g > 0.0 {
			return true
		} else {
			return false
		}
	}

	fn texture_rot(&self, image: Texture2D, x: f32, y: f32, pos_x: i32, pos_y: i32, rotation_deg: f32) {
		draw_texture_ex(
			image,
			x,
			y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(Map::TILE_SIZE, Map::TILE_SIZE)),
				source: Some(Rect::new(pos_x as f32 * Map::TILE_SIZE, pos_y as f32 * Map::TILE_SIZE, Map::TILE_SIZE, Map::TILE_SIZE)),
				rotation: rotation_deg.to_radians(),
				pivot: None,
				..Default::default()
			},
		);
	}


}
