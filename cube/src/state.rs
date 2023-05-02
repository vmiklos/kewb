use self::{Corner::*, Edge::*, Move::*};
use crate::moves::*;
use std::ops::Mul;

#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Corner {
    UBL, UBR, UFR, UFL,
    DFL, DFR, DBR, DBL,
}

impl From<u8> for Corner {
    fn from(value: u8) -> Self {
        match value {
            0 => UBL,
            1 => UBR,
            2 => UFR,
            3 => UFL,
            4 => DFL,
            5 => DFR,
            6 => DBR,
            7 => DBL,
            _ => panic!("Invalid corner value"),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Edge {
    BL, BR, FR, FL,
    UB, UR, UF, UL,
    DF, DR, DB, DL,
}

impl From<u8> for Edge {
    fn from(value: u8) -> Self {
        match value {
            0 => BL,
            1 => BR,
            2 => FR,
            3 => FL,
            4 => UB,
            5 => UR,
            6 => UF,
            7 => UL,
            8 => DF,
            9 => DR,
            10 => DB,
            11 => DL,
            _ => panic!("Invalid edge value"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct State {
    pub cp: [Corner; 8],
    pub co: [u8; 8],
    pub ep: [Edge; 12],
    pub eo: [u8; 12],
}

impl Mul for State {
    type Output = Self;

    fn mul(self, rhs: State) -> Self::Output {
        let mut res = SOLVED_STATE;
        // (A * B).c = A(B(x).c).c
        // (A * B).o = A(B(x).c).o + B(x).o

        for i in 0..8 {
            res.cp[i] = self.cp[rhs.cp[i] as usize];
            res.co[i] = (self.co[rhs.cp[i] as usize] + rhs.co[i]) % 3;
        }

        for i in 0..12 {
            res.ep[i] = self.ep[rhs.ep[i] as usize];
            res.eo[i] = (self.eo[rhs.ep[i] as usize] + rhs.eo[i]) % 2;
        }

        res
    }
}

impl State {
    pub fn apply_move(self, move_name: Move) -> Self {
        let move_state = match move_name {
            U => U_MOVE,
            U2 => U_MOVE * U_MOVE,
            U3 => U_MOVE * U_MOVE * U_MOVE,
            D => D_MOVE,
            D2 => D_MOVE * D_MOVE,
            D3 => D_MOVE * D_MOVE * D_MOVE,
            R => R_MOVE,
            R2 => R_MOVE * R_MOVE,
            R3 => R_MOVE * R_MOVE * R_MOVE,
            L => L_MOVE,
            L2 => L_MOVE * L_MOVE,
            L3 => L_MOVE * L_MOVE * L_MOVE,
            F => F_MOVE,
            F2 => F_MOVE * F_MOVE,
            F3 => F_MOVE * F_MOVE * F_MOVE,
            B => B_MOVE,
            B2 => B_MOVE * B_MOVE,
            B3 => B_MOVE * B_MOVE * B_MOVE,
        };

        self * move_state
    }
}

impl From<&Vec<Move>> for State {
    fn from(moves: &Vec<Move>) -> Self {
        let mut state = SOLVED_STATE;

        for m in moves {
            state = state.apply_move(*m);
        }

        state
    }
}

pub const SOLVED_STATE: State = State {
    cp: [UBL, UBR, UFR, UFL, DFL, DFR, DBR, DBL],
    co: [0, 0, 0, 0, 0, 0, 0, 0],
    ep: [BL, BR, FR, FL, UB, UR, UF, UL, DF, DR, DB, DL],
    eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

#[cfg(test)]
mod test {
    use super::{Corner::*, Edge::*, Move::*, SOLVED_STATE};
    use crate::{moves::*, state::State};

    #[test]
    fn test_mult() {
        let state = SOLVED_STATE.apply_move(R);
        assert_eq!(state, R_MOVE);

        let r2_state = SOLVED_STATE.apply_move(R).apply_move(R);
        assert_eq!(r2_state, R_MOVE * R_MOVE);

        let r3_state = r2_state.apply_move(R);
        assert_eq!(r3_state, r2_state * R_MOVE);

        let fr_state = State {
            cp: [UBL, UFL, UFR, DFL, DFR, DBR, UBR, DBL],
            co: [0, 2, 1, 2, 1, 1, 2, 0],
            ep: [BL, UR, DR, DF, UB, UF, FL, UL, FR, BR, DB, DL],
            eo: [0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0],
        };

        assert_eq!(F_MOVE * R_MOVE, fr_state);
    }

    #[test]
    fn test_move_sequence() {
        // (R U R' U') * 6
        let moves = [
            R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3, R, U, R3, U3,
        ];
        let mut state = SOLVED_STATE;

        for m in moves {
            state = state.apply_move(m);
        }

        assert_eq!(state, SOLVED_STATE);
    }

    #[test]
    fn test_scramble() {
        // U F' D' F2 D B2 D' R2 U' F2 R2 D2 R2 U' L B L R F' D B'
        let scramble = [
            U, F3, D3, F2, D, B2, D3, R2, U3, F2, R2, D2, R2, U3, L, B, L, R, F3, D, B3,
        ];
        let mut state = SOLVED_STATE;

        for m in scramble {
            state = state.apply_move(m);
        }

        let expected = State {
            cp: [DFL, UBL, DFR, UBR, UFL, DBR, DBL, UFR],
            co: [1, 2, 2, 0, 0, 0, 2, 2],
            ep: [UF, UR, DL, DB, BL, DF, UB, FL, UL, BR, FR, DR],
            eo: [0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        };

        assert_eq!(state, expected);
    }
}
