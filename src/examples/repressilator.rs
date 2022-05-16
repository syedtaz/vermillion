use crate::system::System;

const KMU: f32 = 0.5;
const KMO: f32 = 0.0005;
const KP: f32 = 0.167;
const GAMMA_M: f32 = 0.005776;
const GAMMA_P: f32 = 0.001155;
const KR: f32 = 1.;
const KU1: f32 = 224.;
const KU2: f32 = 9.;

pub struct Repressilator {}

impl System for Repressilator {
    fn size(&self) -> usize {
        21
    }

    fn name(&self) -> &str {
        "repressilator"
    }

    fn update(&self, reaction: usize, state: &mut Vec<f32>) -> Result<(), usize> {
        match reaction {
            0 => {
                state[0] += 1.;
                Ok(())
            }
            1 => {
                state[2] += 1.;
                Ok(())
            }
            2 => {
                state[4] += 1.;
                Ok(())
            }
            3 => {
                state[1] += 1.;
                Ok(())
            }
            4 => {
                state[3] += 1.;
                Ok(())
            }
            5 => {
                state[5] += 1.;
                Ok(())
            }
            6 => {
                state[0] -= 1.;
                Ok(())
            }
            7 => {
                state[2] -= 1.;
                Ok(())
            }
            8 => {
                state[4] -= 1.;
                Ok(())
            }
            9 => {
                state[1] -= 1.;
                Ok(())
            }
            10 => {
                state[3] -= 1.;
                Ok(())
            }
            11 => {
                state[5] -= 1.;
                Ok(())
            }
            12 => {
                state[6] -= 1.;
                Ok(())
            }
            13 => {
                state[7] -= 1.;
                Ok(())
            }
            14 => {
                state[8] -= 1.;
                Ok(())
            }
            15 => {
                state[5] -= 1.;
                state[8] += 1.;
                Ok(())
            }
            16 => {
                state[1] -= 1.;
                state[6] += 1.;
                Ok(())
            }
            17 => {
                state[3] -= 1.;
                state[7] += 1.;
                Ok(())
            }
            18 => {
                state[5] += 1.;
                state[8] -= 1.;
                Ok(())
            }
            19 => {
                state[1] += 1.;
                state[6] -= 1.;
                Ok(())
            }
            20 => {
                state[3] += 1.;
                state[7] -= 1.;
                Ok(())
            }
            _ => Err(reaction),
        }
    }

    fn propensity(&self, proptbl: &mut Vec<f32>, state: &mut Vec<f32>) -> Result<(), usize> {
        proptbl[0] = if state[8] == 0. { KMU } else { KMO };
        proptbl[1] = if state[6] == 0. { KMU } else { KMO };
        proptbl[2] = if state[7] == 0. { KMU } else { KMO };
        proptbl[3] = KP * state[0];
        proptbl[4] = KP * state[2];
        proptbl[5] = KP * state[4];
        proptbl[6] = GAMMA_M * state[0];
        proptbl[7] = GAMMA_M * state[2];
        proptbl[8] = GAMMA_M * state[4];
        proptbl[9] = GAMMA_P * state[1];
        proptbl[10] = GAMMA_P * state[3];
        proptbl[11] = GAMMA_P * state[5];
        proptbl[12] = GAMMA_P * state[6];
        proptbl[13] = GAMMA_P * state[7];
        proptbl[14] = GAMMA_P * state[8];
        proptbl[15] = KR * state[5] * if state[8] < 2. { 1. } else { 0. };
        proptbl[16] = KR * state[1] * if state[6] < 2. { 1. } else { 0. };
        proptbl[17] = KR * state[3] * if state[7] < 2. { 1. } else { 0. };
        proptbl[18] = KU1 * if state[8] == 1. { 1. } else { 0. }
            + 2. * KU2 * if state[8] == 2. { 1.0 } else { 0. };
        proptbl[19] = KU1 * if state[6] == 1. { 1. } else { 0. }
            + 2. * KU2 * if state[6] == 2. { 1.0 } else { 0. };
        proptbl[20] = KU1 * if state[7] == 1. { 1. } else { 0. }
            + 2. * KU2 * if state[7] == 2. { 1.0 } else { 0. };
        Ok(())
    }
}
