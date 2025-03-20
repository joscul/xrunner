
use macroquad::prelude::*;

pub struct Map {
	pub tiles: Vec<Vec<char>>,
	pub width: usize,
	pub height: usize,
	sprite_bg1: Texture2D,
}

impl Map {

	pub async fn from_file(path: &str) -> Self {

		let sprite_bg1 = load_texture("sprites/bg1.png").await.unwrap();
		sprite_bg1.set_filter(FilterMode::Nearest);

		let content = std::fs::read_to_string(path).unwrap();

		let tiles: Vec<Vec<char>> = content
			.lines()
			.map(|line| line.chars().collect())
			.collect();

		let height = tiles.len();
		let width = if height > 0 {
			tiles[0].len()
		} else {
			0
		};

		Map {
			tiles,
			width,
			height,
			sprite_bg1,
		}
	}

	pub fn draw(&self, tile_size: f32) {
		// Example "draw" routine using Macroquad
		use macroquad::prelude::*;
		for (row_index, row) in self.tiles.iter().enumerate() {
			for (col_index, &tile) in row.iter().enumerate() {
				let x = col_index as f32 * tile_size;
				let y = row_index as f32 * tile_size;
				match tile {
					' ' => {
						draw_rectangle(x, y, tile_size, tile_size, SKYBLUE);
					}
					'x' => {
						let (u1, _u2, r1, d1, l1) = self.get_solid_tile_context(row_index, col_index);
						if !u1 && r1 && d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 1.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && r1 && !d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 1.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && r1 && d1 && !l1 { self.texture_rot(self.sprite_bg1, x, y, 0.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && !r1 && d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 2.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && r1 && !d1 && !l1 { self.texture_rot(self.sprite_bg1, x, y, 0.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && !r1 && !d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 2.0*32.0, 0.0*32.0, 0.0); }
						else if !u1 && !r1 && !d1 && !l1 { self.texture_rot(self.sprite_bg1, x, y, 5.0*32.0, 1.0*32.0, 0.0); }
						else if u1 && !r1 && !d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 5.0*32.0, 5.0*32.0, 0.0); }
						else if u1 && !r1 && !d1 && !l1 { self.texture_rot(self.sprite_bg1, x, y, 4.0*32.0, 0.0*32.0, 180.0); }
						else if u1 && !r1 && d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 5.0*32.0, 5.0*32.0, 0.0); }
						else if u1 && r1 && !d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 4.0*32.0, 5.0*32.0, 270.0); }
						else if u1 && r1 && d1 && l1 { self.texture_rot(self.sprite_bg1, x, y, 11.0*32.0, 1.0*32.0, 180.0); }
						else if u1 && r1 && d1 && !l1 { self.texture_rot(self.sprite_bg1, x, y, 4.0*32.0, 5.0*32.0, 0.0); }
						else { self.texture(self.sprite_bg1, x, y, 3.0*32.0, 0.0); }
					}
					'g' => {
						self.texture_rot(self.sprite_bg1, x, y, 10.0*32.0, 7.0*32.0, 0.0);
					}
					_ => {
						draw_rectangle(x, y, tile_size, tile_size, PINK);
					}
				}
			}
		}
	}

	fn get_solid_tile_context(&self, row: usize, col: usize) -> (bool, bool, bool, bool, bool) {

		let is_x = |r: isize, c: isize| -> bool {
			if r >= 0 && c >= 0 && (r as usize) < self.height && (c as usize) < self.width {
				self.tiles[r as usize][c as usize] == 'x'
			} else {
				true
			}
		};

		let u1 = is_x(row as isize - 1, col as isize);  // up
		let u2 = is_x(row as isize - 2, col as isize);  // up two
		let r1 = is_x(row as isize, col as isize + 1);  // right
		let l1 = is_x(row as isize, col as isize - 1);  // left
		let d1 = is_x(row as isize + 1, col as isize);  // down

		(u1, u2, r1, d1, l1)
	}

	fn texture(&self, image: Texture2D, x: f32, y: f32, pos_x: f32, pos_y: f32) {
		draw_texture_ex(
			image,
			x,
			y,
			WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(32.0, 32.0)),
				// If your sprite has a known tile size, specify it as the source rect.
				// For example, if it's 16x16:
				source: Some(Rect::new(pos_x, pos_y, 32.0, 32.0)),
				..Default::default()
			},
		);
	}

	fn texture_rot(&self, image: Texture2D, x: f32, y: f32, pos_x: f32, pos_y: f32, rotation_deg: f32) {
	draw_texture_ex(
		image,
		x,
		y,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(32.0, 32.0)),
			source: Some(Rect::new(pos_x, pos_y, 32.0, 32.0)),
			rotation: rotation_deg.to_radians(),
			pivot: None,
			..Default::default()
		},
	);
}

	// Returns `Some((t_enter, (ix, iy)))` if there's a valid intersection in front of `ray_origin`;
	// otherwise `None`.
	fn ray_box_intersection(
		ray_origin: (f32, f32),
		ray_dir: (f32, f32),
		box_min: (f32, f32),
		box_max: (f32, f32),
	) -> Option<(f32, (f32, f32))>
	{
		let (x0, y0) = ray_origin;
		let (dx, dy) = ray_dir;
		let (xmin, ymin) = box_min;
		let (xmax, ymax) = box_max;

		// Handle vertical direction = 0
		let (t_xmin, t_xmax) = if dx.abs() < f32::EPSILON {
			// Ray is vertical; check if within box in X
			if x0 < xmin || x0 > xmax {
				return None;
			}
			(-f32::INFINITY, f32::INFINITY)
		} else {
			let t1 = (xmin - x0) / dx;
			let t2 = (xmax - x0) / dx;
			(t1.min(t2), t1.max(t2))
		};

		// Handle horizontal direction = 0
		let (t_ymin, t_ymax) = if dy.abs() < f32::EPSILON {
			// Ray is horizontal; check if within box in Y
			if y0 < ymin || y0 > ymax {
				return None;
			}
			(-f32::INFINITY, f32::INFINITY)
		} else {
			let t3 = (ymin - y0) / dy;
			let t4 = (ymax - y0) / dy;
			(t3.min(t4), t3.max(t4))
		};

		// Parametric range of intersection
		let t_enter = t_xmin.max(t_ymin);
		let t_exit  = t_xmax.min(t_ymax);

		// We need t_exit >= t_enter and some portion >= 0 for a valid intersection ahead of origin
		if t_exit >= t_enter && t_exit >= 0.0 {
			let t_hit = t_enter.max(0.0);
			let ix = x0 + t_hit * dx;
			let iy = y0 + t_hit * dy;
			Some((t_hit, (ix, iy)))
		} else {
			None
		}
	}

	/*
	 * Returns the distance as a number between the point (x,y) and the closest solid in the direction (dir_x, dir_y)
	 * */
	pub fn raycast(&self, start: (f32, f32), dir: (f32, f32), distance: f32) -> Option<f32> {
		self.raycast_any(start, dir, distance, 'x').map(|(result, _, _)| result)
	}

	pub fn raycast_any(&self, start: (f32, f32), dir: (f32, f32), distance: f32, solid: char) -> Option<(f32, usize, usize)> {
		// How far we step forward each iteration.
		let step_size = 19.0_f32;

		let mut pos = start;
		let mut total_distance = 0.0_f32;

		loop {
			// Step forward along ray.
			pos.0 += dir.0 * step_size;
			pos.1 += dir.1 * step_size;
			total_distance += step_size;

			// If we crossed our maximum ray distance, give up.
			if total_distance > distance {
				break;
			}

			// Check if we hit a "solid" tile.
			if let Some((tile_pos_x, tile_pos_y, tile_x, tile_y)) = self.get_solid(pos.0, pos.1, solid) {
				// Assume (tile_pos_x, tile_pos_y) is the top-left corner of a 32x32 tile.
				// So if y is increasing downward, the bottom-right corner is (tile_pos_x + 32, tile_y + 32).
				let box_min = (tile_pos_x, tile_pos_y);
				let box_max = (tile_pos_x + 32.0, tile_pos_y + 32.0);

				// Check for intersection using our helper.
				if let Some((t_enter, (hit_x, hit_y))) = Map::ray_box_intersection(start, dir, box_min, box_max) {
					// We only care about an intersection "in front" of start; also check if it’s within
					// the distance we’ve traveled this loop (or you can check if t_enter <= distance, etc.)
					if t_enter >= 0.0 && t_enter <= total_distance {
						// Optionally draw or do whatever you want with (hit_x, hit_y).
						// Return the parametric distance t_enter, or the actual Euclidean distance from `start`.
						let dx = hit_x - start.0;
						let dy = hit_y - start.1;
						let intersection_dist = (dx*dx + dy*dy).sqrt();

						//draw_line(start.0, start.1, start.0 + 30.0, start.1, 2.0, GREEN);

						return Some((intersection_dist, tile_x, tile_y));
					}
				}
			}
		}

		None
	}

	pub fn get_solid(&self, x: f32, y: f32, solid: char) -> Option<(f32, f32, usize, usize)> {
		let tile_x = (x / 32.0).floor() as usize;
		let tile_y = (y / 32.0).floor() as usize;

		if tile_x >= self.width || tile_y >= self.height {
			return None;
		}


		match self.tiles[tile_y][tile_x] {
			tile if tile == solid   => {
				return Some(((tile_x as f32) * 32.0, (tile_y as f32) * 32.0, tile_x, tile_y))
			}
			_ => {
				return None;
			}
		};
	}

	pub fn remove_entity(&mut self, solid: char, tile_x :usize, tile_y :usize) {

		if let Some(row) = self.tiles.get_mut(tile_y) {
			if let Some(tile) = row.get_mut(tile_x) {
				if *tile == solid {
					*tile = ' ';
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_raycast_hits_wall() {
		let map = Map::from_file("maps/map1.txt");

		let start = (32.0, 32.0);
		let dir = (1.0, 0.0);
		let distance = 10.0;

		let result = map.raycast(start, dir, distance);

		assert_eq!(result, Some(64.0, _, _));
	}
}
