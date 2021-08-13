mod file;
pub mod ui;
pub mod err;
pub mod note;

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
