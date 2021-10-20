/*
shows a simple text note
*/
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
};

pub fn note(str: &str) {
    let mut wind = note_pop(str);

    wind.show();

    while wind.shown() {
        //println!("shit {:#?}",r.recv());
        app::wait();
    }

    //rt();
}

const WIND_WID: i32 = 200;
const WIND_HI: i32 = 160;

const TEXT_WID: i32 = 100;

const BUTTON_WID: i32 = 120;
const BUTTON_HI: i32 = 40;

fn note_pop(str: &str) -> Window {
    //  let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("Note");
    //format!("Error happen: {}",
    let mut a = Frame::default()
        .with_size(TEXT_WID, 100)
        .with_label(&str.format_note(TEXT_WID, 4));
    a.set_pos((WIND_WID - TEXT_WID) / 2, 10);
    let mut ok = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Ok");
    ok.set_pos((WIND_WID - BUTTON_WID) / 2, WIND_HI - BUTTON_HI - 2);

    wind.end();
    //wind.show();
    let mut wind2 = wind.clone();

    ok.set_callback(move |_| {
        wind2.hide();
    });

    //app.run().unwrap_e("Error making the note window");
    wind
}
