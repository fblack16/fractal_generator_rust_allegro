extern crate allegro;

mod word;
//mod semantics;
//mod dictionary;
//mod fractal;
//mod word_slice;

mod coordinates;
//mod common_fractals;
//mod tryout;
//mod letter;

use allegro::*;
use allegro_primitives::*;

use coordinates::MathPosition;
use coordinates::ScreenPosition;

mod check_this;
use check_this::*;

//use common_fractals::*;
//use tryout::*;

const DISPLAY_WIDTH: i32 = 1900;
const DISPLAY_HEIGHT: i32 = 1080;
const S: f32 = 300.0;

//pub struct LindenmayerFractal<Op: Operation + Replacement> {
//    starting_word: Vec<Op>,
//    word_buffer: Vec<Vec<Op>>,
//    iteration_buffer: Vec<Vec<Option<MathPosition>>>,
//    payload: LindenmayerPayload,
//}
//
//impl<Op: Operation + Replacement> LindenmayerFractal<Op> {
//    pub fn new(starting_word: &[Op]) -> Self {
//        let starting_word = starting_word.to_owned();
//        let mut payload = LindenmayerPayload::new();
//        payload.compute_vertices(&starting_word, S);
//        let iteration_buffer = vec![payload.vertex_buffer.clone()];
//        let word_buffer = vec![starting_word.clone()];
//        Self {
//            starting_word,
//            word_buffer,
//            iteration_buffer,
//            payload,
//        }
//    }
//    pub fn next_operations(&mut self) {
//        let mut iteration_result = vec![];
//        if let Some(word) = self.word_buffer.last() {
//            for &op in word {
//                if let Some(mut replacement) = op.replacement() {
//                    iteration_result.append(&mut replacement);
//                } else {
//                    iteration_result.push(op);
//                }
//            }
//        }
//        self.word_buffer.push(iteration_result);
//    }
//    pub fn next_iteration(&mut self) {
//        let staunching_factor = Self::compute_staunching_factor();
//        let depth = self.word_buffer.len() - 1;
//
//        if let Some(word) = self.word_buffer.last() {
//            self.payload.compute_vertices(word, S);
//            for entry in &mut self.payload.vertex_buffer {
//                if let Some(vertex) = entry {
//                    vertex.scale(staunching_factor.powi(depth as i32));
//                }
//            }
//            self.iteration_buffer.push(self.payload.vertex_buffer.clone());
//        }
//    }
//    pub fn iterate(&mut self, depth: usize) {
//        if depth >= self.iteration_buffer.len() {
//            for _ in 1..=(depth - self.iteration_buffer.len() + 1) {
//                self.next_operations();
//                self.next_iteration();
//            }
//        }
//    }
//    fn compute_staunching_factor() -> f32 {
//        let mut payload = LindenmayerPayload::new();
//        let mut staunching_factor = 1.0;
//        if let Some(replacement) = Op::forward().replacement() {
//            payload.compute_base_vertices(&replacement);
//            staunching_factor = 1.0 / payload.vertex_buffer.last().unwrap().unwrap().norm();
//        }
//        return staunching_factor;
//    }
//}

