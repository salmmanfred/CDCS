use openfile;
use random_color::{Color, Luminosity, RandomColor};
use regex::Regex;
mod pop;
use crate::pop::pop_creator;

mod ui_ext;
#[derive(Debug)]
struct stateColl {
    states: Vec<Vec<(String, String)>>,
    name: Vec<String>,
    nation: String,
    population: String,
    religion: String,
    save: String,
    pub pop: pop_creator,
}
impl stateColl {
    pub fn new() -> stateColl {
        stateColl {
            states: Vec::new(),
            name: Vec::new(),

            nation: "the user forgot to enter a valid nation".to_string(),
            population: "the user forgot to enter a valid population name".to_string(),
            religion: "the user forgot to enter a valid religion".to_string(),
            save: "CDCSDEFAULT.CDCS".to_string(),
            pop: pop_creator::new(),
        }
    }
    pub fn register_states(&mut self, name: String) {
        for x in self.name.clone() {
            if x == name {
                return ();
            }
        }
        self.name.push(name);
        self.states.push(Vec::new());
    }
    pub fn register_prov(&mut self, name: [String; 3]) {
        for x in 0..self.name.len() {
            if self.name[x] == name[0] {
                self.states[x].push((name[1].clone(), name[2].clone()));
            }
        }
    }
    pub fn compile(&mut self) {
        let mut files: String = "".to_string();
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
                    let ar = table[x];
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
        openfile::write_file(&self.save, &files).expect("save error");
    }
    pub fn register_args(&mut self, args: Vec<String>) {
        let args = args.clone();
        self.nation = args[1].clone();
        self.population = args[2].clone();
        self.religion = args[3].clone();
        self.save = args[4].clone();
        self.pop.population = args[5].clone().parse().unwrap();
    }
}
use std::env;
mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        if args[1] != "ui" {
            let x = openfile::read_file(args[4].as_str());

            run(args, x);
            return;
        }
    }
    //ui::run_ui();
    ui_ext::ui::run();
}

// TODO: There should be a better way to do this
// not really sadly
fn name_to_ref_name(name: String) -> String {
    let mut st = name;
    st.make_ascii_lowercase();
    let st = st.replace("-", "_");
    let st = st.replace(" ", "_");
    let st = st.replace("à", "a");
    let st = st.replace("è", "e");
    let st = st.replace("ì", "i");
    let st = st.replace("ò", "o");
    let st = st.replace("ù", "u");

    let st = st.replace("á", "a");
    let st = st.replace("é", "e");
    let st = st.replace("í", "i");
    let st = st.replace("ó", "o");
    let st = st.replace("ú", "u");
    let st = st.replace("ý", "y");

    let st = st.replace("â", "a");
    let st = st.replace("ê", "e");
    let st = st.replace("î", "i");
    let st = st.replace("ô", "o");
    let st = st.replace("û", "u");

    let st = st.replace("ã", "a");
    //let st = st.replace("e", "e");
    //let st = st.replace("i", "i");
    let st = st.replace("õ", "o");
    //let st = st.replace("u", "u");
    let st = st.replace("ñ", "n");

    let st = st.replace("ä", "a");
    let st = st.replace("ë", "e");
    let st = st.replace("ï", "i");
    let st = st.replace("ö", "o");
    let st = st.replace("ü", "u");
    let st = st.replace("ÿ", "y");

    st
}

fn run(args: Vec<String>, data: String) {
    //(\((.* ?),( ?\w*)\)|(.*)\((.*\)))

    let mut col = Box::new(stateColl::new());
    col.register_args(args.clone());
    let x = data;
    let re = Regex::new(r#"\((.*),(.*),(\d*)\)"#).unwrap();

    for state in re.captures_iter(&x) {
        //println!("{:#?}", state);
        let st = name_to_ref_name(state[2].to_string());

        col.register_states(st.clone());
        let st2 = state[1].to_string();
        col.register_prov([
            st.clone(),
            st2.clone(),
            name_to_ref_name(state[1].to_string()),
        ]);
        col.pop
            .register((st2.clone(), state[3].parse::<u8>().unwrap()))
    }

    // println!("{:#?}", col);
    col.compile();
}
