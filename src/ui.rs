use crate::run;
use orbtk::prelude::*;
static STACK_ID: &str = "STACK";
static ARGS: [&str; 4] = ["x", "lo", "SOE", "Tk"];
static FIL: &str = "file";
use std::fs;
#[derive(Copy, Clone)]
enum PopUpAction {
    Show,
    Hide,
}
#[derive(Copy, Clone)]

enum BuildAction {
    Build,
    Hide,
}
#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<PopUpAction>,
    action2: Option<BuildAction>,

    show_popup: bool,
    popup: Option<Entity>,
    file: String,
    ent_fil: usize,
    ent: Entity,
    num: [Entity; 4],
}
impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let l = ctx.entity_of_child(ARGS[0]).expect("err");
        let ll = ctx.entity_of_child(ARGS[1]).unwrap();
        let lll = ctx.entity_of_child(ARGS[2]).unwrap();
        let lv = ctx.entity_of_child(ARGS[3]).unwrap();

        self.num = [l, ll, lll, lv]
    }

    /*



    */
    /*
    Main loop
    if the action is pop up action it makes the pop up where you can select the correct file

    */
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                PopUpAction::Show => {
                    let stack = ctx.entity_of_child(STACK_ID).unwrap();
                    let current_entity = ctx.entity_of_parent().unwrap();
                    let build_context = &mut ctx.build_context();

                    let popup = create_popup(self.ent, current_entity, "Popup text", build_context);
                    build_context.append_child(stack, popup);
                    self.popup = Some(popup);
                    //println!("Popup created: {:?}", self.popup);
                }
                PopUpAction::Hide => {
                    if let Some(popup) = self.popup {
                        let lv = ctx.entity_of_child(ARGS[3]).unwrap();
                        //println!("{}",self.ent_fil);

                        let entit = ctx
                            .entity_of_child(self.ent_fil.to_string().as_str())
                            .unwrap();
                        let entfor = ctx.get_widget(entit).clone_or_default::<String16>("text");
                        let button = ctx.entity_of_child(FIL).unwrap();
                        ctx.get_widget(button).set("text", entfor);
                        ctx.remove_child(popup);

                        //println!("Popup removed !");
                    }
                }
            }
            self.action = None;
        }
        /*

        */
        if let Some(action) = self.action2 {
            match action {
                BuildAction::Build => {
                    //let x = ctx.get_widget(self.num[0]).has::<String>("input");
                    //println!("x{}",x);
                    let x = openfile::read_file(self.file.as_str());

                    let arg1 = ctx
                        .get_widget(self.num[0])
                        .clone_or_default::<String16>("text")
                        .as_string();
                    //println!("{}",self.file);

                    let arg2 = ctx
                        .get_widget(self.num[1])
                        .clone_or_default::<String16>("text")
                        .as_string();
                    let arg3 = ctx
                        .get_widget(self.num[2])
                        .clone_or_default::<String16>("text")
                        .as_string();
                    let arg4 = ctx
                        .get_widget(self.num[3])
                        .clone_or_default::<String16>("text")
                        .as_string();

                    run(vec!["ax".to_string(), arg1, arg2, arg3, arg4], x);

                    self.action2 = Some(BuildAction::Hide);
                }

                _ => {}
            }
        }
    }
}
impl MainViewState {
    /*
    shows and hides the pop up
    */
    fn show_popup(&mut self) {
        if self.show_popup {
            self.action = Some(PopUpAction::Hide);
        } else {
            self.action = Some(PopUpAction::Show);
        }
        self.show_popup = !self.show_popup;
    }
    fn file(&mut self, file: String, ent: usize) {
        self.file = file;
        self.ent_fil = ent;
    }
    fn build(&mut self) {
        self.action2 = Some(BuildAction::Build);
    }
}