//pub struct LindenmayerPayload {
//    vertex_buffer: Vec<Option<MathPosition>>,
//    coordinate_buffer: Vec<(MathPosition, f32)>,
//    current_position: MathPosition,
//    current_angle: f32,
//}
//
//impl LindenmayerPayload {
//    pub fn new() -> Self {
//        Self {
//            vertex_buffer: vec![],
//            coordinate_buffer: vec![],
//            current_position: MathPosition::new(0.0, 0.0),
//            current_angle: 90.0f32.to_radians(),
//        }
//    }
//    pub fn compute_base_vertices<Op: Operation>(&mut self, operations: &[Op]) {
//        self.clear_vertex_buffer();
//        self.clear_current_position();
//        self.clear_current_angle();
//        self.vertex_buffer.push(Some(self.current_position));
//        for op in operations {
//            op.apply(self);
//        }
//    }
//    pub fn compute_vertices<Op: Operation>(&mut self, operations: &[Op], scaling_factor: f32) {
//        self.compute_base_vertices(operations);
//        self.apply_center_offset();
//
//        for entry in &mut self.vertex_buffer {
//            if let Some(vertex) = entry {
//                vertex.scale(scaling_factor);
//            }
//        }
//    }
//    pub fn compute_center(&mut self) -> Option<MathPosition> {
//        let mut center = MathPosition::new(0.0, 0.0);
//
//        let mut n_vertices = 0;
//        for entry in &mut self.vertex_buffer {
//            if let Some(vertex) = entry {
//                center += *vertex;
//                n_vertices += 1;
//            }
//        }
//
//        // If we have a closed curve and the first index equals the last,
//        // do not consider the redundant point in the computation of the center.
//        // if (*vertices.first().unwrap() - *vertices.last().unwrap()).norm() <= 5.0 * f32::EPSILON {
//        //     n_vertices = vertices.len() - 1;
//        // }
//
//        // Check for points that are redundant.
//        // We need -1 here since our vertices always include the first also as the last one
//        if n_vertices == 0 {
//            return None;
//        }
//        center.scale(1.0 / (n_vertices as f32));
//        return Some(center);
//    }
//    pub fn apply_center_offset(&mut self) {
//        let center = self.compute_center();
//
//        if let Some(center) = center {
//            for entry in &mut self.vertex_buffer {
//                if let Some(vertex) = entry {
//                    *vertex -= center;
//                }
//            }
//        }
//    }
//    pub fn clear_vertex_buffer(&mut self) {
//        self.vertex_buffer.clear();
//    }
//    pub fn clear_coordinate_buffer(&mut self) {
//        self.coordinate_buffer.clear();
//    }
//    pub fn clear_current_position(&mut self) {
//        self.current_position = MathPosition::new(0.0, 0.0);
//    }
//    pub fn clear_current_angle(&mut self) {
//        self.current_angle = 90.0f32.to_radians();
//    }
//    pub fn update_current_position(&mut self) {
//        self.current_position += MathPosition::new(self.current_angle.cos(), self.current_angle.sin());
//    }
//    pub fn increase_current_angle(&mut self, delta: f32) {
//        self.current_angle += delta;
//    }
//    pub fn decrease_current_angle(&mut self, delta: f32) {
//        self.current_angle -= delta;
//    }
//    pub fn push_current_position(&mut self) {
//        self.vertex_buffer.push(Some(self.current_position));
//    }
//    pub fn save_current_position_and_angle(&mut self) {
//        self.coordinate_buffer.push((self.current_position, self.current_angle));
//    }
//    pub fn pop_and_restore_current_position_and_angle(&mut self) {
//        if let Some((stored_position, stored_angle)) = self.coordinate_buffer.pop() {
//            self.current_position = stored_position;
//            self.current_angle = stored_angle;
//        }
//    }
//}

//pub trait Operation: Copy {
//    fn apply(&self, payload: &mut LindenmayerPayload);
//    // fn apply(&self, vertex_buffer: &mut Vec<Option<MathPosition>>, coordinate_buffer: &mut Vec<(MathPosition, f32)>, current_pos: &mut MathPosition, current_angle: &mut f32);
//    fn forward() -> Self;
//}

//pub trait Replacement: Sized {
//    fn replacement(&self) -> Option<Vec<Self>>;
//}

//pub fn iterate_operations<Op: Operation + Replacement>(operations: &[Op]) -> Vec<Op> {
//    let mut iteration_result = vec![];
//
//    for &op in operations {
//        if let Some(mut replacement) = op.replacement() {
//            iteration_result.append(&mut replacement);
//        } else {
//            iteration_result.push(op);
//        }
//    }
//    return iteration_result;
//}
//
//pub fn iterate_fractal<Op: Operation + Replacement>(base_operations: &[Op], iteration_depth: usize) -> Vec<Vec<Op>> {
//    let mut iteration_results: Vec<Vec<Op>> = vec![base_operations.to_owned()];
//
//    for index in 1..=iteration_depth {
//        let operations = iterate_operations(&iteration_results[index - 1]);
//        iteration_results.push(operations);
//    }
//
//    return iteration_results;
//}

