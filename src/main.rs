#[allow(non_snake_case)]
#[macro_use]
extern crate glium;
use openfile;
use random_color::{Luminosity, RandomColor};
use regex::Regex;
mod pop;
mod graphics;
use crate::pop::PopCreator;
use std::collections::HashMap;
use std::error::Error;

mod ui_ext;
#[allow(unused_imports)]
use crate::ui_ext::err;
mod common_traits;
use crate::common_traits::*;
#[derive(Debug)]
struct StateColl {
    states: Vec<Vec<(String, String)>>,
    name: Vec<String>,
    nation: String,
    population: String,
    religion: String,
    save: String,
    name_hash: HashMap<String, usize>,

    pub pop: PopCreator,
}
impl StateColl {
    pub fn new() -> StateColl {
        StateColl {
            states: Vec::new(),
            name: Vec::new(),
            name_hash: HashMap::new(),
            nation: "the user forgot to enter a valid nation".to_string(),
            population: "the user forgot to enter a valid population name".to_string(),
            religion: "the user forgot to enter a valid religion".to_string(),
            save: "CDCSDEFAULT.CDCS".to_string(),
            pop: PopCreator::new(),
        }
    }
    pub fn register_states(&mut self, name: String) {
        match self.name_hash.get(&name) {
            Some(_) => return (),
            _ => {}
        }
        self.name.push(name.clone());
        self.name_hash.insert(name, self.name.len() - 1);
        self.states.push(Vec::new());
    }
    pub fn register_prov(&mut self, name: [String; 3]) {
        let x = self
            .name_hash
            .get(&name[0])
            .unwrap_e("Tried to register a province but failed horribly. ");
        self.states[o!(x)].push((name[1].clone(), name[2].clone()));
    }
    pub fn compile(&mut self) {
        let mut files: String = "".to_string();

        // compile the weight map
        let pop = self.pop.compile();

        for x in 0..self.name.len() {
            for st in 0..self.states[x].len() {
                let newfile = str::replace(
                    include_str!("template"),
                    "province_name",
                    self.name[x].as_str(),
                );

                /*
                gets the sanitized name (n3) and unsanitized name(n2)
                it then proceeds to replace different things in template to get the correct final output .
                when it has the final output it pushes it to file where it will later be written to the hard drive.

                */
                let (n2, n3) = self.states[x][st].clone();
                let newfile = str::replace(newfile.as_str(), "n2", &n2);
                let newfile = str::replace(newfile.as_str(), "n3", &n3);

                let newfile = str::replace(newfile.as_str(), "relg", self.religion.as_str());
                let newfile = str::replace(newfile.as_str(), "pop_name", self.population.as_str());
                let mut newfile =
                    str::replace(newfile.as_str(), "pop_nation", self.nation.as_str());

                // gets the population from the compiled pop struct.
                let table = ["L1", "L2", "L3", "L4", "L5", "L6", "L7", "L8", "L9"];
                let pop = pop.find(n2.clone());
                // it then applies this here
                for x in 0..9 {
                    newfile = str::replace(newfile.as_str(), table[x], pop[x].to_string().as_str());
                }
                //adds a random colour here
                let color = RandomColor::new()
                    .luminosity(Luminosity::Light)
                    .to_hex()
                    .replace("#", "0x");
                let newfile = str::replace(newfile.as_str(), "colour", color.as_str());

                //println!("{}", newfile);
                files.push_str(&newfile.as_str());
            }
        }
        openfile::write_file(&self.save, &files).unwrap_e("Error writing your file");
    }
    pub fn register_args(&mut self, args: Vec<String>) {
        let args = args.clone();
        self.nation = args[1].clone();
        self.population = args[2].clone();
        self.religion = args[3].clone();
        self.save = args[4].clone();
        self.pop.population = args[5]
            .clone()
            .parse()
            .unwrap_e("Population must be number");
    }
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        if args[1] != "ui" {
            run(args.clone(), &args[4])
                .map_err(|e| println!("{}", e))
                .ok();
            return;
        }
    }

    ui_ext::ui::run();
}

fn name_to_ref_name(name: String) -> String {
    let mut st = name;
    st.make_ascii_lowercase();
    let mut st2: String = s!("");
    for x in st.chars() {
        st2.push(match x {
            '-' => '_',
            ' ' => '_',
            'à' => 'a',
            'è' => 'e',
            'ì' => 'i',
            'ù' => 'u',
            'ó' => 'o',
            'ú' => 'u',
            'ý' => 'y',
            'â' => 'a',
            'ê' => 'e',
            'î' => 'i',
            'ô' => 'o',
            'û' => 'u',
            'ã' => 'a',
            'õ' => 'o',
            'ñ' => 'n',
            'ä' => 'a',
            'å' => 'a',
            'ë' => 'e',
            'ï' => 'i',
            'ö' => 'o',
            'ü' => 'u',
            'ÿ' => 'y',

            _ => x,
        });
    }

    st2
}

fn run(args: Vec<String>, path: &String) -> Result<(), Box<dyn Error>> {
    //(\((.* ?),( ?\w*)\)|(.*)\((.*\)))

    let data =
        std::fs::read_to_string(&path).map_err(|_| ErrorMsg("Failed to read input file".into()))?;

    let mut col = Box::new(StateColl::new());
    args[5]
        .clone()
        .parse::<u64>()
        .map_err(|_| ErrorMsg("Population must be number".into()))?;

    col.register_args(args.clone());
    let re = Regex::new(r#"\((.*),(.*),(\d*)\)"#)?;

    for state in re.captures_iter(&data) {
        let st = name_to_ref_name(state[2].to_string());

        col.register_states(st.clone());
        let st2 = state[1].to_string();
        col.register_prov([
            st.clone(),
            st2.clone(),
            name_to_ref_name(state[1].to_string()),
        ]);
        col.pop.register((
            st2.clone(),
            state[3]
                .parse::<u8>()
                .unwrap_e(&format!("{}s weight is a invalid number!", &st)),
        ))
    }

    col.compile();
    println!("done");
    Ok(())
}
