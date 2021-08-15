use crate::ui_ext::{err, note};
use std::fmt;

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
    fn unwrap_n(self, err_mes: &'static str, rt: Box<dyn Fn()>) -> bool;
}
impl<T, E> UnwrapN for Result<T, E> {
    fn unwrap_n(self, err_mes: &'static str, rt: Box<dyn Fn()>) -> bool {
        match self {
            Ok(_) => true,
            Err(_) => {
                note::note(err_mes, rt);
                false
            }
        }
    }
}
impl<T> UnwrapN for Option<T> {
    fn unwrap_n(self, err_mes: &'static str, rt: Box<dyn Fn()>) -> bool {
        match self {
            Some(_) => true,
            None => {
                note::note(err_mes, rt);
                false
            }
        }
    }
}
