

pub fn relu_af(x: f64) -> f64 {
    if x > 0.0 {
        return x;
    }
    0.0
}

pub fn htan_af(x: f64) -> f64 {
    let ef = x.exp();
    let emf = (-x).exp();
    (ef - emf) / (ef + emf)
}

pub fn sig_af(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct Neuron {
    weights: Vec<f64>,
    act_function: fn(f64) -> f64,
}

impl Neuron {
    pub fn new(in_number: usize,
               act_function: fn(f64) -> f64) -> Self {
        Neuron {
            weights: vec![0.5; in_number], //todo random distribution
            act_function: act_function,
        }
    }

    pub fn calc(&self, input: &[f64]) -> f64 {
        assert!(input.len() == self.weights.len(),
                "input_len: {:?} weights_num: {:?}",
                input.len(),
                self.weights.len()
        );
        let mut sum = 0.0;
        for i in 0..input.len() {
            sum += input[i] * self.weights[i];
        }
        (self.act_function)(sum)
    }
}

pub enum LayerType {
    IN,
    OUT,
    INNER
}

pub struct Layer {
    neurons: Vec<Neuron>,
    layer_type: LayerType,
    next: Option<Box<Layer>>,
}


impl Layer {
    pub fn new(n_neurons: usize,
               in_number: usize,
               layer_type: LayerType,
               act_function: fn(f64) -> f64,
    ) -> Self {
        let mut neurons = Vec::with_capacity(n_neurons);
        for _ in 0..n_neurons {
            match layer_type {
                LayerType::IN => {
                    neurons.push(Neuron::new(1, act_function));
                }
                _ => {
                    neurons.push(Neuron::new(in_number, act_function));
                }
            }
        }

        Layer{
            neurons: neurons,
            layer_type: layer_type,
            next: None
        }
    }

    pub fn push_layer(
        &mut self,
        n_neurons: usize,
        in_number: usize,
        layer_type: LayerType,
        act_function: fn(f64) -> f64) {
        self.next = Some(Box::new(Layer::new(n_neurons,
                                        in_number,
                                        layer_type,
                                        act_function)));
    }

    pub fn calc(&self, input: &[f64]) -> Vec<f64> {
        let mut output: Vec<f64>;

        match self.layer_type {
            LayerType::OUT => {
                output = Vec::with_capacity(self.neurons.len());}
            _ => {
                output = Vec::with_capacity(self.neurons.len() + 1);}
        }

        match self.layer_type {
            LayerType::IN => {
                assert!(self.neurons.len() == input.len(),
                        "neuros={:?}, input={:?}",
                        self.neurons.len(),
                        input.len());
                for i in 0..input.len() {
                    output.push(self.neurons[i]
                                .calc(&[input[i], ]));
                }
            }
            _ => {
                for neuron in &self.neurons {
                    output.push(neuron.calc(input));
                }
            }
        }


        match self.layer_type {
            LayerType::OUT => {}
            _ => {output.push(1.0)}
        }

        match &self.next {
            &Some(ref next) => next.calc(output.as_slice()),
            &None => output

        }
    }

}


pub struct Brain {
    layer: Layer,
}


impl Brain {
    pub fn new(in_params: usize,
               out_params: usize,
               inner_layers: usize) -> Self {

        let mut layer = Layer::new(
            in_params,
            in_params,
            LayerType::IN,
            htan_af);

        for _ in 0..inner_layers {
            layer.push_layer(
                in_params,
                in_params + 1,
                LayerType::INNER,
                htan_af);
        }

        layer.push_layer(
            out_params,
            in_params + 1,
            LayerType::OUT,
            sig_af);

        Brain {
            layer: layer
        }
    }

    pub fn calc(&self, input: &Vec<f64>) -> Vec<f64> {
        self.layer.calc(input.as_slice())
        //array.iter().enumerate().max_by(|&(_, item)| item)
    }



}
