use crate::LindenmayerPayload;
use crate::Operation;
use crate::Replacement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Koch {
    F,
    L,
    R,
}

impl Operation for Koch {
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::R => { payload.decrease_current_angle(60.0f32.to_radians()); },
            Self::L => { payload.increase_current_angle(60.0f32.to_radians()); },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for Koch {
    fn replacement(&self) -> Option<Vec<Self>> {
        if let Self::F = self {
            return Some(vec![Self::F, Self::L, Self::F, Self::R, Self::R, Self::F, Self::L, Self::F]);
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::R => { payload.decrease_current_angle(45.0f32.to_radians()); },
            Self::L => { payload.increase_current_angle(45.0f32.to_radians()); },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for Levy {
    fn replacement(&self) -> Option<Vec<Self>> {
        if let Levy::F = self {
            return Some(vec![Self::L, Self::F, Self::R, Self::R, Self::F, Self::L]);
        }
        return None;
    }
}

// Enum for Sierpinski-Teppich
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SierTepp {
    F,
    J,
    L,
    R,
}

impl Operation for SierTepp {
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::J => {
                // Update the position, but do not draw.
                payload.update_current_position();
                // To mark that we do not need to draw, put a none in the vertex buffer as a stop
                // sign, and then push the new point.
                if let Some(_) = payload.vertex_buffer.last() {
                    payload.vertex_buffer.push(None);
                }
                payload.push_current_position();
            },
            Self::R => { payload.decrease_current_angle(90.0f32.to_radians()); },
            Self::L => { payload.increase_current_angle(90.0f32.to_radians()); },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for SierTepp {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                return Some(vec![Self::F, Self::L, Self::F, Self::R, Self::F, Self::R, Self::F, Self::F, Self::R, Self::F, Self::R, Self::F, Self::R, Self::J, Self::F]);
            },
            Self::J => {
                return Some(vec![Self::J, Self::J, Self::J]);
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::W => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(45.0f32);
            },
            Self::R => {
                payload.decrease_current_angle(45.0f32);
            },
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for DragonCurve {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::F => {
                Some(vec![Self::L, Self::F, Self::R, Self::R, Self::W, Self::L])
            },
            Self::W => {
                Some(vec![Self::R, Self::F, Self::L, Self::L, Self::W, Self::R])
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::W => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(60.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(60.0f32.to_radians());
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(90.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(90.0f32.to_radians());
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
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(36.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(36.0f32.to_radians());
            },
            Self::T => {
                payload.increase_current_angle(180.0f32.to_radians());
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::W => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(60.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(60.0f32.to_radians());
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
    J,
    L,
    R,
}

impl Operation for SierpinskiTriangle {
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::J => {
                payload.update_current_position();
                if let Some(_) = payload.vertex_buffer.last() {
                    payload.vertex_buffer.push(None);
                }
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(60.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(60.0f32.to_radians());
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
                Some(vec![Self::F, Self::R, Self::R, Self::F, Self::R, Self::R, Self::F, Self::R, Self::R, Self::J, Self::J])
            },
            Self::J => {
                Some(vec![Self::J, Self::J])
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(25.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(25.0f32.to_radians());
            },
            Self::PUSH => {
                payload.save_current_position_and_angle();
            },
            Self::POP => {
                payload.pop_and_restore_current_position_and_angle();
                payload.vertex_buffer.push(None);
                payload.push_current_position();
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(25.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(25.0f32.to_radians());
            },
            Self::PUSH => {
                payload.save_current_position_and_angle();
            },
            Self::POP => {
                payload.pop_and_restore_current_position_and_angle();
                payload.vertex_buffer.push(None);
                payload.push_current_position();
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
                Some(vec![Self::F, Self::L, Self::PUSH, Self::L, Self::PUSH, Self::R, Self::R, Self::R, Self::PUSH, Self::X, Self::POP, Self::X, Self::POP, Self::X, Self::POP, Self::F, Self::X])
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
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(25.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(25.0f32.to_radians());
            },
            Self::PUSH => {
                payload.save_current_position_and_angle();
            },
            Self::POP => {
                payload.pop_and_restore_current_position_and_angle();
                payload.vertex_buffer.push(None);
                payload.push_current_position();
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
                Some(vec![Self::F, Self::R, Self::PUSH, Self::PUSH, Self::Y, Self::POP, Self::L, Self::L, Self::X, Self::POP, Self::F, Self::Y])
            },
            Self::Y => {
                Some(vec![Self::F, Self::L, Self::PUSH, Self::PUSH, Self::X, Self::POP, Self::R, Self::R, Self::Y, Self::POP, Self::F, Self::Y])
            }
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantThree {
    F,
    X,
    Y,
    L,
    R,
    PUSH,
    POP,
}

impl Operation for PlantThree {
    fn apply(&self, payload: &mut LindenmayerPayload) {
        match self {
            Self::F => {
                payload.update_current_position();
                payload.push_current_position();
            },
            Self::L => {
                payload.increase_current_angle(20.0f32.to_radians());
            },
            Self::R => {
                payload.decrease_current_angle(20.0f32.to_radians());
            },
            Self::PUSH => {
                payload.save_current_position_and_angle();
            },
            Self::POP => {
                payload.pop_and_restore_current_position_and_angle();
                payload.vertex_buffer.push(None);
                payload.push_current_position();
            },
            _ => (),
        }
    }
    fn forward() -> Self {
        Self::F
    }
}

impl Replacement for PlantThree {
    fn replacement(&self) -> Option<Vec<Self>> {
        match self {
            Self::X => {
                Some(vec![Self::F, Self::L, Self::PUSH, Self::X, Self::POP, Self::R, Self::F, Self::R, Self::R, Self::PUSH, Self::X, Self::POP, Self::L, Self::L, Self::X])
            },
            Self::F => {
                Some(vec![Self::F, Self::F])
            }
            _ => None,
        }
    }
}
