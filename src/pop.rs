use std::collections::HashMap;
use crate::o;

#[derive(Debug)]
pub struct pop_creator {
    map: Vec<(String, u8)>,
    pub population: u64,
    amof: u64, // amounts of weights
    comp: Vec<(String, Vec<u64>)>,
    comp_hash:  HashMap<String, Vec<u64>>
}

impl pop_creator {
    pub fn new() -> pop_creator {
        pop_creator {
            map: Vec::new(),
            population: 0,
            amof: 0,
            comp: Vec::new(),
            comp_hash: HashMap::new(),
        }
    }
    pub fn register(&mut self, add: (String, u8)) {
        let (_, b) = add;
        self.amof += b as u64;
        self.map.push(add);
    }

    pub fn compile(&mut self) -> &pop_creator {
        let enc = self.population / self.amof;
        for (name, weight) in self.map.clone() {
            let fi = (weight as u64 * enc.clone()) as f64;
            // println!("{} , {}  , {}",fi,enc.clone(), weight.clone());
            let spread = vec![
                (fi * 0.04).ceil() as u64,
                (fi * 0.32).ceil() as u64,
                (fi * 0.06).ceil() as u64,
                (fi * 0.04).ceil() as u64,
                (fi * 0.04).ceil() as u64,
                (fi * 0.04).ceil() as u64,
                (fi * 0.04).ceil() as u64,
                (fi * 0.40).ceil() as u64,
                (fi * 0.02).ceil() as u64,
            ];
            self.comp_hash.insert(name, spread);
        }
        //println!("{:#?}", self.comp);
        self
    }
    pub fn find(&self, name: String) -> Vec<u64> {
        use std::time::Instant;
        let now = Instant::now();

        /*for (x, x2) in self.comp.clone() {
            if x == name {

                return x2;
            }
        }*/
        println!("Extime: {}", now.elapsed().as_nanos());
        return o!(self.comp_hash.get(&name).unwrap())

       // panic!("{} does not exist!", name);
    }
}
