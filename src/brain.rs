use rand::{Rng, thread_rng};



use tank::TankAction;

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

#[derive(Debug, Clone)]
pub struct Neuron {
    weights: Vec<f64>,
    act_function: fn(f64) -> f64,
}

impl Neuron {

    pub fn gen_weights(weights_number: usize) -> Vec<f64> {
        let mut rng = thread_rng();
        let mut weights = Vec::with_capacity(weights_number);
        for _ in 0..weights_number {
            weights.push(rng.gen_range(-10.0, 10.0)); //TODO random float....
        }
        weights
    }

    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        let n_mutations = rng.gen_range(0, self.weights.len());
        for _ in 0..n_mutations {
            let mutation_index = rng.gen_range(0,
                                               self.weights.len());
            self.weights[mutation_index as usize] = rng.gen_range(-10.0, 10.0);
        }

    }

    pub fn new(in_number: usize,
               act_function: fn(f64) -> f64) -> Self {
        let weights = Neuron::gen_weights(in_number);

        Neuron {
            weights: weights, //todo random distribution
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

//        println!("input {:?} weights: {:?}, sum: {:?}", &input, &self.weights, &sum);
        let result = (self.act_function)(sum);
        result
    }
}

#[derive(Debug, Clone)]
pub enum LayerType {
    IN,
    OUT,
    INNER
}


#[derive(Debug, Clone)]
pub struct Layer {
    neurons: Vec<Neuron>,
    layer_type: LayerType,
}

impl Layer {
    pub fn new(n_neurons: usize,
               in_number: usize,
               layer_type: LayerType,
               act_function: fn(f64) -> f64,
    ) -> Self {
        let mut neurons = Vec::with_capacity(n_neurons);
        for _ in 0..n_neurons {
            neurons.push(Neuron::new(in_number, act_function));
        }

        Layer{
            neurons: neurons,
            layer_type: layer_type,
        }
    }

    pub fn fuck(&self, partner: &Layer) -> (Layer, Layer) {
        assert!(self.neurons.len() == partner.neurons.len());
        let middle = self.neurons.len() / 2;
        let mut neurons = Vec::with_capacity(self.neurons.len());
        neurons.clone_from_slice(&self.neurons[0..middle]);
        neurons.extend_from_slice(
            &partner.neurons[middle..partner.neurons.len()]
        );
        let l1 = Layer {
            neurons: neurons,
            layer_type: self.layer_type.clone()
        };

        let mut neurons = Vec::with_capacity(self.neurons.len());
        neurons.clone_from_slice(&partner.neurons[0..middle]);
        neurons.extend_from_slice(
            &self.neurons[middle..self.neurons.len()]
        );
        let l2 = Layer {
            neurons: neurons,
            layer_type: self.layer_type.clone()
        };
        (l1, l2)
    }

    pub fn mutate(&mut self) {
        let mut rng = thread_rng();
        let n_mutations = rng.gen_range(0, self.neurons.len());
        for _ in 0..n_mutations {
            let mutation_index = rng.gen_range(0,
                                               self.neurons.len());
            self.neurons[mutation_index as usize].mutate();
        }

    }

    pub fn calc(&self, input: &[f64]) -> Vec<f64> {
        let mut output: Vec<f64>;


        match self.layer_type {
            LayerType::OUT => {
                output = Vec::with_capacity(self.neurons.len());}
            _ => {
                output = Vec::with_capacity(self.neurons.len() + 1);}
        }

        for neuron in &self.neurons {
            let a = neuron.calc(input);
            output.push(a);
        }


        match self.layer_type {
            LayerType::OUT => {}
            _ => {output.push(1.0)}
        }
        output
    }

}


#[derive(Debug)]
pub struct LayersChain {
    layers: Vec<Layer>
}


impl LayersChain {
    pub fn new(description: &Vec<usize>) -> Self {
        // descrition - neuron numbers in  layers
        // simple example 6,7,5
        assert!(description.len() >= 2);
        let mut layers = Vec::with_capacity(description.len());

        let mut n_inputs = description[0];
        let layer = Layer::new(n_inputs,
                               n_inputs,
                               LayerType::IN,
                               htan_af);
        layers.push(layer);
        n_inputs += 1;
        for neuron_number in &description[1..description.len()-1] {
            let layer = Layer::new(*neuron_number,
                                   n_inputs,
                                   LayerType::INNER,
                                   htan_af);
            layers.push(layer);
            n_inputs = neuron_number + 1;
        }
        let n_outputs = description[description.len() - 1];
        let layer = Layer::new(n_outputs,
                               n_inputs,
                               LayerType::OUT,
                               sig_af);
        layers.push(layer);
//        println!("{:?}", layers);
        LayersChain{layers}
    }

    pub fn fuck(&self, partner: &LayersChain)
                -> (LayersChain, LayersChain) {
        assert!(self.layers.len() == partner.layers.len());
        let mut ls1 = Vec::with_capacity(self.layers.len());
        let mut ls2 = Vec::with_capacity(self.layers.len());
        for i in 0..self.layers.len() {
            let (nl1, nl2 ) = self.layers[i].fuck(&partner.layers[i]);
            ls1.push(nl1);
            ls2.push(nl2);
        }
        (LayersChain{layers: ls1}, LayersChain{layers: ls2})
    }


