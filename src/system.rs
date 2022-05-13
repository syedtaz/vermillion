pub trait System {
    fn update(&self, reaction: usize, state: &mut Vec<f32>) -> Result<(), usize>;
    fn propensity(&self, proptbl: &mut Vec<f32>, state: &mut Vec<f32>) -> Result<(), usize>;
    fn size(&self) -> usize;
    fn name(&self) -> &str;
}
