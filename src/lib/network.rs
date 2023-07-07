use super::{matrix::Matrix, activations::Activations};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biasis: Vec<Matrix>,
    data: Vec<Matrix>,
    activations: Activations<'a>,
    learning_rate: f64
}

impl Network<'_> {
    pub fn new<'a>(layers: Vec<usize>, learning_rate: f64,  activations: Activations<'a>) -> Network {
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
            data: vec![],
            activations: activations,
            learning_rate: learning_rate 
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Invalid number of inputs");
        }

        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() -1 {
            current = self.weights[i]
            .multiply(&current)
            .addition(&self.biasis[i])
            .map(self.activations.function);
            //current = ((self.weights[i] * &current) + &self.biasis[i])
            //.map(self.activations.function);
        }

        current.data[0].to_owned()
    }

    pub fn back_propagation(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers[self.layers.len() -1] {
            panic!("Invalid Number of Targets");
        }

        let mut parsed = Matrix::from(vec![outputs]);
        let mut errors = Matrix::from(vec![targets]).subtract(&parsed);
        let mut gradients = parsed.map(self.activations.derivative);

        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients.dot_multiply(&errors).map(&|x| x* self.learning_rate);
            self.weights[i] = self.weights[i].addition(&gradients.multiply(&self.data[i].transpose()));
            self.biasis[i] = self.biasis[i].addition(&gradients);
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activations.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16) -> () {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs/100) == 0 {
                print!("Epoch {} of {}", i, epochs);
            }
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(inputs[j].clone());
                self.back_propagation(outputs, targets[j].clone());
            }
        }
    }


    pub fn l<'a, T>(f: &'a mut dyn Fn(&f64)->Vec<T>)->() {

    }
}