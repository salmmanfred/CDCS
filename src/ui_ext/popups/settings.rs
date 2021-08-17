#[allow(unused_imports)]
use crate::common_traits::*;
#[allow(unused_imports)]
use crate::ui_ext::popups::{ask, err, note};
use crate::ui_ext::ui;
#[allow(unused_imports)]
use crate::ui_ext::FormatNote;
#[allow(unused_imports)]
use crate::{o, s};
#[allow(unused_imports)]
use fltk::{
    app::{self, Receiver, Sender},
    button::Button,
    enums::*,
    frame::Frame,
    prelude::*,
    window::Window,
};

#[derive(Debug, Clone)]
enum Message {
    Save,
    ChangeWarn,
    Reload,
}

pub fn settings(st: &mut Settings) {
    settings_pop(st);

    //false
    //rt();
}

const WIND_WID: i32 = 400;
const WIND_HI: i32 = 200;

const TEXT_WID: i32 = 100;

const BUTTON_WID: i32 = 100;
const BUTTON_HI: i32 = 40;

fn settings_pop(st: &mut Settings) {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(WIND_WID, WIND_HI)
        .with_label("Settings");
    //format!("Error happen: {}",

    let mut save = Button::default()
        .with_size(WIND_WID, BUTTON_HI)
        .with_label("Save");
    save.set_pos(0, WIND_HI - BUTTON_HI - 2);

    let mut reload = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Reload");
    reload.set_pos(
        (WIND_WID - BUTTON_WID) / 1,
        (WIND_HI - BUTTON_HI - 2) - BUTTON_HI,
    );
    reload.set_color(Color::Red);

    let mut op1 = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Warn: True");
    op1.set_pos(0, (WIND_HI - BUTTON_HI - 2) - BUTTON_HI);
    //(WIND_WID - BUTTON_WID) /
    wind.end();
    //wind.show();

    // let vals = Rc::new(RefCell::new(5));

    //app.run().unwrap_e("Error making the ask window");
    let (s, r) = app::channel::<Message>();
    save.emit(s.clone(), Message::Save);
    op1.emit(s.clone(), Message::ChangeWarn);
    reload.emit(s.clone(), Message::Reload);

    &op1.set_label(&format!("Warn: {}", st.warn));

    wind.show();

    while wind.shown() {
        //println!("shit {:#?}",r.recv());
        app::wait();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Save => {
                    st.save();
                    wind.hide();
                }
                Message::ChangeWarn => {
                    // wind.hide();
                    if st.warn {
                        &op1.set_label("Warn: False");
                        st.warn = false;
                    } else {
                        &op1.set_label("Warn: True");
                        st.warn = true;
                    }
                }
                Message::Reload => {
                    if !st.warn || ask::ask("Are you sure?") {
                        app.quit();
                        let _ = Box::new(ui::run());

                        panic!("shut down");
                    }
                }
            }
        }
    }
}
