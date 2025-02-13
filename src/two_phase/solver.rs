use std::{
    fmt,
    time::{Duration, Instant},
};

use crate::{
    cube::{
        index::*,
        moves::{is_move_available, Move},
        state::{State, SOLVED_STATE},
    },
    fs::read_table,
};

use super::moves::MoveTable;
use super::pruning::PruningTable;
use super::utils::{ALL_MOVES, PHASE2_MOVES};

#[derive(Debug)]
struct Phase1State {
    co_index: usize,
    eo_index: usize,
    e_combo_index: usize,
}

impl Phase1State {
    fn is_solved(&self) -> bool {
        self.co_index == 0 && self.eo_index == 0 && self.e_combo_index == 0
    }

    fn next(&self, table: &MoveTable, move_index: usize) -> Self {
        let co_index = table.co[self.co_index][move_index].into();
        let eo_index = table.eo[self.eo_index][move_index].into();
        let e_combo_index = table.e_combo[self.e_combo_index][move_index].into();

        Self {
            co_index,
            eo_index,
            e_combo_index,
        }
    }

    fn prune(&self, table: &PruningTable, depth: u8) -> bool {
        let co_e_dist = table.co_e[self.co_index][self.e_combo_index];
        let eo_e_dist = table.eo_e[self.eo_index][self.e_combo_index];
        let max = co_e_dist.max(eo_e_dist);

        max > depth
    }
}

impl From<State> for Phase1State {
    fn from(value: State) -> Self {
        let co_index = co_to_index(&value.co).into();
        let eo_index = eo_to_index(&value.eo).into();
        let e_combo_index = e_combo_to_index(&value.ep).into();

        Self {
            co_index,
            eo_index,
            e_combo_index,
        }
    }
}

struct Phase2State {
    cp_index: usize,
    ep_index: usize,
    e_ep_index: usize,
}

impl From<State> for Phase2State {
    fn from(value: State) -> Self {
        let cp_index = cp_to_index(&value.cp).into();
        let ep_index = ud_ep_to_index(&value.ep).into();
        let e_ep_index = e_ep_to_index(&value.ep).into();

        Self {
            cp_index,
            ep_index,
            e_ep_index,
        }
    }
}

impl Phase2State {
    fn is_solved(&self) -> bool {
        self.cp_index == 0 && self.ep_index == 0 && self.e_ep_index == 0
    }

    fn next(&self, table: &MoveTable, move_index: usize) -> Self {
        let cp_index = table.cp[self.cp_index][move_index].into();
        let ep_index = table.ep[self.ep_index][move_index].into();
        let e_ep_index = table.e_ep[self.e_ep_index][move_index].into();

        Self {
            cp_index,
            ep_index,
            e_ep_index,
        }
    }

    fn prune(&self, table: &PruningTable, depth: u8) -> bool {
        let cp_e_dist = table.cp_e[self.cp_index][self.e_ep_index];
        let ep_e_dist = table.ep_e[self.ep_index][self.e_ep_index];
        let max = cp_e_dist.max(ep_e_dist);

        max > depth
    }
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub phase_1: Vec<Move>,
    pub phase_2: Vec<Move>,
}

impl Solution {
    pub fn len(&self) -> usize {
        self.phase_1.len() + self.phase_2.len()
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut clone = self.phase_1.clone();
        clone.extend(&self.phase_2);
        let stringified = clone
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{stringified}")
    }
}

impl Solution {
    pub fn phase1_to_string(&self) -> String {
        self.phase_1
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn phase2_to_string(&self) -> String {
        self.phase_2
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut solution = self.phase_1.clone();
        solution.extend(&self.phase_2);
        solution
    }
}

pub struct Solver<'a> {
    move_table: &'a MoveTable,
    pruning_table: &'a PruningTable,
    max_length: u8,
    timeout: Option<Duration>,
    initial_state: State,
    solution_phase_1: Vec<Move>,
    solution_phase_2: Vec<Move>,
    best_solution: Option<Solution>,
}

impl<'a> Solver<'a> {
    pub fn new(
        move_table: &'a MoveTable,
        pruning_table: &'a PruningTable,
        max_length: u8,
        timeout: Option<f32>,
    ) -> Self {
        let timeout = match timeout {
            Some(value) => Some(Duration::from_secs_f32(value)),
            None => None,
        };

        Self {
            move_table,
            pruning_table,
            initial_state: SOLVED_STATE,
            max_length,
            timeout,
            solution_phase_1: vec![],
            solution_phase_2: vec![],
            best_solution: None,
        }
    }

