use crate::o;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PopCreator {
    map: Vec<(String, u8)>,
    pub population: u64,
    amof: u64, // amounts of weights
    comp: Vec<(String, Vec<u64>)>,
    comp_hash: HashMap<String, Vec<u64>>,
}

impl PopCreator {
    pub fn new() -> PopCreator {
        PopCreator {
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

    pub fn compile(&mut self) -> &PopCreator {
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
        
        self
    }
    pub fn find(&self, name: String) -> Vec<u64> {
        return o!(match self.comp_hash.get(&name) {
            Some(x) => x,
            None => panic!("{} not found!", name),
        });
        // panic!("{} does not exist!", name);
    }
    
}