//pub fn iterated_vertices<Op: Operation + Replacement>(iterated_operations: &[Vec<Op>]) -> Vec<Vec<Option<MathPosition>>> {
//    let mut iteration_results = vec![];
//    let mut payload = LindenmayerPayload::new();
//    let staunching_factor = compute_staunching_factor::<Op>();
//
//    for (index, operations) in iterated_operations.iter().enumerate() {
//        payload.compute_vertices(operations, S);
//        for entry in &mut payload.vertex_buffer {
//            if let Some(vertex) = entry {
//                vertex.scale(staunching_factor.powi(index as i32));
//            }
//        }
//        iteration_results.push(payload.vertex_buffer.clone());
//    }
//    return iteration_results;
//}

// pub fn iterated_vertices<Op: Operation + Replacement>(iterated_operations: &[Vec<Op>]) -> Vec<Vec<Option<MathPosition>>> {
//     let mut iteration_results = vec![];
//     let mut payload = LindenmayerPayload::new();
//     let staunching_factor = compute_staunching_factor::<Op>();
// 
//     for (index, operations) in iterated_operations.iter().enumerate() {
//         payload.compute_vertices(operations, S);
//         for entry in &mut payload.vertex_buffer {
//             if let Some(vertex) = entry {
//                 vertex.scale(staunching_factor.powi(index as i32));
//             }
//         }
//         iteration_results.push(payload.vertex_buffer.clone());
//     }
//     return iteration_results;
// }

// Compute the center of a given set of vertices
//pub fn compute_center(vertices: &[Option<MathPosition>]) -> Option<MathPosition> {
//    let mut center = MathPosition::new(0.0, 0.0);
//
//    let mut n_vertices = 0;
//    for entry in vertices {
//        if let Some(vertex) = entry {
//            center += *vertex;
//            n_vertices += 1;
//        }
//    }
//
//    // If we have a closed curve and the first index equals the last,
//    // do not consider the redundant point in the computation of the center.
//    // if (*vertices.first().unwrap() - *vertices.last().unwrap()).norm() <= 5.0 * f32::EPSILON {
//    //     n_vertices = vertices.len() - 1;
//    // }
//
//    // Check for points that are redundant.
//    // We need -1 here since our vertices always include the first also as the last one
//    if n_vertices == 0 {
//        return None;
//    }
//    center.scale(1.0 / (n_vertices as f32));
//    return Some(center);
//}

// Apply center offset to all vertices in the given set.
//pub fn apply_center_offset(vertices: &mut [Option<MathPosition>]) {
//    let center = compute_center(vertices);
//
//    if let Some(center) = center {
//        for vertex in vertices {
//            if let Some(vertex) = vertex {
//                *vertex -= center;
//            }
//        }
//    }
//}

// fn compute_staunching_factor<Op: Operation + Replacement>() -> f32 {
//     let mut payload = LindenmayerPayload::new();
//     let mut staunching_factor = 1.0;
//     if let Some(replacement) = Op::forward().replacement() {
//         payload.compute_base_vertices(&replacement);
//         staunching_factor = 1.0 / payload.vertex_buffer.last().unwrap().unwrap().norm();
//     }
//     return staunching_factor;
// }

// pub fn compute_base_vertices<Op: Operation>(operations: &[Op]) -> Vec<Option<MathPosition>> {
//     let mut vertex_buffer = vec![];
//     let mut coordinate_buffer = vec![];
//     let mut current_pos = MathPosition::new(0.0, 0.0);
//     let mut current_angle: f32 = 90.0f32.to_radians();
// 
//     vertex_buffer.push(Some(current_pos));
//     for op in operations {
//         op.apply(&mut vertex_buffer, &mut coordinate_buffer, &mut current_pos, &mut current_angle);
//     }
// 
//     return vertex_buffer;
// }

// TODO: this leaves out the first point at 0.0.
// Not a problem if fractal loops around, as then last point will be equal to first
// Compute the corresponding vertices for a given set of operations
//pub fn compute_scaled_vertices<Op: Operation>(operations: &[Op]) -> Vec<Option<MathPosition>> {
//    let mut vertex_buffer = compute_base_vertices(operations);
//
//    apply_center_offset(&mut vertex_buffer);
//    for vertex in &mut vertex_buffer {
//        if let Some(vertex) = vertex {
//            vertex.scale(S);
//        }
//    }
//
//    return vertex_buffer;
//}

pub fn draw_polygon(primitives: &PrimitivesAddon, vertices: &[MathPosition], color: Color) {
    let vertices: Vec<(f32, f32)> = vertices.iter()
        .map(|pos| {
            ScreenPosition::from(pos).into()
        })
        .collect();
    primitives.draw_polygon(&vertices, LineJoinType::Round, color, 2.0, 1.0);
}

