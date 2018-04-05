

pub fn my_af(x: f64) -> f64 {
    if x > 0 {
        return x;
    }
    0
}

pub struct Neuron {
    weights: Vec<f64>,
    act_fucnction: fn(f64) -> f64,
}

impl Neuron {
    pub fn new(in_number: u32,) -> Self {
        Neuron {
            weights: vec![0; in_number], //todo random distribution
        }
    }

//    pub fn calc(input: Vec<f64>) -> f64 {
//        let sum = 0;
//        for i in 0..input.len() {
//            
//        }
//    }


}



pub struct Brain {

}


impl Brain {
    pub fn new() -> Self {
        Brain{
            weights: vec![0],
        }
    }



}
