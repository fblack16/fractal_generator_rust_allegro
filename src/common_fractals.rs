use crate::Operation;
use crate::Replacement;

use crate::coordinates::MathPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Koch {
    F,
    L,
    R,
}

impl Operation for Koch {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Koch::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Koch::R => { *current_angle -= 60.0f32.to_radians(); },
            Koch::L => { *current_angle += 60.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        Koch::F
    }
}

impl Replacement for Koch {
    fn replacement(&self) -> Option<Vec<Self>> {
        if let Koch::F = self {
            return Some(vec![Koch::F, Koch::L, Koch::F, Koch::R, Koch::R, Koch::F, Koch::L, Koch::F]);
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Levy {
    F,
    L,
    R,
}

impl Operation for Levy {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Levy::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Levy::R => { *current_angle -= 45.0f32.to_radians(); },
            Levy::L => { *current_angle += 45.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        Levy::F
    }
}

impl Replacement for Levy {
    fn replacement(&self) -> Option<Vec<Self>> {
        if let Levy::F = self {
            return Some(vec![Levy::L, Levy::F, Levy::R, Levy::R, Levy::F, Levy::L]);
        }
        return None;
    }
}

// Enum for Sierpinski-Teppich
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SierTepp {
    F,
    f,
    L,
    R,
}

impl Operation for SierTepp {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            SierTepp::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            SierTepp::f => {
                // Update the position, but do not draw.
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                // To mark that we do not need to draw, put a none in the vertex buffer as a stop
                // sign, and then push the new point.
                if !vertex_buffer.is_empty() {
                    if let Some(_) = vertex_buffer.last() {
                        vertex_buffer.push(None);
                    }
                }
                vertex_buffer.push(Some(*current_pos))
            },
            SierTepp::R => { *current_angle -= 90.0f32.to_radians(); },
            SierTepp::L => { *current_angle += 90.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        SierTepp::F
    }
}

impl Replacement for SierTepp {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            SierTepp::F => {
                return Some(vec![SierTepp::F, SierTepp::L, SierTepp::F, SierTepp::R, SierTepp::F, SierTepp::R, SierTepp::F, SierTepp::F, SierTepp::R, SierTepp::F, SierTepp::R, SierTepp::F, SierTepp::R, SierTepp::f, SierTepp::F]);
            },
            SierTepp::f => {
                return Some(vec![SierTepp::f, SierTepp::f, SierTepp::f]);
            },
            _ => return None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragonCurve {
    F,
    W,
    L,
    R,
}

impl Operation for DragonCurve {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            DragonCurve::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            DragonCurve::W => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            DragonCurve::L => {
                *current_angle += 45.0f32.to_radians();
            },
            DragonCurve::R => {
                *current_angle -= 45.0f32.to_radians();
            },
        }
    }
    fn forward() -> Self {
        DragonCurve::F
    }
}

impl Replacement for DragonCurve {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            DragonCurve::F => {
                Some(vec![DragonCurve::L, DragonCurve::F, DragonCurve::R, DragonCurve::R, DragonCurve::W, DragonCurve::L])
            },
            DragonCurve::W => {
                Some(vec![DragonCurve::R, DragonCurve::F, DragonCurve::L, DragonCurve::L, DragonCurve::W, DragonCurve::R])
            },
            _ => None
        }
    }
}
