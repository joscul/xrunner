use crate::map::Map;
use macroquad::prelude::*;

pub struct Player {
	pub x: f32,
	pub y: f32,
	pub vx: f32,
	pub vy: f32,
	pub g: f32,
}

impl Player {

	// new player with x and y coords.
	pub fn new(x: f32, y: f32) -> Self {
		Player {
			x, y, vx: 0.0, vy: 0.0, g: 0.0,
		}
	}

	// update player position on map and draw it.
	pub fn update(&mut self, map: &Map) {

		// player is falling more and more as default. This is gravity.
		if (self.has_gravity()) {
			self.update_with_gravity(map);
		} else {
			self.update_without_gravity(map);
		}
	}

	fn update_with_gravity(&mut self, map: &Map) {
		self.vy += self.gravity();
		if map.is_solid(self.left(), self.bottom() + self.vy) {
			self.vy = 0.0;
		}

		// Unless it stands on something.
		// move to left
		while map.is_solid(self.right(), self.bottom()) && map.is_solid(self.right(), self.top()) {
			self.vy = 0.0;
			self.vx = 0.0;
			self.x -= 0.1;
		}
		// move right
		while map.is_solid(self.left(), self.bottom()) && map.is_solid(self.left(), self.top()) {
			self.vy = 0.0;
			self.vx = 0.0;
			self.x += 0.1;
		}
		// move up from solid.
		while map.is_solid(self.left(), self.bottom()) || map.is_solid(self.right(), self.bottom()) {
			self.vy = 0.0;
			self.y -= 0.1;
		}

		// or hits a roof.
		while map.is_solid(self.left(), self.top() + self.vy) || map.is_solid(self.right(), self.top() + self.vy) {
			self.vy = 0.0;
			self.y += 0.1;
		}

		let mut vy = self.vy * self.vy.abs().sqrt();

		if (vy > 10.0) {
			vy = 10.0;
		}

		let delta = get_frame_time() * 100.0;

		self.x += self.vx * delta;
		self.y += vy * delta;

		// reset
		self.vx = 0.0;
	}

	fn update_without_gravity(&mut self, map: &Map) {

		if (map.is_solid(self.left(), self.bottom() + self.vy) || map.is_solid(self.right(), self.bottom() + self.vy)) {
			self.vy = 0.0
		}
		if (map.is_solid(self.left(), self.top() + self.vy) || map.is_solid(self.right(), self.top() + self.vy)) {
			self.vy = 0.0
		}
		if (map.is_solid(self.right() + self.vx, self.top()) || map.is_solid(self.right() + self.vx, self.bottom())) {
			self.vx = 0.0
		}
		if (map.is_solid(self.left() + self.vx, self.top()) || map.is_solid(self.left() + self.vx, self.bottom())) {
			self.vx = 0.0
		}

		let delta = get_frame_time() * 100.0;

		self.x += self.vx * delta;
		self.y += self.vy * delta;
	}

	pub fn jump(&mut self, map: &Map) {
		// can only jump if standing on a solid
		if (self.gravity() > 0.0) {
			if map.is_solid(self.center(), self.bottom() + Player::delta()) {
				self.vy -= 4.0;
			}
		}
	}

	pub fn move_right(&mut self, map: &Map) {
		if (self.has_gravity()) {
			self.vx = 3.0;
		} else {
			if (self.vy == 0.0 && self.vx == 0.0) {
				let distance = map.ray_cast(self.center(), self.center_y(), 1.0, 0.0) / 80.0;
				self.vx = distance;
			}
		}
	}

	pub fn move_left(&mut self, map: &Map) {
		if (self.gravity() > 0.0) {
			self.vx = -3.0;
		} else {
			if (self.vy == 0.0 && self.vx == 0.0) {
				let distance = map.ray_cast(self.center(), self.center_y(), -1.0, 0.0) / 80.0;
				self.vx = -distance;
			}
		}
	}

	pub fn move_up(&mut self, map: &Map) {
		if (self.gravity() > 0.0) {
		} else {
			if (self.vy == 0.0 && self.vx == 0.0) {
				let distance = map.ray_cast(self.center(), self.center_y(), 0.0, -1.0) / 80.0;
				self.vy = -distance;
			}
		}
	}

	pub fn move_down(&mut self, map: &Map) {
		if (self.gravity() > 0.0) {
		} else {
			if (self.vy == 0.0 && self.vx == 0.0) {
				let distance = map.ray_cast(self.center(), self.center_y(), 0.0, 1.0) / 80.0;
				self.vy = distance;
			}
		}
	}

	pub fn center(&self) -> f32 {
		return self.x + 16.0;
	}

	pub fn center_y(&self) -> f32 {
		return self.y + 16.0;
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

	pub fn delta() -> f32 {
		return 0.1;
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
		if (self.g > 0.0) {
			self.g = 0.0;
		} else {
			self.g = 0.1;
		}
	}

	pub fn has_gravity(&mut self) -> bool {
		if (self.g > 0.0) {
			return true
		} else {
			return false
		}
	}


}
