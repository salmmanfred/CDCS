use fltk::{
    app, button::Button, frame::Frame, prelude::*,
    window::Window,
};
#[allow(unused_imports)]
use crate::{s, o};

#[derive(Debug,Clone)]
enum Message{
    Close,
}


pub fn error(str: &str){
    err_pop(str);
    panic!("Err: {}",str);
}

const WIND_WID: i32 = 200;
const WIND_HI: i32 = 160;

const TEXT_WID: i32 = 100;

const BUTTON_WID: i32 = 120;
const BUTTON_HI: i32 = 40;

fn err_pop(str: &str){
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("CDCS - Error");
    //format!("Error happen: {}",
    let mut a = Frame::default().with_size(TEXT_WID, 100).with_label(&str.format_er(TEXT_WID, 4));
    a.set_pos((WIND_WID - TEXT_WID) / 2,10);
    let mut ok = Button::default().with_size(BUTTON_WID, BUTTON_HI).with_label("Ok");
    ok.set_pos((WIND_WID - BUTTON_WID) / 2, WIND_HI - BUTTON_HI - 2);





    wind.end();
    wind.show();
    let (s, r) = app::channel::<Message>();
    ok.emit(s, Message::Close);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Close => {
                app.quit();
                }
            }
        }
    }
    app.run().unwrap();

}

trait FormatErr{
    fn format_er (&self, wid: i32, font: i32) -> String;
}
impl FormatErr for str{
    fn format_er(&self, wid: i32, font: i32) -> String {
        let wid = wid/font;

        let mut v: Vec<Vec<char>> = Vec::new();
        let mut mess = self.chars().collect::<Vec<char>>();
        println!("{}, {}",mess.len(), wid as usize);

        while mess.len() >= wid as usize{
            println!("2 {}, {}",mess.len(), wid as usize);

            if mess.len() >= wid as usize{
                let mut split = wid as usize;
                while mess[split] != ' '{
                    split -= 1;
                }

                let mut x = mess[0..split].to_vec();
                x.push('\n');
                v.push(x);

                mess = mess[split..mess.len()].to_vec();
                
            }
        }
        if mess.len() >= 1 && mess.len() <= wid as usize{
            let mut x = mess[0..mess.len()].to_vec();
            x.push('\n');
            v.push(x);
        }
        let mut string = s!("");
        for x in v{
            string.push_str(&x.iter().collect::<String>())
        }
        
        string
    }
}