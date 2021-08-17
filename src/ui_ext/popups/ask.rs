#[allow(unused_imports)]
use crate::common_traits::*;
#[allow(unused_imports)]
use crate::ui_ext::popups::{err, note};
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
};

#[derive(Debug, Clone)]
enum Message {
    Yes,
    No,
}

pub fn ask(str: &str) -> bool {
    let (mut wind, mut b1, mut b2) = ask_pop(str);
    let (s, r) = app::channel::<Message>();
    b1.emit(s.clone(), Message::Yes);
    b2.emit(s, Message::No);

    wind.show();

    while wind.shown() {
        //println!("shit {:#?}",r.recv());
        app::wait();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Yes => {
                    wind.hide();
                    return true;
                }
                Message::No => {
                    wind.hide();
                    return false;
                }
            }
        }
    }

    err::error("how");
    false

    //false
    //rt();
}

const WIND_WID: i32 = 200;
const WIND_HI: i32 = 160;

const TEXT_WID: i32 = 100;

const BUTTON_WID: i32 = 100;
const BUTTON_HI: i32 = 40;

fn ask_pop(str: &str) -> (Window, Button, Button) {
    //  let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("!");
    //format!("Error happen: {}",
    let mut a = Frame::default()
        .with_size(TEXT_WID, 100)
        .with_label(&str.format_note(TEXT_WID, 4));
    a.set_pos((WIND_WID - TEXT_WID) / 2, 10);

    let mut yes = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("yes");
    yes.set_pos(0, WIND_HI - BUTTON_HI - 2);

    let mut no = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("no");
    no.set_pos((WIND_WID - BUTTON_WID) / 1, WIND_HI - BUTTON_HI - 2);
    //(WIND_WID - BUTTON_WID) /
    wind.end();
    //wind.show();

    // let vals = Rc::new(RefCell::new(5));

    //app.run().unwrap_e("Error making the ask window");
    (wind, yes, no)
}
