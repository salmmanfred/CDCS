/*
this popup get the name for the province if in debug mode
and if it is not in debug mode it will say do you want this province for the mod
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
    enums::*,
    frame::Frame,
    input::Input,
    prelude::*,
    window::Window,
};

const WIND_WID: i32 = 200;
const WIND_HI: i32 = 160;

const TEXT_WID: i32 = 180;

const BUTTON_WID: i32 = 120;
const BUTTON_HI: i32 = 40;

pub fn prov_register(color: u32) -> Option<String> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("Note");

    let pos_x = (WIND_WID - TEXT_WID) / 2;
    let color_str = format!("{:#06x}", color);
    let mut a = Frame::default()
        .with_size(TEXT_WID, 50)
        .with_label(&color_str.format_note(TEXT_WID, 4));
    a.set_pos(pos_x, 10);
    let mut color_box = Frame::new(TEXT_WID / 2 + 50, 25, 20, 20, "");
    color_box.set_color(Color::from_u32(color));
    color_box.set_frame(FrameType::FlatBox);
    let mut name = Input::new(pos_x, 50, TEXT_WID, 40, "");

    let mut ok = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Ok");
    ok.set_pos((WIND_WID - BUTTON_WID) / 2, WIND_HI - BUTTON_HI - 2);

    wind.end();
    let (s, r) = app::channel::<bool>();
    ok.emit(s, true);

    // Ok on pressing enter
    name.set_trigger(CallbackTrigger::EnterKey);
    name.set_callback({
        move |_| {
            s.send(true);
        }
    });

    wind.show();
    while wind.shown() {
        if r.recv().is_some() {
            wind.hide();
            return Some(name.value());
        }
        app::wait();
    }
    return None;
}
