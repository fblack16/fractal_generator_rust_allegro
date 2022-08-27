use std::ops::{Add, Sub, AddAssign, SubAssign};
use crate::DISPLAY_HEIGHT;
use crate::DISPLAY_WIDTH;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MathPosition {
    pub x: f32,
    pub y: f32,
}

impl MathPosition {
    pub fn new(x: f32, y: f32) -> Self {
        MathPosition {
            x,
            y,
        }
    }

    pub fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
}

impl From<(f32, f32)> for MathPosition {
    fn from(pos: (f32, f32)) -> Self {
        MathPosition {
            x: pos.0,
            y: pos.1,
        }
    }
}

impl From<&(f32, f32)> for MathPosition {
    fn from(pos: &(f32, f32)) -> Self {
        MathPosition {
            x: pos.0,
            y: pos.1,
        }
    }
}

impl Add for MathPosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for MathPosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for MathPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for MathPosition {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPosition {
    x: f32,
    y: f32,
}

impl From<MathPosition> for ScreenPosition {
    fn from(math_pos: MathPosition) -> Self {
        ScreenPosition {
            x: math_pos.x + (DISPLAY_WIDTH as f32) / 2.0,
            y: -math_pos.y + (DISPLAY_HEIGHT as f32) / 2.0,
        }
    }
}

impl From<&MathPosition> for ScreenPosition {
    fn from(math_pos: &MathPosition) -> Self {
        ScreenPosition {
            x: math_pos.x + (DISPLAY_WIDTH as f32) / 2.0,
            y: -math_pos.y + (DISPLAY_HEIGHT as f32) / 2.0,
        }
    }
}

impl From<(f32, f32)> for ScreenPosition {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Into<(f32, f32)> for ScreenPosition {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl From<ScreenPosition> for MathPosition {
    fn from(screen_pos: ScreenPosition) -> Self {
        MathPosition {
            x: screen_pos.x - (DISPLAY_WIDTH as f32) / 2.0,
            y: -screen_pos.y + (DISPLAY_HEIGHT as f32) / 2.0,
        }
    }
}

impl From<&ScreenPosition> for MathPosition {
    fn from(screen_pos: &ScreenPosition) -> Self {
        MathPosition {
            x: screen_pos.x - (DISPLAY_WIDTH as f32) / 2.0,
            y: -screen_pos.y + (DISPLAY_HEIGHT as f32) / 2.0,
        }
    }
}

impl Into<(f32, f32)> for MathPosition {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
