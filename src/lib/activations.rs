use std::f64::consts::E;


#[derive(Clone)]
pub struct Activations<'a> {
    pub function: &'a dyn Fn(f64) -> f64 ,
    pub derivative: &'a dyn Fn(f64) -> f64,
}


pub const SIGMOID: Activations = Activations {
    function:  &|x| 1.0/(1.0 + E.powf(-x)),
    derivative: &|x| x* (1.0 - x)
};
