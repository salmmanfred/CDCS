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
pub trait Write {
    fn write_file(&self, path: &str, st: Settings);
}
impl Write for String {
    fn write_file(&self, path: &str, st: Settings) {
        let text = self.as_str();
        if !Path::new(path).exists() {
            openfile::write_file(path, text).unwrap_e("Error writing your file");
            return ();
        } else {
            if st.warn {
                if ask::ask(&format!(
                    "the file {} already exists do you want to over write it?",
                    path
                )) {
                    openfile::write_file(path, text).unwrap_e("Error writing your file");
                    return ();
                }
            } else {
                openfile::write_file(path, text).unwrap_e("Error writing your file");
                return ();
            }
        }
        println!("did not write file");
    }
}
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub warn: bool,
}
impl Settings {
    pub fn new() -> Settings {
        Settings { warn: true }
    }
    pub fn load() -> Option<Settings> {
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
        openfile::write_file(
            SAVE_FILE,
            &serde_json::to_string(self).unwrap_e("failed to create json"),
        )
        .unwrap_e("fucked up saving your settings sorry");
    }
    pub fn change(&mut self) {
        settings::settings(self);
    }
}
