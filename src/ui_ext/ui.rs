use crate::common_traits::*;
use crate::ui_ext::popups::{ask, err, note};
use fltk::{
    app, app::wait_for, button::Button, enums::*, frame::Frame, group::Pack, input::Input,
    output::MultilineOutput, prelude::*, window::Window, *,
};

use crate::provinces;
use crate::s;
use crate::ui_ext::file;
use crate::ui_ext::popups::provr;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
enum Message {
    Build,
    File,
    Settings,
}
use crate::graphics::map::Map;

/*
honestly the Builder struct is un needed but it make it easier i suppose


*/
struct Builder {
    pub file: String,
    pub args: Vec<String>,
}
impl Builder {
    pub fn new() -> Builder {
        Builder {
            file: "".to_string(),
            args: Vec::new(),
        }
    }
}

pub fn run() {
    // allot of the ui is cluttery because of how fltk-rs works
    // this loads in the settings from a setting file
    // if no settings then it crates a new settings file
    let mut settings_head = match Settings::load() {
        Some(a) => a,
        None => Settings::new(),
    };
    // crates the app and starts the map
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let map_size = (700, 500);
    let mut wind = Window::default()
        .with_size(map_size.0 + 350, map_size.1)
        .with_label("CDCS - Country Detail Collection System @ 2.0.0");
    // makes the map
    let mut map = Map::new(map_size);

    // ! Standard input fields and stuff made here

    let mut pack = Pack::default().with_size(120, 140);
    // creates a pack that spaces out the following buttons
    pack.set_pos(map_size.0 + 2, 0);
    pack.set_spacing(10);
    let _ = Frame::default().with_size(0, 40).with_label("Nation name");
    let nation_name = Input::default().with_size(0, 40).with_label("-");
    let _ = Frame::default().with_size(0, 40).with_label("Culture name");
    let culture_name = Input::default().with_size(0, 40).with_label("-");
    let _ = Frame::default().with_size(0, 40).with_label("Religion");
    let religion = Input::default().with_size(0, 40).with_label("-");
    let _ = Frame::default()
        .with_size(0, 40)
        .with_label("Population size");
    let pop_size = Input::default().with_size(0, 40).with_label("-");
    let _ = Frame::default().with_size(0, 40).with_label("Build file");
    let build_file = Input::default().with_size(0, 40).with_label("-");
    pack.end();

    // ! menu buttons
    //pack2.set_spacing(10);
    // makes a button that has multiple buttons in it
    let mut frame2 = Button::default()
        .with_size(120, 40)
        .with_label("Select file");

    frame2.set_pos(map_size.0 + 150, 2);
    let mut menu = menu::MenuButton::default()
        .size_of(&frame2)
        .center_of(&frame2)
        .with_type(menu::MenuButtonType::Popup123);
    menu.set_color(Color::Green);
    menu.add_choice(&file::get_files());
    menu.set_callback(|m| {
        println!("{:?}", m.choice());
    });
    // function buttons
    let mut build = Button::default().with_size(120, 40).with_label("Build");
    build.set_pos(map_size.0 + 150, 52);

    let mut settings = Button::default().with_size(120, 40).with_label("Settings");
    settings.set_pos(map_size.0 + 230, 460);

    // Output field for error messages
    let mut error_disp = MultilineOutput::new(map_size.0 + 150, 100, 190, 150, "");
    error_disp.set_wrap(true);
    error_disp.set_color(Color::from_u32(0xc0c0c0));
    error_disp.set_frame(FrameType::FlatBox);
    error_disp.set_text_size(15);
    error_disp.set_text_color(Color::from_u32(0x8f0000));
    // starts the window
    wind.end();
    wind.show();
    map.init_context();
    // channels for communications between buttons
    let (s, r) = app::channel::<Message>();

    build.emit(s, Message::Build);
    menu.emit(s, Message::File);
    settings.emit(s.clone(), Message::Settings);

    let mut builder = Builder::new();

    let mut provinces = provinces::prov::new();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Build => {
                    /*
                    builds the builder and the arguments.

                    */

                    // here the builder struct is being used
                    // it pushes all the arguments into a single struct that will later be used
                    // by the main.rs script
                    builder.args.push(s!("value"));
                    builder.args.push(nation_name.value());
                    builder.args.push(culture_name.value());
                    builder.args.push(religion.value());
                    // if there is no build file name it generates one
                    if build_file.value() != "".to_string() {
                        builder.args.push(build_file.value());
                    } else {
                        // it uses the values of nation_name and so on
                        builder.args.push(format!(
                            "{}_{}_{}.lua",
                            nation_name.value(),
                            culture_name.value(),
                            religion.value()
                        ));
                    }

                    builder.args.push(pop_size.value());

                    // Build and print on errors
                    // here it runs the main.rs script and calculates the mod
                    // if there is an error it will set a text field to the error instead of crashing the
                    // program
                    match crate::run(builder.args.clone(), &builder.file, settings_head.clone()) {
                        Ok(_) => (),
                        Err(e) => error_disp.set_value(&e.to_string()),
                    }
                    builder.args = Vec::new();
                }
                Message::File => {
                    // gets the input file
                    // this file contains the provinces being chaned in the mod
                    // and pop size etc
                    //println!("{:#?}", );
                    if let Some(msg) = menu.choice() {
                        frame2.set_label(&msg);
                        builder.file = msg;
                    }
                }
                Message::Settings => {
                    // here it opens the settings window in popups/settings.rs
                    settings_head.change();
                }
            }
        } else {
            match map.msg.recv_timeout(Duration::from_nanos(1)) {
                // ERIK THE FUCK DID YOU DO?!
                Ok(pix) => {
                    let color = (pix.0 as u32) * 256 * 256 + (pix.1 as u32) * 256 + pix.2 as u32;
                    wind.deactivate();
                    map.widget.deactivate();
                    match settings_head.debug {
                        false => if ask::ask("Do you want to use it?") {},
                        true => match provr::prov_register(color) {
                            Some(name) => provinces.add(color, name),
                            None => (),
                        },
                    }
                    provinces.save();
                    wind.activate();
                    map.widget.activate();
                    println!("{:?}", provinces.provinces);
                }
                Err(_) => (),
            }
        }
    }
    // if for some reason it fails making the window the makes a window telling you that
    // a bit ironic
    app.run().unwrap_e("error making the main window");
}
