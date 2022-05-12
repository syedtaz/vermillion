use crate::system::System;

struct Protein {
    state: [f32; 1],
    /*
    Reaction 1: a -> null
    */
}

impl System for Protein {
    fn update(reaction: usize, state: &mut [f64]) -> Result<(), usize> {
        match reaction {
            0 => {
                state[0] -= 1.;
                Ok(())
            }
            _ => Err(reaction),
        }
    }
    fn propensity(proptbl: &mut [f64], _state: &mut [f64]) -> Result<(), usize> {
        proptbl[0] = 1.0;
        Ok(())
    }
}
