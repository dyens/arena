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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LayerType {
    IN,
    OUT,
    INNER
}


#[derive(Debug)]
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
            neurons.push(Neuron::new(in_number, act_function));
        }
//        for _ in 0..n_neurons {
//            match layer_type {
//                LayerType::IN => {
//                    neurons.push(Neuron::new(1, act_function));
//                }
//                _ => {
//                    neurons.push(Neuron::new(in_number, act_function));
//                }
//            }
//        }

        Layer{
            neurons: neurons,
            layer_type: layer_type,
            next: None
        }
    }

    pub fn mutate_layer(&mut self) {
        let mut rng = thread_rng();
        let n_mutations = rng.gen_range(0, self.neurons.len());
        for _ in 0..n_mutations {
            let mutation_index = rng.gen_range(0,
                                               self.neurons.len());
            self.neurons[mutation_index as usize].mutate();
        }

    }

    pub fn mutate(&mut self) {
        self.mutate_layer();
        //TODO: http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
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

//        match self.layer_type {
//            LayerType::IN => {
//                assert!(self.neurons.len() == input.len(),
//                        "neuros={:?}, input={:?}",
//                        self.neurons.len(),
//                        input.len());
//                for i in 0..input.len() {
//                    output.push(self.neurons[i]
//                                .calc(&[input[i], ]));
//                }
//            }
//            _ => {
//                for neuron in &self.neurons {
//                    output.push(neuron.calc(input));
//                }
//            }
//        }
        for neuron in &self.neurons {
            let a = neuron.calc(input);
            output.push(a);
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
    bullet_layer: Layer,
    tank_layer: Layer,
}


impl Brain {

    pub fn new(
        in_bullet: usize,
        out_bullet: usize,
        inner_bullet: usize,

        in_tank: usize,
        out_tank: usize,
        inner_tank: usize,

        out_layer: usize,
        inner_layer:usize,
    ) -> Self {


        let in_bullet = in_bullet + out_bullet + 1;
        let in_tank = in_tank + out_tank + 1;


        let mut bullet_layer = Layer::new(
            in_bullet,
            in_bullet,
            LayerType::IN,
            htan_af);

        for _ in 0..inner_bullet {
            bullet_layer.push_layer(
                in_bullet,
                in_bullet + 1,
                LayerType::INNER,
                htan_af);
        }

        bullet_layer.push_layer(
            out_bullet,
            in_bullet + 1,
            LayerType::OUT,
            sig_af);


        let mut tank_layer = Layer::new(
            in_tank,
            in_tank,
            LayerType::IN,
            htan_af);

        for _ in 0..inner_tank {
            tank_layer.push_layer(
                in_tank,
                in_tank + 1,
                LayerType::INNER,
                htan_af);
        }

        tank_layer.push_layer(
            out_tank,
            in_tank + 1,
            LayerType::OUT,
            sig_af);

        let in_layer = out_tank + out_bullet;
        let mut layer = Layer::new(
            in_layer,
            in_layer,
            LayerType::IN,
            htan_af);

        for _ in 0..inner_layer {
            layer.push_layer(
                in_layer,
                in_layer + 1,
                LayerType::INNER,
                htan_af);
        }

        layer.push_layer(
            out_layer,
            in_layer + 1,
            LayerType::OUT,
            sig_af);



        Brain {
            bullet_layer: bullet_layer,
            tank_layer: tank_layer,
            layer: layer
        }
    }

    pub fn mutate(&mut self) {
        self.bullet_layer.mutate();
        self.tank_layer.mutate();
        self.layer.mutate();
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
            let res = self.tank_layer.calc(&tank_out);
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
            let res = self.bullet_layer.calc(bullet_out.as_slice());
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


        let output = self.layer.calc(in_data.as_slice());

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