    pub fn mutate(&mut self) {
        for layer in &mut self.layers {
            layer.mutate()
        }
    }

    pub fn calc(&self, input: &[f64]) -> Vec<f64> {
        let mut output  = input.to_vec();
        for layer in &self.layers {
            output = layer.calc(&output);
        }
        output.to_vec()
    }

}



pub struct Brain {
    result_chain: LayersChain,
    bullet_chain: LayersChain,
    tank_chain: LayersChain,
}


impl Brain {

    pub fn new(
        in_bullet: usize,
        out_bullet: usize,
        in_tank: usize,
        out_tank: usize,
        out:usize,
    ) -> Self {
        let bullet_chain = LayersChain::new(
            &[in_bullet  + out_bullet + 1,
             in_bullet + out_bullet + 1,
             out_bullet].to_vec()
        );
        let tank_chain = LayersChain::new(
            &[in_tank  + out_tank + 1,
             in_tank + out_tank + 1,
             out_tank].to_vec()
        );
        let result_chain = LayersChain::new(
            &[out_tank + out_bullet,
             out_tank + out_bullet, out].to_vec()
        );

        Brain {result_chain, bullet_chain, tank_chain}
    }

    pub fn fuck(&self, partner: &Brain)
                -> (Brain, Brain)
    {
        let (rc1, rc2) = self.result_chain
            .fuck(&partner.result_chain);
        let (bc1, bc2) = self.bullet_chain
            .fuck(&partner.bullet_chain);
        let (tc1, tc2) = self.tank_chain
            .fuck(&partner.tank_chain);
        (Brain {
            result_chain: rc1,
            bullet_chain: bc1,
            tank_chain: tc1,
        },
         Brain {
             result_chain: rc2,
             bullet_chain: bc2,
             tank_chain: tc2,
         },
        )

    }

    pub fn mutate(&mut self) {
        self.bullet_chain.mutate();
        self.tank_chain.mutate();
        self.result_chain.mutate();
    }

    pub fn normalize(in_v: &Vec<[f64; 6]>) -> Vec<[f64; 6]> {

        let max_abs_values = in_v.iter().fold(
            [0.; 6], |mut acc, &i| {
                for j in 0..acc.len() {
                    if acc[j] < i[j].abs() {acc[j] = i[j].abs()};
                }
                acc
            });
        in_v.iter().map(|x| {
            let mut out = [0.;6];
            for j in 0..x.len() {
                if max_abs_values[j] == 0.0 {
                    continue;
                }
                out[j] = x[j] / max_abs_values[j];
            }
            out
        }).collect()
    }


    pub fn calc(&self,
                tanks: &Vec<[f64; 6]>,
                bullets: &Vec<[f64; 6]>) -> TankAction {

        let tanks = Brain::normalize(&tanks);
        let bullets = Brain::normalize(&bullets);

        let mut tank_out = vec![0.0; 12];
        for (i, tank) in tanks.iter().enumerate() {
            tank_out[0] = i as f64;
            tank_out[1] = tank[0];
            tank_out[2] = tank[1];
            tank_out[3] = tank[2];
            tank_out[4] = tank[3];
            tank_out[5] = tank[4];
            tank_out[6] = tank[5];
            let res = self.tank_chain.calc(&tank_out);
            tank_out[7] = res[0];
            tank_out[8] = res[1];
            tank_out[9] = res[2];
            tank_out[10] = res[3];
            tank_out[11] = res[4];
        }

        let mut bullet_out = vec![0.0; 12];
        for (i, bullet) in bullets.iter().enumerate() {
            bullet_out[0] = i as f64;
            bullet_out[1] = bullet[0];
            bullet_out[2] = bullet[1];
            bullet_out[3] = bullet[2];
            bullet_out[4] = bullet[3];
            bullet_out[5] = bullet[4];
            bullet_out[6] = bullet[5];
            let res = self.bullet_chain.calc(bullet_out.as_slice());
            bullet_out[7] = res[0];
            bullet_out[8] = res[1];
            bullet_out[9] = res[2];
            bullet_out[10] = res[3];
            bullet_out[11] = res[4];
        }

        let in_data = vec![
            tank_out[7],
            tank_out[8],
            tank_out[9],
            tank_out[10],
            tank_out[11],
            bullet_out[7],
            bullet_out[8],
            bullet_out[9],
            bullet_out[10],
            bullet_out[11],
        ];


        let output = self.result_chain.calc(in_data.as_slice());

        let max_value = output.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        let max_index = match max_value {
            Some(value) => output.iter().position(|x| x == value),
            None => None,
        };
        let max_index = match max_index {
            Some(value) => value,
            None => 0
        };
        match max_index {
            0 => TankAction::FWD,
            1 => TankAction::ROT,
            2 => TankAction::UROT,
            3 => TankAction::FIRE,
            4 => TankAction::STOP,
            _ => TankAction::FWD,
        }

    }

}
