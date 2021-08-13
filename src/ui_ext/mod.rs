pub mod err;
mod file;
pub mod note;
pub mod ui;

/*
i am not wasting more time doing &str.to_string()

*/
#[macro_export]
macro_rules! s {
    ($e:expr) => {
        $e.to_string()
    };
}
#[macro_export]
macro_rules! o {
    ($e:expr) => {
        $e.to_owned()
    };
}
