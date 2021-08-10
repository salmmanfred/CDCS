mod file;
pub mod ui;
#[macro_export]
macro_rules! s {
    ($e:expr) => {
        $e.to_string()
    };
}
