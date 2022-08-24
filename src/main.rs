extern crate allegro;

mod word;
mod semantics;
mod dictionary;
mod fractal;
mod word_slice;

use allegro::*;
use allegro_primitives::*;

const DISPLAY_WIDTH: i32 = 1200;
const DISPLAY_HEIGHT: i32 = 800;
const A: f32 = std::f32::consts::PI * (1.0 / 3.0); // 60.0 degrees for Koch Fractal
const S: f32 = 50.0;

// user defines an enum, specifiying operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum K {
    F,
    L,
    R,
}

// Enum for Levy-C-Curve
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Levy {
    F,
    L,
    R,
}

// Enum for Sierpinski-Teppich
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SierTepp {
    F,
    f,
    L,
    R,
}

// Generalize this to an arbitrary mutable Payload, if possible.
pub trait Operation: Copy {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32);
}

impl Operation for K {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            K::F => {
                current_pos.x += current_angle.cos();
                current_pos.y += current_angle.sin();
                vertex_buffer.push(*current_pos);
            },
            K::R => { *current_angle -= A; },
            K::L => { *current_angle += A; },
        }
    }
}

impl Operation for Levy {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Levy::F => {
                current_pos.x += current_angle.cos();
                current_pos.y += current_angle.sin();
                vertex_buffer.push(*current_pos);
            },
            Levy::R => { *current_angle -= 45.0f32.to_radians(); },
            Levy::L => { *current_angle += 45.0f32.to_radians(); },
        }
    }
}

impl Operation for SierTepp {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            SierTepp::F => {
                current_pos.x += current_angle.cos();
                current_pos.y += current_angle.sin();
                vertex_buffer.push(*current_pos);
            },
            SierTepp::f => {
                current_pos.x += current_angle.cos();
                current_pos.y += current_angle.sin();
                vertex_buffer.push(*current_pos);
            },
            SierTepp::R => { *current_angle -= 90.0f32.to_radians(); },
            SierTepp::L => { *current_angle += 90.0f32.to_radians(); },
        }
    }
}

pub trait Replacement: Sized {
    fn replacement(&self) -> Option<Vec<Self>>;
}

