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
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
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
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
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
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
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
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GosperCurve {
    F,
    W,
    L,
    R,
}

impl Operation for GosperCurve {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::W => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 60.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 60.0f32.to_radians();
            }
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for GosperCurve {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                Some(vec![Self::F, Self::L, Self::W, Self::L, Self::L, Self::W, Self::R, Self::F, Self::R, Self::R, Self::F, Self::F, Self::R, Self::W, Self::L])
            },
            Self::W => {
                Some(vec![Self::R, Self::F, Self::L, Self::W, Self::W, Self::L, Self::L, Self::W, Self::L, Self::F, Self::R, Self::R, Self::F, Self::R, Self::W])
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HilbertCurve {
    F,
    A,
    B,
    L,
    R,
}

impl Operation for HilbertCurve {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 90.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 90.0f32.to_radians();
            },
            _ => (),
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for HilbertCurve {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::A => {
                Some(vec![Self::L, Self::B, Self::F, Self::R, Self::A, Self::F, Self::A, Self::R, Self::F, Self::B, Self::L])
            },
            Self::B => {
                Some(vec![Self::R, Self::A, Self::F, Self::L, Self::B, Self::F, Self::B, Self::L, Self::F, Self::A, Self::R])
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PentaPlexity {
    F,
    L,
    R,
    T,
}

impl Operation for PentaPlexity {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 36.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 36.0f32.to_radians();
            },
            Self::T => {
                *current_angle += 180.0f32.to_radians();
            },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for PentaPlexity {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                Some(vec![Self::F, Self::L, Self::L, Self::F, Self::L, Self::L, Self::F, Self::T, Self::F, Self::R, Self::F, Self::L, Self::L, Self::F])
            },
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrowHead {
    F,
    W,
    L,
    R,
}

impl Operation for ArrowHead {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::W => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 60.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 60.0f32.to_radians();
            },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for ArrowHead {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                Some(vec![Self::R, Self::W, Self::L, Self::F, Self::L, Self::W, Self::R])
            },
            Self::W => {
                Some(vec![Self::L, Self::F, Self::R, Self::W, Self::R, Self::F, Self::L])
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SierpinskiTriangle {
    F,
    f,
    L,
    R,
}

impl Operation for SierpinskiTriangle {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::f => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                if !vertex_buffer.is_empty() {
                    if let Some(_) = vertex_buffer.last() {
                        vertex_buffer.push(None);
                    }
                }
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 60.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 60.0f32.to_radians();
            },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for SierpinskiTriangle {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                Some(vec![Self::F, Self::R, Self::R, Self::F, Self::R, Self::R, Self::F, Self::R, Self::R, Self::f, Self::f])
            },
            Self::f => {
                Some(vec![Self::f, Self::f])
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FirstPlant {
    F,
    X,
    L,
    R,
    PUSH,
    POP,
}

impl Operation for FirstPlant {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 25.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 25.0f32.to_radians();
            },
            Self::PUSH => {
                coordinate_buffer.push((*current_pos, *current_angle));
            },
            Self::POP => {
                if let Some((stored_position, stored_angle)) = coordinate_buffer.pop() {
                    *current_angle = stored_angle;
                    *current_pos = stored_position;
                }
                vertex_buffer.push(None);
                vertex_buffer.push(Some(*current_pos));
            },
            _ => (),
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for FirstPlant {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::X => {
                Some(vec![FirstPlant::F, FirstPlant::L, FirstPlant::PUSH, FirstPlant::PUSH, FirstPlant::X, FirstPlant::POP, FirstPlant::R, FirstPlant::X, FirstPlant::POP, FirstPlant::R, FirstPlant::F, FirstPlant::PUSH, FirstPlant::R, FirstPlant::F, FirstPlant::X, FirstPlant::POP, FirstPlant::L, FirstPlant::X])
            },
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantOne {
    F,
    X,
    L,
    R,
    PUSH,
    POP,
}

impl Operation for PlantOne {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 25.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 25.0f32.to_radians();
            },
            Self::PUSH => {
                coordinate_buffer.push((*current_pos, *current_angle));
            },
            Self::POP => {
                if let Some((stored_position, stored_angle)) = coordinate_buffer.pop() {
                    *current_angle = stored_angle;
                    *current_pos = stored_position;
                }
                vertex_buffer.push(None);
                vertex_buffer.push(Some(*current_pos));
            },
            _ => (),
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for PlantOne {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::X => {
                Some(vec![Self::F, Self::L, Self::PUSH, Self::PUSH, Self::X, Self::POP, Self::R, Self::R, Self::X, Self::POP, Self::F, Self::X])
            },
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantTwo {
    F,
    X,
    Y,
    L,
    R,
    PUSH,
    POP,
}

impl Operation for PlantTwo {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Self::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(Some(*current_pos));
            },
            Self::L => {
                *current_angle += 25.0f32.to_radians();
            },
            Self::R => {
                *current_angle -= 25.0f32.to_radians();
            },
            Self::PUSH => {
                coordinate_buffer.push((*current_pos, *current_angle));
            },
            Self::POP => {
                if let Some((stored_position, stored_angle)) = coordinate_buffer.pop() {
                    *current_angle = stored_angle;
                    *current_pos = stored_position;
                }
                vertex_buffer.push(None);
                vertex_buffer.push(Some(*current_pos));
            },
            _ => (),
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for PlantTwo {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::X => {
                Some(vec![Self::F, Self::PUSH, Self::R, Self::R, Self::R, Self::PUSH, Self::X, Self::POP, Self::X, Self::POP, Self::F, Self::R, Self::X])
            },
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
            _ => None,
        }
    }
}
