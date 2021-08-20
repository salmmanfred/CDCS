#[allow(unused_imports)]
use crate::common_traits::*;
use crate::ui_ext::FormatNote;
#[allow(unused_imports)]
use crate::{o, s};
#[allow(unused_imports)]
use fltk::{
    app::{self, Receiver, Sender},
    button::Button,
    frame::Frame,
    prelude::*,
    window::Window,
    input::Input,
};

#[derive(Debug, Clone)]
enum Message {
    Close,
    Name,
}

pub fn prov_register(str: &str) {
    let l = provr(str);
    println!("l{}l",l);

    //rt();
}

const WIND_WID: i32 = 200;
const WIND_HI: i32 = 160;

const TEXT_WID: i32 = 100;

const BUTTON_WID: i32 = 120;
const BUTTON_HI: i32 = 40;

fn provr(str: &str) -> String {
    //  let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("Note");
    //format!("Error happen: {}",
    let mut a = Frame::default()
        .with_size(TEXT_WID, 100)
        .with_label(&str.format_note(TEXT_WID, 4));
    a.set_pos((WIND_WID - TEXT_WID) / 2, 10);
    let name = Input::default().with_size(20, 40).with_label("-");

    let mut ok = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Ok");
    ok.set_pos((WIND_WID - BUTTON_WID) / 2, WIND_HI - BUTTON_HI - 2);

    wind.end();
    //wind.show();
    wind.show();
    let (s, r) = app::channel::<Message>();
    ok.emit(s, Message::Name);

    while wind.shown(){
        if let Some(msg) = r.recv() {
            match msg {
                Message::Close => {
                    wind.hide();
                }
                Message::Name =>{
                    return name.value()
                }
            }
        }
    }
    panic!("What");

    //app.run().unwrap_e("Error making the note window");
  
}
