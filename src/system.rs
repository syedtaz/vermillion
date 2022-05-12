pub trait System {
    fn update(reaction: usize, state: &mut [f64]) -> Result<(), usize>;
    fn propensity(proptbl: &mut [f64], state: &mut [f64]) -> Result<(), usize>;
}