pub fn draw_single_lines(primitives: &PrimitivesAddon, vertices: &[Option<MathPosition>], color: Color) {
    let (red, green, blue) = color.to_rgb_f();
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
                primitives.draw_line(vertex.0, vertex.1, next_vertex.0, next_vertex.1, Color::from_rgb_f(red - 0.0000001f32 * index as f32, green - 0.0000001f32 * index as f32, blue - 0.0000001f32 * index as f32), 2.0);
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

    //let mut current_depth = 0;

    // let base_operations = vec![Koch::F, Koch::R, Koch::R, Koch::F, Koch::R, Koch::R, Koch::F];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![Levy::F];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![SierTepp::F];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![DragonCurve::F];
    // let iterated_operations = iterate_fractal(&base_operations, 20);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![GosperCurve::F];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![HilbertCurve::A];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![PentaPlexity::F, PentaPlexity::L, PentaPlexity::L, PentaPlexity::F, PentaPlexity::L, PentaPlexity::L, PentaPlexity::F, PentaPlexity::L, PentaPlexity::L, PentaPlexity::F, PentaPlexity::L, PentaPlexity::L, PentaPlexity::F];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![ArrowHead::F];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![SierpinskiTriangle::F, SierpinskiTriangle::R, SierpinskiTriangle::R, SierpinskiTriangle::F, SierpinskiTriangle::R, SierpinskiTriangle::R, SierpinskiTriangle::F];
    // let iterated_operations = iterate_fractal(&base_operations, 6);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![FirstPlant::X];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![PlantOne::X];
    // let iterated_operations = iterate_fractal(&base_operations, 10);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![PlantTwo::X];
    // let iterated_operations = iterate_fractal(&base_operations, 15);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);

    // let base_operations = vec![PlantThree::X];
    // let iterated_operations = iterate_fractal(&base_operations, 15);
    // let vertex_iterations = iterated_vertices(&iterated_operations[..]);
    
    //let mut fractals = vec![
    //    LindenmayerSystem::koch(),
    //    LindenmayerSystem::levy(),
    //    LindenmayerSystem::dragon_curve(),
    //    LindenmayerSystem::first_plant(),
    //];

    //let mut current_fractal = 0;
    //fractals[current_fractal].update_vertex_stack(8);

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let first_word = Word::new(
        &[
            MyFirstLetter::Forward {length: 1.0f32},
            MyFirstLetter::TurnLeft {angle: 1.0f32},
            MyFirstLetter::TurnRight {angle: 1.0f32},
        ]
    );
    let second_word = Word::new(
        &[
            MySecondLetter::Forward {length: 0.5f32},
            MySecondLetter::TurnLeft {angle: 0.5f32},
            MySecondLetter::TurnRight {angle: 0.5f32},
        ]
    );

    println!("{}", first_word);
    println!("{}", second_word);

    let mut holder = WordHolder::new(Box::new(first_word));
    holder.change_word(Box::new(second_word));

    let mut redraw = true;
    timer.start();

    'exit: loop {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.1, 0.1, 0.1));
            // draw_single_lines(&primitives, &vertex_iterations[current_depth], Color::from_rgb_f(0.7, 0.9, 0.7));
            //if let Some(vertices) = fractals[current_fractal].get_vertex_stack_at(current_depth) {
            //    draw_single_lines(&primitives, &vertices[..], Color::from_rgb_f(0.5, 0.9, 0.7));
            //}

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
            //KeyDown{source: _, timestamp: _, keycode, ..} => {
            //    match keycode {
            //        KeyCode::I => {
            //            println!("Key: I");
            //            if current_depth < 8 {
            //                current_depth += 1;
            //            }
            //        },
            //        KeyCode::P => {
            //            println!("Key: P");
            //            if current_depth > 0 {
            //                current_depth -= 1;
            //            }
            //        },
            //        KeyCode::C => {
            //            println!("Key: C");
            //            current_fractal += 1;
            //            current_fractal = current_fractal.checked_rem_euclid(fractals.len()).unwrap();
            //            fractals[current_fractal].update_vertex_stack(8);
            //        },
            //        _ => (),
            //    }
            //},
            _ => (),
        }
    }
}
