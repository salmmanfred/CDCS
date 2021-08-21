extern crate image;
use super::camera;
use crate::ui_ext;
use crate::{o, s};
use fltk::{enums::*, prelude::*, *};
use glium::index::PrimitiveType;
use glium::Surface;
use std::cell::RefCell;
use std::io::Cursor;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::mpsc;
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

pub struct MapContext {
    context: Rc<glium::backend::Context>,
    shader: Rc<glium::Program>,
    vertex_buffer: Rc<glium::VertexBuffer<Vertex>>,
    index_buffer: Rc<glium::IndexBuffer<u16>>,
    texture: Rc<glium::texture::SrgbTexture2d>,
    image: Rc<RefCell<image::RgbaImage>>,
}

pub struct Map {
    pub widget: window::GlutWindow,
    camera: Rc<RefCell<camera::CameraState>>,
    map_context: Option<Rc<RefCell<MapContext>>>,
    pub msg: mpsc::Receiver<(u8, u8, u8)>,
    sender: mpsc::Sender<(u8, u8, u8)>,
}

impl Map {
    pub fn new(size: (i32, i32)) -> Map {
        let mut wind = window::GlutWindow::new(0, 0, size.0, size.1, "");
        wind.end();
        wind.set_mode(enums::Mode::Opengl3);
        let (s, r) = mpsc::channel::<(u8, u8, u8)>();

        Map {
            widget: wind,
            map_context: None,
            camera: Rc::new(RefCell::new(camera::CameraState::new(size))),
            msg: r,
            sender: s,
        }
    }
    // Must be called after window.show()
    pub fn init_context(&mut self) {
        match self.map_context.as_ref() {
            None => (),
            _ => panic!("Map.init_context was called twice"),
        };

        // Magic code from https://github.com/fltk-rs/demos/blob/master/glium/src/main.rs
        // It just works
        let gl_window = Rc::new(RefCell::new(self.widget.clone()));
        struct Backend {
            gl_window: Rc<RefCell<window::GlutWindow>>,
        }
        unsafe impl glium::backend::Backend for Backend {
            fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
                Ok(self.gl_window.borrow_mut().swap_buffers())
            }

            unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
                self.gl_window.borrow().get_proc_address(symbol) as *const _
            }

            fn get_framebuffer_dimensions(&self) -> (u32, u32) {
                (
                    self.gl_window.borrow().width() as u32,
                    self.gl_window.borrow().height() as u32,
                )
            }

            fn is_current(&self) -> bool {
                unimplemented!()
            }

            unsafe fn make_current(&self) {
                self.gl_window.borrow_mut().make_current()
            }
        }
        let context = unsafe {
            let backend = Backend {
                gl_window: gl_window,
            };
            glium::backend::Context::new(backend, false, Default::default())
        }
        .unwrap();

        let image = image::load(
            Cursor::new(&include_bytes!("map_div.png")[..]),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();
        let image_dimensions = image.dimensions();
        let opengl_texture = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &image.clone().into_raw(),
            image_dimensions,
        );
        let opengl_texture = glium::texture::SrgbTexture2d::new(&context, opengl_texture).unwrap();
        let fs_code = s!(include_str!("map_shader.fs"));
        let vs_code = s!(include_str!("map_shader.vs"));
        let program = program!(&context,
            140 => {
                vertex: &vs_code,
                fragment: &fs_code,
            },
        )
        .unwrap();

        let vertex_buffer = {
            implement_vertex!(Vertex, position, tex_coord);
            glium::VertexBuffer::new(
                &context,
                &[
                    Vertex {
                        position: [0.0, 0.0],
                        tex_coord: [0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0],
                        tex_coord: [0.0, 1.0],
                    },
                    Vertex {
                        position: [2.0, 0.0],
                        tex_coord: [1.0, 0.0],
                    },
                    Vertex {
                        position: [2.0, 1.0],
                        tex_coord: [1.0, 1.0],
                    },
                ],
            )
            .unwrap()
        };
        let index_buffer =
            glium::IndexBuffer::new(&context, PrimitiveType::TriangleStrip, &[0u16, 1, 2, 3])
                .unwrap();

        let map_context = Rc::new(RefCell::new(MapContext {
            vertex_buffer: Rc::new(vertex_buffer),
            index_buffer: Rc::new(index_buffer),
            context: context,
            shader: Rc::new(program),
            texture: Rc::new(opengl_texture),
            image: Rc::new(RefCell::new(image)),
        }));
        self.map_context = Some(map_context.clone());

        self.widget.handle({
            let camera = self.camera.clone();
            let image = (*map_context).borrow_mut().image.clone();
            let sender = self.sender.clone();
            move |w, ev| match ev {
                Event::Released => {
                    if app::event_mouse_button() == app::MouseButton::Middle {
                        let mouse_pos = app::event_coords();
                        (*camera).borrow_mut().set_drag(mouse_pos, false);
                        w.redraw();
                        true
                    } else {
                        false
                    }
                }
                Event::Push => {
                    if app::event_is_click() {
                        match app::event_mouse_button() {
                            app::MouseButton::Middle => {
                                let mouse_pos = app::event_coords();
                                (*camera).borrow_mut().set_drag(mouse_pos, true);
                                w.redraw();
                                true
                            }
                            app::MouseButton::Left => {
                                let mouse_pos = app::event_coords();
                                let map_pos = (*camera).borrow_mut().get_map_pos(mouse_pos);
                                if map_pos.0 < 0.0
                                    || map_pos.0 > 2.0
                                    || map_pos.1 < 0.0
                                    || map_pos.1 > 1.0
                                {
                                   return false; 
                                }

                                let image_data = (*image).borrow_mut();
                                let image_size = image_data.dimensions();
                                let pixel_pos = (
                                    map_pos.0 / 2.0 * image_size.0 as f32,
                                    (1. - map_pos.1) * image_size.1 as f32,
                                );
                                println!("{:?}", map_pos);

                                let pixel =
                                    image_data.get_pixel(pixel_pos.0 as u32, pixel_pos.1 as u32);

                                sender.send((pixel[0], pixel[1], pixel[2])).unwrap();
                                false
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Event::Drag => {
                    let mouse_pos = app::event_coords();
                    (*camera).borrow_mut().drag(mouse_pos);
                    w.redraw();
                    true
                }
                Event::MouseWheel => match app::event_dy() {
                    app::MouseWheel::Up => {
                        (*camera).borrow_mut().scroll(1.1);
                        w.redraw();
                        true
                    }
                    app::MouseWheel::Down => {
                        (*camera).borrow_mut().scroll(0.9);
                        w.redraw();
                        true
                    }
                    _ => false,
                },

                _ => false,
            }
        });
        self.widget.draw({
			let context = map_context.clone();
            let camera = self.camera.clone();
			move |w| {
                let context = (*context).borrow_mut();
                let camera = (*camera).borrow_mut();
                // building the uniforms
                let uniforms = uniform! {
                    proj_matrix: camera.get_perspective(),
                    view_matrix: camera.get_view(),
                    tex: context.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                };

                let mut target = glium::Frame::new(
                    context.context.clone(),
                    context.context.get_framebuffer_dimensions(),
                );
                // drawing a frame
                target.clear_color(0.0, 0.0, 1.0, 1.0);
                target
                    .draw(
                        &*context.vertex_buffer,
                        &*context.index_buffer,
                        &*context.shader,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
            }
        });
    }
}
