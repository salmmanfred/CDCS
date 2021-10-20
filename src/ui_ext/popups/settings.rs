/*
settings popup where you can change the settings
*/

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
    ChangeDebug,
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
    // creates the window after getting the settings &mut 
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
    let mut op2 = Button::default()
        .with_size(BUTTON_WID, BUTTON_HI)
        .with_label("Debug: False");
    op2.set_pos(BUTTON_WID, (WIND_HI - BUTTON_HI - 2) - BUTTON_HI);
    //(WIND_WID - BUTTON_WID) /
    wind.end();
    //wind.show();

    // let vals = Rc::new(RefCell::new(5));

    //app.run().unwrap_e("Error making the ask window");
    let (s, r) = app::channel::<Message>();
    save.emit(s.clone(), Message::Save);
    op1.emit(s.clone(), Message::ChangeWarn);
    op2.emit(s.clone(), Message::ChangeDebug);

    reload.emit(s.clone(), Message::Reload);

    &op1.set_label(&format!("Warn: {}", st.warn));
    &op2.set_label(&format!("Debug: {}", st.debug));

    wind.show();

    while wind.shown() {
        //println!("shit {:#?}",r.recv());
        app::wait();
        if let Some(msg) = r.recv() {
            match msg {
                // when pressing save the settings file will write itself to a file
                Message::Save => {
                    st.save();
                    wind.hide();
                }
                // this will change the warn prefrence for the program if its turned off it will simply
                // never use the ask.rs(ui_ext/popups/ask.rs) window
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
                // change if the program should be in debug or not
                Message::ChangeDebug => {
                    // wind.hide();
                    if st.debug {
                        &op2.set_label("Debug: False");
                        st.debug = false;
                    } else {
                        &op2.set_label("Debug: True");
                        st.debug = true;
                    }
                }
                // forces a reload of the program ui
                Message::Reload => {
                    // if the user wants a warn it will ask otherwise it will not
                    if !st.warn || ask::ask("Are you sure?") {
                        app.quit();
                        // it will start the new app in a box 
                        // so you can start more without stack issues
                        Box::new(ui::run());
                    }
                }
            }
        }
    }
}
