extern crate image;
use super::camera;
use fltk::{enums::*, prelude::*, *};
use glium::index::PrimitiveType;
use glium::Surface;
use std::cell::RefCell;
use std::io::Cursor;
use std::os::raw::c_void;
use std::rc::Rc;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

pub struct MapContext {
    context: std::rc::Rc<glium::backend::Context>,
    shader: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    texture: glium::texture::SrgbTexture2d,
}

pub struct Map {
    pub widget: window::GlutWindow,
    camera: camera::CameraState,
    map_context: Option<MapContext>,
}

impl Map {
    pub fn new(size: (i32, i32)) -> Map {
        let mut wind = window::GlutWindow::new(0, 0, size.0, size.1, "");
        wind.end();
        wind.set_mode(enums::Mode::Opengl3);

        Map {
            widget: wind,
            map_context: None,
            camera: camera::CameraState::new(size),
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
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = glium::texture::SrgbTexture2d::new(&context, image).unwrap();
        let fs_code = std::fs::read_to_string("src/graphics/map_shader.fs").unwrap();
        let vs_code = std::fs::read_to_string("src/graphics/map_shader.vs").unwrap();
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

        self.map_context = Some(MapContext {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            context: context,
            shader: program,
            texture: opengl_texture,
        });
    }
    pub fn update(&mut self) -> bool {
        // FLTK fails to capture some event types over the map,
        // so we have to use Event::NoEvent
        let mut changed = false;

        // Move map
        let mouse_pos = app::event_coords();
        if app::event_button() == app::MouseButton::Middle as i32 {
            changed |= match app::event() {
                Event::Released | Event::NoEvent => {
                    self.camera.update(mouse_pos, false);
                    true
                }
                Event::Drag => {
                    self.camera.update(mouse_pos, true);
                    true
                }
                _ => false,
            }
        }

        // Check mouse scroll
        if app::belowmouse::<window::GlutWindow>().is_some()
            && app::belowmouse::<window::GlutWindow>()
                .unwrap()
                .is_same(&self.widget)
            && app::event() == Event::NoEvent
        {
            changed |= match app::event_dy() {
                app::MouseWheel::Up => {
                    self.camera.scroll(1.1);
                    true
                }
                app::MouseWheel::Down => {
                    self.camera.scroll(0.9);
                    true
                }
                _ => false,
            }
        }
        return changed;
    }
    // Must be called after init_context()
    pub fn draw(&self) {
        match self.map_context {
            None => panic!("Map.draw() was called before Map.init_context()"),
            _ => (),
        };

        let map_context = self.map_context.as_ref().unwrap();
        // building the uniforms
        let uniforms = uniform! {
            proj_matrix: self.camera.get_perspective(),
            view_matrix: self.camera.get_view(),
            tex: map_context.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        let mut target = glium::Frame::new(
            map_context.context.clone(),
            map_context.context.get_framebuffer_dimensions(),
        );
        // drawing a frame
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &map_context.vertex_buffer,
                &map_context.index_buffer,
                &map_context.shader,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}