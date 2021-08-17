use crate::common_traits::*;
use fltk::{
    app, app::wait_for, button::Button, enums::*, frame::Frame, group::Pack, input::Input,
    output::MultilineOutput, prelude::*, window::Window, *,
};

use crate::s;
use crate::ui_ext::file;

use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Message {
    Build,
    File,
    Settings,
}
use crate::graphics::map::Map;

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
    let mut settings_head = match Settings::load() {
        Some(a) => a,
        None => Settings::new(),
    };
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let map_size = (700, 500);
    let mut wind = Window::default()
        .with_size(map_size.0 + 350, map_size.1)
        .with_label("CDCS - Country Detail Collection System @ 2.0.0");

    let mut map = Map::new(map_size);

    // ! Standard input fields and stuff made here
    let mut pack = Pack::default().with_size(120, 140);
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

    wind.end();
    wind.show();
    map.init_context();
    map.draw();

    let (s, r) = app::channel::<Message>();

    build.emit(s, Message::Build);
    menu.emit(s, Message::File);
    settings.emit(s, Message::Settings);

    let mut builder = Builder::new();

    let secs_between_frames = 1. / 60.;
    let mut last_time = Instant::now();

    let mut scroll = 0;
    loop {
        let secs_elapsed = last_time.elapsed().as_secs_f64();
        let secs_remaining = secs_between_frames - secs_elapsed;
        match wait_for(f64::max(secs_remaining, 0.0)) {
            Err(_) => app.run().unwrap_e("error making the main window"),
            Ok(x) => match x {
                false => {
                    // There is no event, so just update and draw the map
                    map.update(scroll);
                    scroll = 0;
                    map.draw();
                    last_time = Instant::now();
                    continue;
                }
                true => (),
            },
        }

        if let Some(msg) = r.recv() {
            match msg {
                Message::Build => {
                    /*
                    builds the builder and the arguments.

                    */
                    builder.args.push(s!("value"));
                    builder.args.push(nation_name.value());
                    builder.args.push(culture_name.value());
                    builder.args.push(religion.value());
                    if build_file.value() != "".to_string() {
                        builder.args.push(build_file.value());
                    } else {
                        builder.args.push(format!(
                            "{}_{}_{}.lua",
                            nation_name.value(),
                            culture_name.value(),
                            religion.value()
                        ));
                    }
                    builder.args.push(pop_size.value());

                    // Build and print on errors
                    match crate::run(builder.args.clone(), &builder.file, settings_head.clone()) {
                        Ok(_) => (),
                        Err(e) => error_disp.set_value(&e.to_string()),
                    }
                    builder.args = Vec::new();
                }
                Message::File => {
                    //println!("{:#?}", );
                    if let Some(msg) = menu.choice() {
                        frame2.set_label(&msg);
                        builder.file = msg;
                    }
                }
                Message::Settings => {
                    settings_head.change();
                }
            }
        } else {
            if app::event() == Event::NoEvent || app::event() == Event::MouseWheel {
                match app::event_dy() {
                    app::MouseWheel::Up => scroll += 1,
                    app::MouseWheel::Down => scroll -= 1,
                    _ => (),
                }
                println!("TJA");
            }
            if app::event() == Event::Close {
                return;
            }
        }
    }
    app.run().unwrap_e("error making the main window");
}
