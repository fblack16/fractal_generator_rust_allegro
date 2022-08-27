extern crate allegro;

mod word;
mod semantics;
mod dictionary;
mod fractal;
mod word_slice;

mod coordinates;
mod common_fractals;

use allegro::*;
use allegro_primitives::*;

use coordinates::MathPosition;
use coordinates::ScreenPosition;

use common_fractals::*;

const DISPLAY_WIDTH: i32 = 1200;
const DISPLAY_HEIGHT: i32 = 800;
const S: f32 = 300.0;


// Generalize this to an arbitrary mutable Payload, if possible.
pub trait Operation: Copy {
    fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, current_pos: &mut MathPosition, current_angle: &mut f32);
    fn forward() -> Self;
}

pub trait Replacement: Sized {
    fn replacement(&self) -> Option<Vec<Self>>;
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

pub fn iterated_vertices<Op: Operation + Replacement>(iterated_operations: &[Vec<Op>]) -> Vec<Vec<Option<MathPosition>>> {
    let mut iteration_results = vec![];
    let staunching_factor = compute_staunching_factor::<Op>();

    for (index, operations) in iterated_operations.iter().enumerate() {
        let mut vertices = compute_scaled_vertices(operations);
        for vertex in &mut vertices {
            if let Some(vertex) = vertex {
                vertex.scale(staunching_factor.powi(index as i32));
            }
        }
        iteration_results.push(vertices);
    }
    return iteration_results;
}

// Compute the center of a given set of vertices
pub fn compute_center(vertices: &[Option<MathPosition>]) -> Option<MathPosition> {
    let mut center = MathPosition::new(0.0, 0.0);

    let mut n_vertices = 0;
    for entry in vertices {
        if let Some(vertex) = entry {
            center += *vertex;
            n_vertices += 1;
        }
    }

    // If we have a closed curve and the first index equals the last,
    // do not consider the redundant point in the computation of the center.
    // if (*vertices.first().unwrap() - *vertices.last().unwrap()).norm() <= 5.0 * f32::EPSILON {
    //     n_vertices = vertices.len() - 1;
    // }

    // Check for points that are redundant.
    // We need -1 here since our vertices always include the first also as the last one
    if n_vertices == 0 {
        return None;
    }
    center.scale(1.0 / (n_vertices as f32));
    return Some(center);
}

// Apply center offset to all vertices in the given set.
pub fn apply_center_offset(vertices: &mut [Option<MathPosition>]) {
    let center = compute_center(vertices);

    if let Some(center) = center {
        for vertex in vertices {
            if let Some(vertex) = vertex {
                *vertex -= center;
            }
        }
    }
}

// TODO: Implement this function correctly, it does not work as expected at the moment.
fn compute_staunching_factor<Op: Operation + Replacement>() -> f32 {
    let mut staunching_factor = 1.0;
    if let Some(replacement) = Op::forward().replacement() {
        let vertices = compute_base_vertices(&replacement);
        staunching_factor = 1.0 / vertices.last().unwrap().unwrap().norm();
    }
    return staunching_factor;
}

pub fn compute_base_vertices<Op: Operation>(operations: &[Op]) -> Vec<Option<MathPosition>> {
    let mut vertices = vec![];
    let mut current_pos = MathPosition::new(0.0, 0.0);
    let mut current_angle: f32 = 0.0;

    vertices.push(Some(current_pos));
    for op in operations {
        op.apply(&mut vertices, &mut current_pos, &mut current_angle);
    }

    return vertices;
}

// TODO: this leaves out the first point at 0.0.
// Not a problem if fractal loops around, as then last point will be equal to first
// Compute the corresponding vertices for a given set of operations
pub fn compute_scaled_vertices<Op: Operation>(operations: &[Op]) -> Vec<Option<MathPosition>> {
    let mut vertices = compute_base_vertices(operations);

    apply_center_offset(&mut vertices);
    for vertex in &mut vertices {
        if let Some(vertex) = vertex {
            vertex.scale(S);
        }
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

pub fn draw_single_lines(primitives: &PrimitivesAddon, vertices: &[Option<MathPosition>], color: Color) {
    let vertices: Vec<Option<(f32, f32)>> = vertices.iter()
        .map(|pos| {
            match pos {
                Some(pos) => Some(ScreenPosition::from(pos).into()),
                None => None,
            }
        })
        .collect();
    for index in 0..vertices.len()-1 {
        if let Some(vertex) = vertices[index] {
            if let Some(next_vertex) = vertices[index+1] {
                primitives.draw_line(vertex.0, vertex.1, next_vertex.0, next_vertex.1, color, 2.0);
            }
        }
    }
}

allegro_main!
{
    let core = Core::init().unwrap();
    if let Ok(_) = core.install_keyboard() {
        println!("Keyboard successfully installed!");
    } else {
        println!("Keyboard could not be installed!");
    }
    let primitives = PrimitivesAddon::init(&core).unwrap();

    let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let queue = EventQueue::new(&core).unwrap();

    let mut current_depth = 0;

    // let base_operations = vec![Koch::F, Koch::R, Koch::R, Koch::F, Koch::R, Koch::R, Koch::F];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![Levy::F];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![SierTepp::F];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    let base_operations = vec![DragonCurve::F];
    let iterated_operations = iterate_fractal(&base_operations, 20);
    let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut redraw = true;
    timer.start();

    'exit: loop {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));

            //for (index, vertices) in vertex_iterations.iter().enumerate() {
            //    let ex = index as i32;
            //    draw_single_lines(&primitives, &vertices, Color::from_rgb_f(0.9f32.powi(ex), 0.8f32.powi(ex), 0.7f32.powi(ex)));
            //}

            draw_single_lines(&primitives, &vertex_iterations[current_depth], Color::from_rgb_f(0.9, 0.8, 0.7));

            core.flip_display();
            redraw = false;
        }

        match queue.wait_for_event()
        {
            DisplayClose{..} => break 'exit,
            TimerTick{..} => redraw = true,
            //KeyDown{source, timestamp, keycode, display} if keycode == KeyCode::F => {
            //    if current_word < starting_words.len()-1 {
            //        current_word += 1;
            //    }
            //    base_operations = starting_words[current_word];
            //    iterated_operations = iterate_fractal(&base_operations, 10);
            //    vertex_iterations = iterated_vertices(&iterated_operations[..]);
            //},
            //KeyDown{source, timestamp, keycode, display} if keycode == KeyCode::N => {
            //    if current_word > 0 {
            //        current_word -= 1;
            //    }
            //    base_operations = starting_words[current_word];
            //    iterated_operations = iterate_fractal(&base_operations, 10);
            //    vertex_iterations = iterated_vertices(&iterated_operations[..]);
            //},
            KeyDown{source: _, timestamp: _, keycode, ..} => {
                match keycode {
                    KeyCode::I => {
                        println!("Key: I");
                        if current_depth < 20 {
                            current_depth += 1;
                        }
                    },
                    KeyCode::P => {
                        println!("Key: P");
                        if current_depth > 0 {
                            current_depth -= 1;
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
