use crate::ui_ext::{ask, err, note};
use openfile;
use std::fmt;
use std::path::Path;
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
                // note::note(err_mes, rt);
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
                // note::note(err_mes, rt);
                false
            }
        }
    }
}
pub trait Write {
    fn write_file(&self, path: &str);
}
impl Write for String {
    fn write_file(&self, path: &str) {
        let text = self.as_str();
        if !Path::new(path).exists() {
            openfile::write_file(path, text).unwrap_e("Error writing your file");
            return ();
        } else {
            if ask::ask(&format!(
                "the file {} already exists do you want to over write it?",
                path
            )) {
                openfile::write_file(path, text).unwrap_e("Error writing your file");
                return ();
            }
        }
        println!("did not write file");
    }
}