    pub fn solve(&mut self, state: State) -> Option<Solution> {
        self.initial_state = state;

        let start = Instant::now();

        for depth in 0..=self.max_length {
            let state = Phase1State::from(state);
            let found = self.solve_phase_1(state, depth, start);

            if let Some(timeout) = self.timeout {
                if start.elapsed() > timeout {
                    return self.best_solution.clone();
                }
            } else if found {
                return self.best_solution.clone();
            }
        }

        None
    }

    fn solve_phase_1(&mut self, state: Phase1State, depth: u8, time: Instant) -> bool {
        if let Some(timeout) = self.timeout {
            if time.elapsed() > timeout {
                return true;
            }
        }

        if depth == 0 && state.is_solved() {
            let mut cube_state = self.initial_state;

            for m in &self.solution_phase_1 {
                cube_state = cube_state.apply_move(*m);
            }

            let max_depth = match self.solution_phase_1.len() {
                0 => self.max_length,
                _ => {
                    if self.max_length > self.solution_phase_1.len() as u8 {
                        self.max_length - self.solution_phase_1.len() as u8
                    } else {
                        return true;
                    }
                }
            };

            for phase2_depth in 0..max_depth {
                let state = Phase2State::from(cube_state);
                if self.solve_phase_2(state, phase2_depth, time) {
                    return true;
                }
            }

            return false;
        }

        if state.prune(&self.pruning_table, depth) || depth == 0 {
            return false;
        }

        for (i_m, m) in ALL_MOVES.iter().enumerate() {
            if let Some(prev) = self.solution_phase_1.last() {
                if !is_move_available(*prev, *m) {
                    continue;
                }
            }

            self.solution_phase_1.push(*m);
            let new_state = state.next(&self.move_table, i_m);
            let found = self.solve_phase_1(new_state, depth - 1, time);

            if found {
                return true;
            }

            self.solution_phase_1.pop();
        }

        false
    }

    fn solve_phase_2(&mut self, state: Phase2State, depth: u8, time: Instant) -> bool {
        if let Some(timeout) = self.timeout {
            if time.elapsed() > timeout {
                return true;
            }
        }

        if depth == 0 && state.is_solved() {
            let solution = Solution {
                phase_1: self.solution_phase_1.clone(),
                phase_2: self.solution_phase_2.clone(),
            };

            if let Some(best_solution) = &mut self.best_solution {
                let current_length = self.solution_phase_1.len() + self.solution_phase_2.len();
                if best_solution.len() > current_length {
                    *best_solution = solution
                }
            } else {
                self.best_solution = Some(solution)
            }

            return true;
        }

        if state.prune(&self.pruning_table, depth) || depth == 0 {
            return false;
        }

        for (i_m, m) in PHASE2_MOVES.iter().enumerate() {
            if let Some(prev) = self.solution_phase_2.last() {
                if !is_move_available(*prev, *m) {
                    continue;
                }
            } else {
                if let Some(prev) = self.solution_phase_1.last() {
                    if !is_move_available(*prev, *m) {
                        continue;
                    }
                }
            }

            self.solution_phase_2.push(*m);
            let new_state = state.next(&self.move_table, i_m);
            let found = self.solve_phase_2(new_state, depth - 1, time);

            if found {
                return true;
            }

            self.solution_phase_2.pop();
        }

        false
    }
}

pub fn solve(state: State, max_length: u8, timeout: Option<f32>) -> Option<Solution> {
    let (move_table, pruning_table) = read_table().unwrap();
    let mut solver = Solver::new(&move_table, &pruning_table, max_length, timeout);
    solver.solve(state)
}

#[cfg(test)]
mod test {
    use crate::{
        cube::{
            moves::Move::*,
            state::{State, SOLVED_STATE},
        },
        two_phase::solver::solve,
    };

    #[test]
    fn test_solve() {
        let scramble = vec![
            D3, R2, L3, U2, F, R, F3, D2, R2, F2, B2, U2, R2, F2, U, R2, U3, R2, D2,
        ];
        let state = State::from(&scramble);
        let solution = solve(state, 23, None).unwrap();
        let solved_state = state.apply_moves(&solution.get_all_moves());

        assert_eq!(solved_state, SOLVED_STATE);
    }
}
