use crate::{index::*, utils::*};
use bincode::{Decode, Encode};
use cube::state::{Edge::*, SOLVED_STATE};

#[derive(Encode, Decode)]
pub struct MoveTable {
    pub co: Table<u16>,
    pub eo: Table<u16>,
    pub e_combo: Table<u16>,
    pub cp: Table<u16>,
    pub ep: Table<u16>,
    pub e_ep: Table<u16>,
}

pub fn get_co_table() -> Table<u16> {
    let mut co_table = vec![vec![0; 18]; CO_COUNT as usize];

    for i in 0..CO_COUNT {
        let co = index_to_co(i);
        let mut state = SOLVED_STATE;
        state.co = co;

        for (i_m, m) in ALL_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            co_table[i as usize][i_m] = co_to_index(&new_state.co);
        }
    }

    co_table
}

pub fn get_eo_table() -> Table<u16> {
    let mut eo_table = vec![vec![0; 18]; EO_COUNT as usize];

    for i in 0..EO_COUNT {
        let eo = index_to_eo(i);
        let mut state = SOLVED_STATE;
        state.eo = eo;

        for (i_m, m) in ALL_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            eo_table[i as usize][i_m] = eo_to_index(&new_state.eo);
        }
    }

    eo_table
}

pub fn get_e_combo_table() -> Table<u16> {
    let mut e_combo_table = vec![vec![0; 18]; E_COMBO_COUNT as usize];
    for i in 0..E_COMBO_COUNT {
        let e_combo = index_to_e_combo(i);
        let mut ep = SOLVED_STATE.ep;

        for i in 0..12 {
            ep[i] = if e_combo[i] == 1 { BR } else { BL }
        }

        let mut state = SOLVED_STATE;
        state.ep = ep;

        for (i_m, m) in ALL_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            let mut new_e_combo = [0; 12];

            for i in 0..12 {
                new_e_combo[i] = new_state.ep[i] as u8;
            }

            e_combo_table[i as usize][i_m] = e_combo_to_index(&new_e_combo);
        }
    }

    e_combo_table
}

pub fn get_cp_table() -> Table<u16> {
    let mut cp_table = vec![vec![0; 10]; CP_COUNT as usize];

    for i in 0..CP_COUNT {
        let cp = index_to_cp(i);
        let mut state = SOLVED_STATE;
        state.cp = cp;

        for (i_m, m) in PHASE2_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            cp_table[i as usize][i_m] = cp_to_index(&new_state.cp);
        }
    }

    cp_table
}

pub fn get_ep_table() -> Table<u16> {
    let mut ep_table = vec![vec![0; 10]; EP_COUNT as usize];

    for i in 0..EP_COUNT {
        let ep = index_to_ud_ep(i);
        let mut state = SOLVED_STATE;
        state.ep = ep;

        for (i_m, m) in PHASE2_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            ep_table[i as usize][i_m] = ud_ep_to_index(&new_state.ep);
        }
    }

    ep_table
}

pub fn get_e_ep_table() -> Table<u16> {
    let mut e_ep_table = vec![vec![0; 10]; E_EP_COUNT as usize];

    for i in 0..E_EP_COUNT {
        let ep = index_to_e_ep(i);
        let mut state = SOLVED_STATE;
        state.ep = ep;

        for (i_m, m) in PHASE2_MOVES.iter().enumerate() {
            let new_state = state.apply_move(*m);
            e_ep_table[i as usize][i_m] = e_ep_to_index(&new_state.ep);
        }
    }

    e_ep_table
}