impl Replacement for K {
    fn replacement(&self) -> Option<Vec<Self>> {
        if let K::F = self {
            return Some(vec![K::F, K::L, K::F, K::R, K::R, K::F, K::L, K::F]);
        }
        return None;
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

pub fn iterate_operations<Op: Operation + Replacement>(operations: &[Op]) -> Vec<Op> {
    let mut iteration_result = vec![];

    for &op in operations {
        if let Some(mut replacement) = op.replacement() {
            iteration_result.append(&mut replacement);
        } else {
            iteration_result.push(op);
        }
    }
    return iteration_result;
}

pub fn iterate_fractal<Op: Operation + Replacement>(base_operations: &[Op], iteration_depth: usize) -> Vec<Vec<Op>> {
    let mut iteration_results: Vec<Vec<Op>> = vec![base_operations.to_owned()];

    for index in 1..=iteration_depth {
        let operations = iterate_operations(&iteration_results[index - 1]);
        iteration_results.push(operations);
    }

    return iteration_results;
}

pub fn iterated_vertices<Op: Operation + Replacement>(iterated_operations: &[Vec<Op>]) -> Vec<Vec<MathPosition>> {
    let mut iteration_results = vec![];
    // TODO: let mut staunching_factor = compute_staunching_factor();
    for (index, operations) in iterated_operations.iter().enumerate() {
        let mut vertices = compute_vertices(operations);
        for vertex in &mut vertices {
            vertex.scale(0.6f32.powi(index as i32));
        }
        iteration_results.push(vertices);
    }
    return iteration_results;
}

// Compute the center of a given set of vertices
pub fn compute_center(vertices: &[MathPosition]) -> MathPosition {
    let mut center = MathPosition::new();

    for vertex in vertices {
        center.x += vertex.x;
        center.y += vertex.y;
    }

    // We need -1 here since our vertices always include the first also as the last one
    center.scale(1.0 / ((vertices.len() - 1) as f32));
    return center;
}

// Apply center offset to all vertices in the given set.
pub fn apply_center_offset(vertices: &mut [MathPosition]) {
    let center = compute_center(vertices);

    for vertex in vertices {
        vertex.x -= center.x;
        vertex.y -= center.y;
    }
}

// TODO: Implement this function correctly, it does not work as expected at the moment.
pub fn compute_staunching_factor(replacement: &[K]) -> f32 {
    let vertices = compute_vertices(replacement);
    let last_vertex = vertices.last().unwrap();
    let norm_of_last_vertex = (last_vertex.x * last_vertex.x + last_vertex.y * last_vertex.y).sqrt();
    return 1.0 / norm_of_last_vertex;
}

// TODO: this leaves out the first point at 0.0.
// Not a problem if fractal loops around, as then last point will be equal to first
// Compute the corresponding vertices for a given set of operations
pub fn compute_vertices<Op: Operation>(operations: &[Op]) -> Vec<MathPosition> {
    let mut vertices = vec![];

    // Current pos and current angle should be abstracted in a Payload struct,
    // or something similar.
    // What exactly the payload will be depends on use case, so make this a marker trait
    let mut current_pos = MathPosition::new();
    let mut current_angle: f32 = 0.0;

    vertices.push(current_pos);
    for op in operations {
        op.apply(&mut vertices, &mut current_pos, &mut current_angle);
    }

    // Scale vertices
    for vertex in &mut vertices {
        vertex.scale(S);
    }

    apply_center_offset(&mut vertices);

    return vertices;
}

pub fn draw_polygon(primitives: &PrimitivesAddon, vertices: &[MathPosition], color: Color) {
    let vertices: Vec<_> = vertices.iter()
        .map(|pos| {
            let pos: ScreenPosition = pos.into();
            (pos.x, pos.y)
        })
        .collect();
    primitives.draw_polygon(&vertices, LineJoinType::Round, color, 2.0, 1.0);
}

pub fn draw_single_lines(primitives: &PrimitivesAddon, vertices: &[MathPosition], color: Color) {
    let vertices: Vec<_> = vertices.iter()
        .map(|pos| {
            let pos: ScreenPosition = pos.into();
            (pos.x, pos.y)
        })
        .collect();
    for index in 0..vertices.len()-1 {
        primitives.draw_line(vertices[index].0, vertices[index].1, vertices[index+1].0, vertices[index+1].1, color, 2.0);
    }
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

    //let base_operations_koch = vec![K::F, K::R, K::R, K::F, K::R, K::R, K::F];
    //let iterated_operations_koch = iterate_fractal(&base_operations_koch, 8);
    //let iterated_vertices_koch = iterated_vertices(&iterated_operations_koch[..]);

    //let base_operations_levy = vec![Levy::F];
    //let iterated_operations_levy = iterate_fractal(&base_operations_levy, 8);
    //let iterated_vertices_levy = iterated_vertices(&iterated_operations_levy[..]);

    let base_operations_sier_tepp = vec![SierTepp::F];
    let iterated_operations_sier_tepp = iterate_fractal(&base_operations_sier_tepp, 5);
    let iterated_vertices_sier_tepp = iterated_vertices(&iterated_operations_sier_tepp[..]);

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'exit: loop {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));

            //for (index, vertices) in iterated_vertices_koch.iter().enumerate() {
            //    let ex = index as i32;
            //    // draw_polygon(&primitives, &vertices, Color::from_rgb_f(0.9f32.powi(ex), 0.8f32.powi(ex), 0.7f32.powi(ex)));
            //    draw_single_lines(&primitives, &vertices, Color::from_rgb_f(0.9f32.powi(ex), 0.8f32.powi(ex), 0.7f32.powi(ex)));
            //}
            //
            //
            //for (index, vertices) in iterated_vertices_levy.iter().enumerate() {
            //    let ex = index as i32;
            //    draw_single_lines(&primitives, &vertices, Color::from_rgb_f(0.9f32.powi(ex), 0.8f32.powi(ex), 0.7f32.powi(ex)));
            //}

            for (index, vertices) in iterated_vertices_sier_tepp.iter().enumerate() {
                let ex = index as i32;
                draw_single_lines(&primitives, &vertices, Color::from_rgb_f(0.9f32.powi(ex), 0.8f32.powi(ex), 0.7f32.powi(ex)));
            }

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
