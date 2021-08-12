
use crate::ui_ext::err;

//use crate::{s, o};

pub trait UnwrapA<T> {
    fn unwrap_e(self, err_mes: &str) -> T;
}

impl <T, E> UnwrapA<T> for Result<T, E>{
    fn unwrap_e(self, err_mes: &str) -> T {
        match self{
            Ok(x) =>{
                x
            }
            Err(_)=>{
                err::error(err_mes);
                panic!("There might have been an error displaying your error")
            }
        }
    }
}
impl <T> UnwrapA<T> for Option<T>{
    fn unwrap_e(self, err_mes: &str) -> T {
        match self{
            Some(x) =>{
                x
            }
            None=>{
                err::error(err_mes);
                panic!("There might have been an error displaying your error")
            }
        }
    }
}