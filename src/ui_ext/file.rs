use std::fs;
use crate::common_traits::*;

pub fn get_files() -> String {
    /*
    probably the coolest function

    it creates a stack and put some spacers on that

    then it looks into the current folder and gets each file name and after that
    */

    let mut list: Vec<String> = Vec::new();
    let mut list2: Vec<String> = Vec::new();

    for entry in fs::read_dir("./".to_string()).unwrap_e("Error reading folder") {
        // parses the folder into a folder struct
        /*
        gets the folder and its contents and then splits up it into <name of file> and <File extension>
        */
        let entry = entry
            .expect("error")
            .path()
            .into_os_string()
            .into_string()
            .expect("error");
        let entry_n = entry.split("/").collect::<Vec<&str>>(); //temp parse var
        let entry_n = entry_n[entry_n.len() - 1].split(".").collect::<Vec<&str>>(); // temp parse var

        /*
        here it puts it into 2 vectors that will keep control of the name and folder it also filters folders and files without extensions
        */
        //let entry_name = entry_n[0]; //final name
        let mut _entry_name2 = "str".to_string();
        if entry_n.len() >= 2 {
            _entry_name2 = vec![entry_n[0], entry_n[1]].join(".");
            let ent = _entry_name2.clone();
            list2.push(entry.clone());
            // con = con
            list.push(ent);
            // println!("x");
        }
        //println!("{:#?}, {}", entry_name2, entry_name);
    }
    let mut master = "".to_string();
    for x in 0..list.len() {
        /*
        iterates through the loop and looks at each name and makes a button out of it
        */
        master.push_str(&format!("{}\t|", list[x]))
    }
    /*

    it makes a scroll view and puts the stack into it
    */

    master
}
