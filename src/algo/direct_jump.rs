use crate::system::System;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

pub fn simulate(
    t_end: f32,
    network: &impl System,
    initial: Vec<f32>,
    granularity: f32,
) -> Result<Vec<Vec<f32>>, usize> {
    // Define initial state.
    let mut state = initial;
    let mut t = 0.;

    // Define time jumps
    let mut jumps: VecDeque<f32> = VecDeque::new();
    let mut queue_time = 0.;
    while queue_time <= t_end {
        jumps.push_back(queue_time);
        queue_time += granularity;
    }
    let mut cur_jump = jumps.pop_front().unwrap();

    // Define propensity and probability vectors.
    let size = network.size();
    let mut proptbl = vec![0.; size];
    let mut probtbl = vec![0.; size];

    // Defining random number generators.
    let mut r1_rng = thread_rng();
    let mut r2_rng = thread_rng();
    let mut r1: f32;
    let mut r2: f32;

    // Define temporary variables.
    let mut tau: f32;
    let mut j: usize;
    let mut prop_sum: f32;
    let mut prob_sum: f32;

    // Results vector
    let mut results: Vec<Vec<f32>> = Vec::new();
    let mut buffer = vec![0.; size + 1];
    let mut idx: usize;

    while t < t_end {
        // Calculate propensities for each reaction.
        network.propensity(&mut proptbl, &mut state).unwrap();

        // Calculate a_0
        prop_sum = proptbl.iter().sum();

        // Generate two independent uniform(0,1) random numbers r1 and r2.
        r1 = r1_rng.gen();
        r2 = r2_rng.gen();

        // Set tau = 1/a0 * ln(1/r1)
        tau = (1. / r1).ln() * (1. / prop_sum);

        // Compute discrete probabilities of each reaction
        for (i, item) in proptbl.iter().enumerate() {
            probtbl[i] = *item / prop_sum;
        }

        // Choose reaction.
        j = 0;
        prob_sum = 0.0;
        while prob_sum < r2 {
            prob_sum += probtbl[j];
            j += 1;
        }
        j -= 1;

        // Update state & time.
        network.update(j, &mut state).unwrap();
        t += tau;

        if t >= cur_jump {
            // Push results
            buffer[0] = t;
            idx = 1;
            for item in &state {
                buffer[idx] = *item;
                idx += 1;
            }
            results.push(buffer.clone());

            // Update time
            cur_jump = jumps.pop_front().unwrap();
        }
    }

    Ok(results)
}
