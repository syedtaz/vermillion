use crate::system::System;

pub struct Protein {}

impl System for Protein {
    fn update(&self, reaction: usize, state: &mut Vec<f32>) -> Result<(), usize> {
        match reaction {
            0 => {
                state[0] -= 1.;
                Ok(())
            }
            _ => Err(reaction),
        }
    }
    fn propensity(&self, proptbl: &mut Vec<f32>, state: &mut Vec<f32>) -> Result<(), usize> {
        proptbl[0] = 1.0 * state[0];
        Ok(())
    }
    fn size(&self) -> usize {
        1
    }
    fn name(&self) -> &str {
        "protein"
    }
}
