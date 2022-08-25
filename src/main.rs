extern crate allegro;

mod word;
mod semantics;
mod dictionary;
mod fractal;
mod word_slice;

mod coordinates;

use allegro::*;
use allegro_primitives::*;

use coordinates::MathPosition;
use coordinates::ScreenPosition;

const DISPLAY_WIDTH: i32 = 1200;
const DISPLAY_HEIGHT: i32 = 800;
const S: f32 = 700.0;

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
    fn forward() -> Self;
}

impl Operation for K {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            K::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(*current_pos);
            },
            K::R => { *current_angle -= 60.0f32.to_radians(); },
            K::L => { *current_angle += 60.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        K::F
    }
}

impl Operation for Levy {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            Levy::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(*current_pos);
            },
            Levy::R => { *current_angle -= 45.0f32.to_radians(); },
            Levy::L => { *current_angle += 45.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        Levy::F
    }
}

impl Operation for SierTepp {
    fn apply(&self, vertex_buffer: &mut Vec<MathPosition>, current_pos: &mut MathPosition, current_angle: &mut f32) {
        match self {
            SierTepp::F => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(*current_pos);
            },
            SierTepp::f => {
                *current_pos += MathPosition::new(current_angle.cos(), current_angle.sin());
                vertex_buffer.push(*current_pos);
            },
            SierTepp::R => { *current_angle -= 90.0f32.to_radians(); },
            SierTepp::L => { *current_angle += 90.0f32.to_radians(); },
        }
    }
    fn forward() -> Self {
        SierTepp::F
    }
}


pub trait Replacement: Sized {
    fn replacement(&self) -> Option<Vec<Self>>;}

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
    let staunching_factor = compute_staunching_factor::<Op>();

    for (index, operations) in iterated_operations.iter().enumerate() {
        let mut vertices = compute_scaled_vertices(operations);
        for vertex in &mut vertices {
            vertex.scale(staunching_factor.powi(index as i32));
        }
        iteration_results.push(vertices);
    }
    return iteration_results;
}

// Compute the center of a given set of vertices
pub fn compute_center(vertices: &[MathPosition]) -> MathPosition {
    let mut center = MathPosition::new(0.0, 0.0);

    for vertex in vertices {
        center += *vertex;
    }

    let mut n_vertices = vertices.len();
    // If we have a closed curve and the first index equals the last,
    // do not consider the redundant point in the computation of the center.
    if (*vertices.first().unwrap() - *vertices.last().unwrap()).norm() <= 5.0 * f32::EPSILON {
        n_vertices = vertices.len() - 1;
    }

    // Check for points that are redundant.
    // We need -1 here since our vertices always include the first also as the last one
    center.scale(1.0 / (n_vertices as f32));
    return center;
}

// Apply center offset to all vertices in the given set.
pub fn apply_center_offset(vertices: &mut [MathPosition]) {
    let center = compute_center(vertices);

    for vertex in vertices {
        *vertex -= center;
    }
}

// TODO: Implement this function correctly, it does not work as expected at the moment.
fn compute_staunching_factor<Op: Operation + Replacement>() -> f32 {
    let mut staunching_factor = 1.0;
    if let Some(replacement) = Op::forward().replacement() {
        let vertices = compute_base_vertices(&replacement);
        staunching_factor = 1.0 / vertices.last().unwrap().norm();
    }
    return staunching_factor;
}

pub fn compute_base_vertices<Op: Operation>(operations: &[Op]) -> Vec<MathPosition> {
    let mut vertices = vec![];
    let mut current_pos = MathPosition::new(0.0, 0.0);
    let mut current_angle: f32 = 0.0;

    vertices.push(current_pos);
    for op in operations {
        op.apply(&mut vertices, &mut current_pos, &mut current_angle);
    }

    return vertices;
}

// TODO: this leaves out the first point at 0.0.
// Not a problem if fractal loops around, as then last point will be equal to first
// Compute the corresponding vertices for a given set of operations
pub fn compute_scaled_vertices<Op: Operation>(operations: &[Op]) -> Vec<MathPosition> {
    let mut vertices = compute_base_vertices(operations);

    apply_center_offset(&mut vertices);
    for vertex in &mut vertices {
        vertex.scale(S);
    }

    return vertices;
}

pub fn draw_polygon(primitives: &PrimitivesAddon, vertices: &[MathPosition], color: Color) {
    let vertices: Vec<(f32, f32)> = vertices.iter()
        .map(|pos| {
            ScreenPosition::from(pos).into()
        })
        .collect();
    primitives.draw_polygon(&vertices, LineJoinType::Round, color, 2.0, 1.0);
}

pub fn draw_single_lines(primitives: &PrimitivesAddon, vertices: &[MathPosition], color: Color) {
    let vertices: Vec<(f32, f32)> = vertices.iter()
        .map(|pos| {
            ScreenPosition::from(pos).into()
        })
        .collect();
    for index in 0..vertices.len()-1 {
        primitives.draw_line(vertices[index].0, vertices[index].1, vertices[index+1].0, vertices[index+1].1, color, 2.0);
    }
}

allegro_main!
{
    let core = Core::init().unwrap();
    let primitives = PrimitivesAddon::init(&core).unwrap();

    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let queue = EventQueue::new(&core).unwrap();

    // let base_operations = vec![K::F, K::R, K::R, K::F, K::R, K::R, K::F];
    // let iterated_operations = iterate_fractal(&base_operations, 8);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![Levy::F];
    // let iterated_operations = iterate_fractal(&base_operations, 8);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    let base_operations = vec![SierTepp::F];
    let iterated_operations = iterate_fractal(&base_operations, 5);
    let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'exit: loop {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));

            for (index, vertices) in vertex_iterations.iter().enumerate() {
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
