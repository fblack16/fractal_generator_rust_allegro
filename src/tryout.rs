use std::{collections::HashMap, hash::Hash};
use crate::{coordinates::MathPosition, S};

pub trait Payload {}
pub trait Letter: Copy + Clone + PartialEq + Eq + Hash {
    fn forward() -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LindenmayerPayload {
    vertex_buffer: Vec<Option<MathPosition>>,
    coordinate_buffer: Vec<(MathPosition, f32)>,
    current_position: MathPosition,
    current_angle: f32,
}

impl LindenmayerPayload {
    pub fn new() -> Self {
        Self {
            vertex_buffer: vec![],
            coordinate_buffer: vec![],
            current_position: MathPosition::new(0.0f32, 0.0f32),
            current_angle: 90.0f32.to_radians(),
        }
    }
    pub fn compute_base_vertices<L: Letter>(&mut self, word: &[L], actions: &HashMap<L, Option<fn(payload: &mut LindenmayerPayload)>>) {
        self.clear_vertex_buffer();
        self.clear_current_position();
        self.clear_current_angle();
        self.vertex_buffer.push(Some(self.current_position));
        for letter in word {
            if let Some(Some(action)) = actions.get(letter) {
                action(self);
            }
        }
    }
    pub fn compute_vertices<L: Letter>(&mut self, word: &[L], actions: &HashMap<L, Option<fn(payload: &mut LindenmayerPayload)>>, scaling_factor: f32) {
        self.compute_base_vertices(word, actions);
        self.apply_center_offset();

        for entry in &mut self.vertex_buffer {
            if let Some(vertex) = entry {
                vertex.scale(scaling_factor);
            }
        }
    }
    pub fn compute_center(&mut self) -> Option<MathPosition> {
        let mut center = MathPosition::new(0.0, 0.0);

        let mut n_vertices = 0;
        for entry in &mut self.vertex_buffer {
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
    pub fn apply_center_offset(&mut self) {
        let center = self.compute_center();

        if let Some(center) = center {
            for entry in &mut self.vertex_buffer {
                if let Some(vertex) = entry {
                    *vertex -= center;
                }
            }
        }
    }
    pub fn clear_vertex_buffer(&mut self) {
        self.vertex_buffer.clear();
    }
    pub fn clear_coordinate_buffer(&mut self) {
        self.coordinate_buffer.clear();
    }
    pub fn clear_current_position(&mut self) {
        self.current_position = MathPosition::new(0.0f32, 0.0f32);
    }
    pub fn clear_current_angle(&mut self) {
        self.current_angle = 90.0f32.to_radians();
    }
    pub fn update_current_position(&mut self) {
        self.current_position += MathPosition::new(self.current_angle.cos(), self.current_angle.sin());
    }
    pub fn increase_current_angle(&mut self, delta: f32) {
        self.current_angle += delta;
    }
    pub fn decrease_current_angle(&mut self, delta: f32) {
        self.current_angle -= delta;
    }
    pub fn push_current_position(&mut self) {
        self.vertex_buffer.push(Some(self.current_position));
    }
    pub fn save_current_position_and_angle(&mut self) {
        self.coordinate_buffer.push((self.current_position, self.current_angle));
    }
    pub fn pop_and_restore_current_position_and_angle(&mut self) {
        if let Some((stored_position, stored_angle)) = self.coordinate_buffer.pop() {
            self.current_position = stored_position;
            self.current_angle = stored_angle;
        }
    }
}

#[derive(Clone)]
pub struct LindenmayerSystem<L: Letter> {
    starting_word: Vec<L>,
    word_stack: Vec<Vec<L>>,
    vertex_stack: Vec<Vec<Option<MathPosition>>>,
    production_rules: HashMap<L, Option<Vec<L>>>,
    actions: HashMap<L, Option<fn(payload: &mut LindenmayerPayload)>>,
    payload: LindenmayerPayload,
    angle: f32,
    staunching_factor: f32,
}

impl<L: Letter> LindenmayerSystem<L> {
    pub fn new(starting_word: &[L], angle: f32, production_rules: &[(L, Option<Vec<L>>)], actions: &[(L, Option<fn(payload: &mut LindenmayerPayload)>)]) -> Self {
        let starting_word = starting_word.to_owned();
        let word_stack = vec![starting_word.clone()];
        let production_rules: HashMap<L, Option<Vec<L>>> = production_rules.to_owned().into_iter().collect();
        let actions: HashMap<L, Option<fn(payload: &mut LindenmayerPayload)>> = actions.to_owned().into_iter().collect();
        let mut payload = LindenmayerPayload::new();

        payload.compute_vertices(&starting_word, &actions, S);
        let vertex_stack = vec![payload.vertex_buffer.clone()];
        payload.clear_current_position();
        payload.clear_current_angle();
        payload.clear_vertex_buffer();
        payload.clear_coordinate_buffer();

        let mut fractal = Self {
            starting_word,
            word_stack,
            vertex_stack,
            production_rules,
            actions,
            payload,
            angle,
            staunching_factor: 1.0f32,
        };

        fractal.compute_staunching_factor();
        return fractal;
    }
    pub fn change_starting_word(&mut self, starting_word: &[L]) {
        self.word_stack.clear();
        self.vertex_stack.clear();
        self.starting_word = starting_word.to_owned();
        self.word_stack.push(self.starting_word.clone());
        self.payload.compute_vertices(&self.starting_word, &self.actions, S);
        self.vertex_stack.push(self.payload.vertex_buffer.clone());
    }
    pub fn change_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
    pub fn with_production_rules(&mut self, production_rules: &[(L, Option<Vec<L>>)]) {
        self.production_rules = production_rules.to_owned().into_iter().collect();
        self.compute_staunching_factor();
    }
    pub fn with_actions(&mut self, actions: &[(L, Option<fn(payload: &mut LindenmayerPayload)>)]) {
        self.actions = actions.to_owned().into_iter().collect();
    }
    pub fn change_production_rule(&mut self, letter: L, replacement: Option<Vec<L>>) {
        self.production_rules.insert(letter, replacement);
        self.compute_staunching_factor();
    }
    pub fn change_action(&mut self, letter: L, action: Option<fn(payload: &mut LindenmayerPayload)>) {
        self.actions.insert(letter, action);
    }
    pub fn apply_production_rules(&mut self) {
        let mut result = vec![];
        if let Some(word) = self.word_stack.last() {
            for letter in word {
                if let Some(Some(replacement)) = self.production_rules.get(letter) {
                    result.append(&mut replacement.clone());
                } else {
                    result.push(*letter);
                }
            }
        }
        self.word_stack.push(result);
    }
    pub fn apply_actions(&mut self) {
        let depth = self.word_stack.len() - 1;
        if let Some(word) = self.word_stack.last() {
            self.payload.compute_vertices(word, &self.actions, S);
            for entry in &mut self.payload.vertex_buffer {
                if let Some(vertex) = entry {
                    vertex.scale(self.staunching_factor.powi(depth as i32));
                }
            }
            self.vertex_stack.push(self.payload.vertex_buffer.clone());
        }
    }
    pub fn compute_staunching_factor(&mut self) {
        let mut payload = LindenmayerPayload::new();
        if let Some(Some(replacement)) = self.production_rules.get(&L::forward()) {
            payload.compute_base_vertices(replacement, &self.actions);
            self.staunching_factor = 1.0 / payload.vertex_buffer.last().unwrap().unwrap().norm();
        }
    }
    pub fn update_vertex_stack(&mut self, depth: usize) {
        if depth >= self.vertex_stack.len() {
            for _ in 1..=(depth - self.vertex_stack.len() + 1) {
                self.apply_production_rules();
                self.apply_actions();
            }
        }
    }
    pub fn get_vertex_stack_at(&self, index: usize) -> Option<&Vec<Option<MathPosition>>> {
        if index < self.vertex_stack.len() {
            return Some(&self.vertex_stack[index]);
        }
        return None;
    }
    pub fn get_vertex_stack(&self) -> &Vec<Vec<Option<MathPosition>>> {
        return &self.vertex_stack;
    }
}

impl LindenmayerSystem<LindenmayerLetter> {
    pub fn koch() -> Self {
        let starting_word = vec![
            LindenmayerLetter::F,
            LindenmayerLetter::R,
            LindenmayerLetter::R,
            LindenmayerLetter::F,
            LindenmayerLetter::R,
            LindenmayerLetter::R,
            LindenmayerLetter::F,
        ];

        let angle = 60.0f32.to_radians();

        let actions: Vec<(LindenmayerLetter, Option<fn(payload: &mut LindenmayerPayload)>)> = vec![
            (
                LindenmayerLetter::F,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.update_current_position();
                    payload.push_current_position();
                }),
            ),
            (
                LindenmayerLetter::L,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.increase_current_angle(60.0f32.to_radians());
                }),
            ),
            (
                LindenmayerLetter::R,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.decrease_current_angle(60.0f32.to_radians());
                }),
            ),
        ];

        let production_rules = vec![
            (LindenmayerLetter::F, Some(vec![
                LindenmayerLetter::F,
                LindenmayerLetter::L,
                LindenmayerLetter::F,
                LindenmayerLetter::R,
                LindenmayerLetter::R,
                LindenmayerLetter::F,
                LindenmayerLetter::L,
                LindenmayerLetter::F,
            ])),
        ];


        LindenmayerSystem::new(
            &starting_word,
            angle,
            &production_rules,
            &actions,
        )
    }

    pub fn levy() -> Self {
        let starting_word = vec![
            LindenmayerLetter::F,
        ];

        let angle = 45.0f32.to_radians();

        let actions: Vec<(LindenmayerLetter, Option<fn(payload: &mut LindenmayerPayload)>)> = vec![
            (
                LindenmayerLetter::F,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.update_current_position();
                    payload.push_current_position();
                }),
            ),
            (
                LindenmayerLetter::L,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.increase_current_angle(45.0f32.to_radians());
                }),
            ),
            (
                LindenmayerLetter::R,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.decrease_current_angle(45.0f32.to_radians());
                }),
            ),
        ];

        let production_rules = vec![
            (LindenmayerLetter::F, Some(vec![
                LindenmayerLetter::L,
                LindenmayerLetter::F,
                LindenmayerLetter::R,
                LindenmayerLetter::R,
                LindenmayerLetter::F,
                LindenmayerLetter::L,
            ])),
        ];


        LindenmayerSystem::new(
            &starting_word,
            angle,
            &production_rules,
            &actions,
        )
    }

    pub fn dragon_curve() -> Self {
        let starting_word = vec![
            LindenmayerLetter::F,
        ];

        let angle = 45.0f32.to_radians();

        let actions: Vec<(LindenmayerLetter, Option<fn(payload: &mut LindenmayerPayload)>)> = vec![
            (
                LindenmayerLetter::F,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.update_current_position();
                    payload.push_current_position();
                }),
            ),
            (
                LindenmayerLetter::G,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.update_current_position();
                    payload.push_current_position();
                }),
            ),
            (
                LindenmayerLetter::L,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.increase_current_angle(45.0f32.to_radians());
                }),
            ),
            (
                LindenmayerLetter::R,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.decrease_current_angle(45.0f32.to_radians());
                }),
            ),
        ];

        let production_rules = vec![
            (LindenmayerLetter::F, Some(vec![
                LindenmayerLetter::L,
                LindenmayerLetter::F,
                LindenmayerLetter::R,
                LindenmayerLetter::R,
                LindenmayerLetter::G,
                LindenmayerLetter::L,
            ])),
            (LindenmayerLetter::G, Some(vec![
                LindenmayerLetter::R,
                LindenmayerLetter::F,
                LindenmayerLetter::L,
                LindenmayerLetter::L,
                LindenmayerLetter::G,
                LindenmayerLetter::R,
            ])),
        ];


        LindenmayerSystem::new(
            &starting_word,
            angle,
            &production_rules,
            &actions,
        )
    }

    pub fn first_plant() -> Self {
        let starting_word = vec![
            LindenmayerLetter::X,
        ];

        let angle = 25.0f32.to_radians();

        let actions: Vec<(LindenmayerLetter, Option<fn(payload: &mut LindenmayerPayload)>)> = vec![
            (
                LindenmayerLetter::F,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.update_current_position();
                    payload.push_current_position();
                }),
            ),
            (
                LindenmayerLetter::L,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.increase_current_angle(25.0f32.to_radians());
                }),
            ),
            (
                LindenmayerLetter::R,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.decrease_current_angle(25.0f32.to_radians());
                }),
            ),
            (
                LindenmayerLetter::PUSH,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.save_current_position_and_angle();
                }),
            ),
            (
                LindenmayerLetter::POP,
                Some(|payload: &mut LindenmayerPayload| {
                    payload.pop_and_restore_current_position_and_angle();
                    payload.vertex_buffer.push(None);
                    payload.push_current_position();
                })
            ),
        ];

        let production_rules = vec![
            (LindenmayerLetter::X, Some(vec![
                LindenmayerLetter::F,
                LindenmayerLetter::L,
                LindenmayerLetter::PUSH,
                LindenmayerLetter::PUSH,
                LindenmayerLetter::X,
                LindenmayerLetter::POP,
                LindenmayerLetter::R,
                LindenmayerLetter::X,
                LindenmayerLetter::POP,
                LindenmayerLetter::R,
                LindenmayerLetter::F,
                LindenmayerLetter::PUSH,
                LindenmayerLetter::R,
                LindenmayerLetter::F,
                LindenmayerLetter::X,
                LindenmayerLetter::POP,
                LindenmayerLetter::L,
                LindenmayerLetter::X,
            ])),
            (LindenmayerLetter::F, Some(vec![
                LindenmayerLetter::F,
                LindenmayerLetter::F,
            ])),
        ];


        LindenmayerSystem::new(
            &starting_word,
            angle,
            &production_rules,
            &actions,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LindenmayerLetter {
    F,
    G,
    H,
    L,
    R,
    X,
    Y,
    Z,
    PUSH,
    POP,
}

impl Letter for LindenmayerLetter {
    fn forward() -> Self {
        Self::F
    }
}
