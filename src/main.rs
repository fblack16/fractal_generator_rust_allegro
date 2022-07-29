extern crate allegro;

mod word;
mod semantics;
mod dictionary;
mod fractal;
mod word_slice;

use allegro::*;
use allegro_primitives::*;

const DISPLAY_WIDTH: i32 = 800;
const DISPLAY_HEIGHT: i32 = 600;
const A: f32 = std::f32::consts::PI * (1.0 / 3.0); // 60.0 degrees for Koch Fractal
const S: f32 = 50.0;

pub enum K {
    F,
    L,
    R,
}

pub fn compute_vertices(operations: &[K]) -> Vec<MathPosition> {
    let mut vertices = vec![];

    let mut current_pos = MathPosition::new();
    let mut current_angle: f32 = 0.0;
    for op in operations {
        match op {
            K::F => {
                current_pos.x += current_angle.cos();
                current_pos.y += current_angle.sin();
                vertices.push(current_pos);
            },
            K::R => { current_angle -= A; },
            K::L => { current_angle += A; },
        }
    }

    // Scale vertices
    for vertex in &mut vertices {
        vertex.scale(S);
    }

    return vertices;
}

pub fn draw_polygon(primitives: &PrimitivesAddon, vertices: &[MathPosition]) {
    let vertices: Vec<_> = vertices.iter()
        .map(|pos| {
            let pos: ScreenPosition = pos.into();
            (pos.x, pos.y)
        })
        .collect();
    primitives.draw_polygon(&vertices, LineJoinType::Round, Color::from_rgb_f(1.0, 0.0, 0.0), 2.0, 1.0);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MathPosition {
    x: f32,
    y: f32,
}

impl MathPosition {
    pub fn new() -> Self {
        MathPosition {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
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
            y: pos.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScreenPosition {
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

allegro_main!
{
    let core = Core::init().unwrap();
    let primitives = PrimitivesAddon::init(&core).unwrap();

    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let queue = EventQueue::new(&core).unwrap();

    let operations = vec![K::F, K::R, K::R, K::F, K::R, K::R, K::F];
    let vertices = compute_vertices(&operations);

    //let triangle_vertices_screen = [
    //    ScreenPosition::from(MathPosition { x: 0.0, y: 0.0 }),
    //    ScreenPosition::from(MathPosition { x: 20.0, y: 0.0 }),
    //    ScreenPosition::from(MathPosition { x: 0.0, y: -20.0 }),
    //];

    //let triangle_vertices_math = [
    //    MathPosition::from(ScreenPosition { x: DISPLAY_WIDTH as f32 / 2.0, y: DISPLAY_HEIGHT as f32 / 2.0 }),
    //    MathPosition::from(ScreenPosition { x: DISPLAY_WIDTH as f32 / 2.0 + 20.0, y: DISPLAY_HEIGHT as f32 / 2.0 }),
    //    MathPosition::from(ScreenPosition { x: DISPLAY_WIDTH as f32 / 2.0, y: DISPLAY_HEIGHT as f32 / 2.0 - 20.0 }),
    //];

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'exit: loop {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));
            //primitives.draw_triangle(0.0, 0.0, 20.0, 0.0, 0.0, 20.0, Color::from_rgb_f(1.0, 0.0, 0.0), 2.0);

            //primitives.draw_triangle(
            //    triangle_vertices_screen[0].x, triangle_vertices_screen[0].y,
            //    triangle_vertices_screen[1].x, triangle_vertices_screen[1].y,
            //    triangle_vertices_screen[2].x, triangle_vertices_screen[2].y,
            //    Color::from_rgb_f(0.0, 0.0, 1.0),
            //    2.0
            //);

            //primitives.draw_triangle(
            //    triangle_vertices_math[0].x, triangle_vertices_math[0].y,
            //    triangle_vertices_math[1].x, triangle_vertices_math[1].y,
            //    triangle_vertices_math[2].x, triangle_vertices_math[2].y,
            //    Color::from_rgb_f(0.0, 1.0, 0.0),
            //    2.0
            //);

            draw_polygon(&primitives, &vertices);
            core.flip_display();
            redraw = false;
        }

        match queue.wait_for_event()
        {
            DisplayClose{..} => break 'exit,
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}
