/*
Function of this file is to add some simple macros and also add all the ui stuff.

*/

mod file;
pub mod popups;
pub mod ui;
/*
i am not wasting more time doing &str.to_string()
    or .to_owned()
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
    /*
    Format note takes a max width size and a font size and formats a str to
    a string but with the correct places for \n


    */

    fn format_note(&self, wid: i32, font: i32) -> String {
        // here it takes the given wdith and devides it by the font size to be able get the "real"
        // width
        let wid = wid / font;

        //V is the vector holding vectors of chars
        // these vectors are the split up "words" and it holds them where there should be a \n
        // so if you have the str "test in the sky with a plane" it splits at "sky " for example
        // then it loads the first part "test in the sky" in to V[0] and the rest in V[1]
        let mut v: Vec<Vec<char>> = Vec::new();
        let mut mess = self.chars().collect::<Vec<char>>();
        println!("{}, {}", mess.len(), wid as usize);

        while mess.len() >= wid as usize {
            println!("2 {}, {}", mess.len(), wid as usize);

            if mess.len() >= wid as usize {
                /*
                it takes the split value which should be the width
                but if the split value is in the middle of a word
                it splits at the closest space.

                */
                let mut split = wid as usize;
                while mess[split - 1] != ' ' {
                    split -= 1;
                }
                // it then splits the mess vector aka the string in Vec<char>
                //after it splits it pushes a \n to the split vector and then pushes it to
                // the V vector
                let mut x = mess[0..split].to_vec();
                x.push('\n');
                v.push(x);
                // after that it removes the already split of vector
                mess = mess[split..mess.len()].to_vec();
            }
        }
        // if the string is more than 1 char and less than width it just pushes it to v and
        // calls it a day
        if mess.len() >= 1 && mess.len() <= wid as usize {
            let mut x = mess[0..mess.len()].to_vec();
            x.push('\n');
            v.push(x);
        }
        // crates a string using the s!("") macro and then collects the V vector into it
        let mut string = s!("");
        for x in v {
            string.push_str(&x.iter().collect::<String>())
        }

        string
    }
}
