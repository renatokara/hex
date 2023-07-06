use super::matrix::Matrix;

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biasis: Vec<Matrix>,
    data: Vec<Matrix>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Network {
        let mut weights = vec![];
        let mut biasis = vec![];

        for i in 0..layers.len() -1 {
            weights.push(Matrix::random(layers[i+1], layers[i]));
            biasis.push(Matrix::random(layers[i+1], 1));
        }

        Network {
            layers: layers, 
            weights: weights, 
            biasis: biasis, 
            data: vec![]
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {

    }
}