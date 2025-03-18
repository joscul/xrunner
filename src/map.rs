
pub struct Map {
    pub tiles: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Map {

    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        let height = tiles.len();
        let width = if (height > 0) {
            tiles[0].len()
        } else {
            0
        };

        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn from_file(path: &str) -> Self {
        let content = std::fs::read_to_string(path).unwrap();

        let tiles: Vec<Vec<char>> = content
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let height = tiles.len();
        let width = if (height > 0) {
            tiles[0].len()
        } else {
            0
        };

        Map {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tiles(&self, row: usize, col: usize) -> Option<char> {
        if (row < self.height && col < self.height) {
            Some(self.tiles[row][col])
        } else {
            None
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
                    'x' => {
                        draw_rectangle(x, y, tile_size, tile_size, DARKGRAY);
                    }
                    ' ' => {
                        draw_rectangle(x, y, tile_size, tile_size, SKYBLUE);
                    }
                    _ => {
                        draw_rectangle(x, y, tile_size, tile_size, PINK);
                    }
                }
            }
        }
    }

    pub fn is_solid(&self, x: f32, y: f32) -> bool {
        let tile_x = (x / 32.0).floor() as usize;
        let tile_y = (y / 32.0).floor() as usize;

        if (tile_x >= self.width || tile_y >= self.height) {
            return true;
        }
        match self.tiles[tile_y][tile_x] {
            ' ' => false,
            _   => true,
        }
    }
}
