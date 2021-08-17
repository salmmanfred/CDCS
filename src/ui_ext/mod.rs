mod file;
pub mod popups;
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
pub trait FormatNote {
    fn format_note(&self, wid: i32, font: i32) -> String;
}
impl FormatNote for str {
    fn format_note(&self, wid: i32, font: i32) -> String {
        let wid = wid / font;

        let mut v: Vec<Vec<char>> = Vec::new();
        let mut mess = self.chars().collect::<Vec<char>>();
        println!("{}, {}", mess.len(), wid as usize);

        while mess.len() >= wid as usize {
            println!("2 {}, {}", mess.len(), wid as usize);

            if mess.len() >= wid as usize {
                let mut split = wid as usize;
                while mess[split - 1] != ' ' {
                    split -= 1;
                }

                let mut x = mess[0..split].to_vec();
                x.push('\n');
                v.push(x);

                mess = mess[split..mess.len()].to_vec();
            }
        }
        if mess.len() >= 1 && mess.len() <= wid as usize {
            let mut x = mess[0..mess.len()].to_vec();
            x.push('\n');
            v.push(x);
        }
        let mut string = s!("");
        for x in v {
            string.push_str(&x.iter().collect::<String>())
        }

        string
    }
}
