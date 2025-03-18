use crate::map::Map;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x, y, vx: 0.0, vy: 0.0,
        }
    }

    pub fn update(&mut self, map: &Map) {

        // player is falling more and more as default. This is gravity.
        self.vy += 0.1;
        if map.is_solid(self.left(), self.bottom() + self.vy) {
            self.vy = 0.0;
        }

        // Unless it stands on something.
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

        if (self.vy > 10.0) {
            self.vy = 10.0;
        }

        self.x += self.vx;
        self.y += self.vy;

        // reset
        self.vx = 0.0;
    }

    pub fn jump(&mut self, map: &Map) {
        // can only jump if standing on a solid
        if map.is_solid(self.center(), self.bottom() + Player::delta()) {
            self.vy -= 7.0;
        }
    }

    pub fn move_right(&mut self) {
        self.vx = 3.0;
    }

    pub fn move_left(&mut self) {
        self.vx = -3.0;
    }

    pub fn center(&self) -> f32 {
        return self.x + 16.0;
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


}
