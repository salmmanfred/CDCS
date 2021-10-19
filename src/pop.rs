// the point of this file is to house the pop distubution system

use crate::{common_traits::UnwrapA, o};
use std::collections::HashMap;
//use crate::ui_ext::err;


/*
this structs main job is to generate the pops and distrubute them according to the 
weight map.
*/


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
        //this adds the pop
        let (_, b) = add;
        // this adds to the total ammounts of weights
        self.amof += b as u64;
        // this adds the pop to a hashmap
        self.map.push(add);
    }

    pub fn compile(&mut self) -> &PopCreator {
        //this compiles the PopCreator
        // population / ammount of weights makes the enc
        // the enc is the acctuall weight moddifier that like makes sense
        let enc = self.population / self.amof;
        for (name, weight) in self.map.clone() {

            // the enc is then used here to get the fi aka the pop for the region

            let fi = (weight as u64 * enc.clone()) as f64;
            // println!("{} , {}  , {}",fi,enc.clone(), weight.clone());
            // the fi is then spread out all over the different classes 
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
        // finds the data
        o!(self
            .comp_hash
            .get(&name)
            .unwrap_e(&format!("{} not found", name)))

        // panic!("{} does not exist!", name);
    }
}