widget!(MainView<MainViewState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        /*
        creates the main interface
        first it creates a gid and then puts it around the grid
        */
        self.name("MainView").margin(16.0).child(
            Grid::new()
                .id(STACK_ID)
                .columns(
                    Columns::create()
                        .columns(&["auto", "auto", "auto", "auto"])
                        .build(),
                )
                .background("#FFFFFF")
                .rows(
                    Rows::create()
                        .rows(&[
                            "auto", "auto", "auto", "auto", "auto", "auto", "auto", "auto", "auto",
                            "auto", "auto", "auto", "auto", "auto", "auto", "auto", "auto", "auto",
                            "auto", "auto", "auto", "auto", "auto", "auto", "auto",
                        ])
                        .build(),
                )
                /*
                these are the input fields
                */
                .place(
                    ctx,
                    TextBlock::new()
                        .text("Dont forget ./ on \n the save file name")
                        .font_size(10),
                    3,
                    1,
                )
                .place(ctx, TextBlock::new().text("Nation:").font_size(20), 1, 1)
                .place(
                    ctx,
                    TextBox::new()
                        .water_mark("Nation name")
                        .id(ARGS[0])
                        .water_mark("Username"),
                    1,
                    3,
                )
                .place(
                    ctx,
                    TextBlock::new().text("Population name:").font_size(20),
                    1,
                    5,
                )
                .place(ctx, TextBox::new().water_mark("pop name").id(ARGS[1]), 1, 7)
                .place(ctx, TextBlock::new().text("Religion:").font_size(20), 1, 8)
                .place(
                    ctx,
                    TextBox::new().water_mark("Religion name").id(ARGS[2]),
                    1,
                    10,
                )
                .place(
                    ctx,
                    Button::new()
                        .text("Select file")
                        .id(FIL)
                        .on_click(move |states, _| -> bool {
                            states.get_mut::<MainViewState>(id).show_popup();
                            states.get_mut::<MainViewState>(id).ent = id;

                            true
                        }),
                    1,
                    14,
                )
                .place(
                    ctx,
                    TextBox::new()
                        .text("./Build File")
                        .water_mark("Build file")
                        .id(ARGS[3]),
                    1,
                    18,
                )
                .place(
                    ctx,
                    Button::new()
                        .text("Build")
                        .on_click(move |states, _| -> bool {
                            states.get_mut::<MainViewState>(id).build();

                            true
                        }),
                    1,
                    24,
                )
                .build(ctx),
        )
    }
}

pub fn run_ui() {
    /*
    this is where the app is started
    */
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("CDCS - Country Detail Collection System")
                .background(Brush::SolidColor(Color::rgb(255, 255, 255)))
                .position((100.0, 100.0))
                .size(420.0, 420.0)
                .resizeable(false)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

fn get_files(target: Entity, _: Entity, ctx: &mut BuildContext) -> Entity {
    /*
    probably the coolest function

    it creates a stack and put some spacers on that

    then it looks into the current folder and gets each file name and after that
    */
    let mut st = Stack::new().spacing(0.1).child(
        TextBlock::new()
            .h_align("center")
            .v_align("top")
            .foreground("#000000")
            .text("Select file")
            .font_size(20)
            .build(ctx),
    );
    let mut list: Vec<String> = Vec::new();
    let mut list2: Vec<String> = Vec::new();

    for entry in fs::read_dir("./".to_string()).expect("Error reading folder") {
        // parses the folder into a folder struct
        /*
        gets the folder and its contents and then splits up it into <name of file> and <File extension>
        */
        let entry = entry
            .expect("error")
            .path()
            .into_os_string()
            .into_string()
            .expect("error");
        let entry_n = entry.split("/").collect::<Vec<&str>>(); //temp parse var
        let entry_n = entry_n[entry_n.len() - 1].split(".").collect::<Vec<&str>>(); // temp parse var

        /*
        here it puts it into 2 vectors that will keep control of the name and folder it also filters folders and files without extensions
        */
        //let entry_name = entry_n[0]; //final name
        let mut _entry_name2 = "str".to_string();
        if entry_n.len() >= 2 {
            _entry_name2 = vec![entry_n[0], entry_n[1]].join(".");
            let ent = _entry_name2.clone();
            list2.push(entry.clone());
            // con = con
            list.push(ent);
            // println!("x");
        }
        //println!("{:#?}, {}", entry_name2, entry_name);
    }

    for x in 0..list.len() {
        /*
        iterates through the loop and looks at each name and makes a button out of it
        */
        let pos = x + 1;
        let xx = list2[x].clone();
        let xxx = x;
        let x = list[x].clone();
        st = st.child(
            Button::new()
                .text(format!("{}", x))
                .id(xxx.to_string().as_str())
                .on_click(move |states, _| -> bool {
                    //  println!("xid {:#?}", target);
                    /*
                    if pressed it sends the mainviewstate the file that it wants to use and closes the popup
                    */
                    states
                        .get_mut::<MainViewState>(target)
                        .file(xx.clone(), xxx);
                    //println!("{}",xxx);

                    states.get_mut::<MainViewState>(target).show_popup();

                    true
                })
                .clip(false)
                .margin(10.0)
                .position(pos as i32)
                .build(ctx),
        );
    }
    /*

    it makes a scroll view and puts the stack into it
    */
    let con = ScrollViewer::new()
        .padding(8.0)
        .speed(2.0)
        .clip(false)
        .child(st.build(ctx));
    con.build(ctx)
    /*

    */
}

fn create_popup(target: Entity, hd: Entity, _: &str, ctx: &mut BuildContext) -> Entity {
    /*
    makes the popup
    */
    Popup::new()
        .target(target)
        .open(true)
        .width(300)
        .height(300.0)
        .child(get_files(target, hd, ctx))
        .build(ctx)
}
