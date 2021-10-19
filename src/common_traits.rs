/*
The point of this file is to house common traits. 
this is so you can easily import it to all files

*/



use crate::ui_ext::popups::{ask, err, note, settings};
use openfile;
use serde_json::json;
use std::fmt;
use std::path::Path;
const SAVE_FILE: &'static str = "./settings";
//use crate::{s, o};

// A custom error to make readable errors for the user
#[derive(Debug, Clone)]
pub struct ErrorMsg(pub String);

impl fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ErrorMsg {}




/*
UnwrapA is a trait for result and option 
what it does is very simple instead of shutting down the entire app 
it opens a window telling you the error otherwise it just returns the data
*/
pub trait UnwrapA<T> {
    fn unwrap_e(self, err_mes: &str) -> T;
}

impl<T, E> UnwrapA<T> for Result<T, E> {
    fn unwrap_e(self, err_mes: &str) -> T {
        match self {
            Ok(x) => x,
            Err(_) => {
                err::error(err_mes);
                panic!("There might have been an error displaying your error")
            }
        }
    }
}
impl<T> UnwrapA<T> for Option<T> {
    fn unwrap_e(self, err_mes: &str) -> T {
        match self {
            Some(x) => x,
            None => {
                err::error(err_mes);
                panic!("There might have been an error displaying your error")
            }
        }
    }
}

/*
same as unwrapA but this shows a note instead of an error that then closes the screen
this also only returns a bool

*/

pub trait UnwrapN {
    fn unwrap_n(self, err_mes: &'static str) -> bool;
}
impl<T, E> UnwrapN for Result<T, E> {
    fn unwrap_n(self, err_mes: &'static str) -> bool {
        match self {
            Ok(_) => true,
            Err(_) => {
                note::note(err_mes);
                false
            }
        }
    }
}
impl<T> UnwrapN for Option<T> {
    fn unwrap_n(self, err_mes: &'static str) -> bool {
        match self {
            Some(_) => true,
            None => {
                note::note(err_mes);
                false
            }
        }
    }
}

/*

write is a simple trait for String 
that does something very simple.
it takes the string data and saves it to a file
however if the file exists it shows a confermation screen.
*/
pub trait Write {
    fn write_file(&self, path: &str, st: Settings);
}
impl Write for String {
    fn write_file(&self, path: &str, st: Settings) {
        let text = self.as_str();
        /*
        checks the path given.
        if it exists it will open an ask window(ui_ext/popups/ask.rs)
        if the person then accepts it will re write the file.
        if the person does not care about warnings it will simply skip this check.
        */
        if !Path::new(path).exists()
            || !st.warn
            || ask::ask(&format!(
                "the file {} already exists do you want to over write it?",
                path
            ))
        {
            // writes the file and returns
            openfile::write_file(path, text).unwrap_e("Error writing your file");
            return ();
        }

        println!("did not write file");
    }
}
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub warn: bool,
    pub debug: bool,
}
impl Settings {
    pub fn new() -> Settings {
        Settings {
            warn: true,
            debug: false,
        }
    }
    pub fn load() -> Option<Settings> {
        // check if save_file exists and if not returns none otherwise the settings struct 
        if Path::new(SAVE_FILE).exists() {
            let json = openfile::read_file(SAVE_FILE);
            let x = json!(json);

            return Some(serde_json::from_str(&json).unwrap_e("failed to read error"));
            /*Settings{
                warn: x["warn"].to_string().parse::<bool>().unwrap_e("failed to read settings: warn"),
            })*/
        }

        None
    }
    pub fn save(&self) {

        // writes the save file as json thanks to serde_json Deserialize funtion
        // it can later read this if the settings struct has not changed
        openfile::write_file(
            SAVE_FILE,
            &serde_json::to_string(self).unwrap_e("failed to create json"),
        )
        // and uhh yeah 
        .unwrap_e("fucked up saving your settings sorry");
    }
    pub fn change(&mut self) {
        // starts the settings in ui_ext/popups/settings.rs
        settings::settings(self);
    }
}
