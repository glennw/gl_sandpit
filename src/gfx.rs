use gleam::gl;
use glutin::WindowProxy;
use inotify::INotify;
use inotify::ffi::*;
use std::collections::HashMap;
use std::mem;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use types::ColorF;
use util;

const MAX_VERTEX_TEXTURE_WIDTH: usize = 1024;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureSampler {
    Float0,
    Float1,
}

struct QuadVertex {
    _pos: [f32; 2],
}

enum VertexAttribute {
    Position = 0,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct ProgramId(usize);

struct Program {
    path: PathBuf,
    program: gl::GLuint,
    u_transform: gl::GLint,
}

#[allow(dead_code)]
pub enum VertexTextureFormat {
    I32,
    F32,
}

pub struct VertexDataTexture {
    id: gl::GLuint,
    sampler: TextureSampler,
}

impl VertexDataTexture {
    fn new(sampler: TextureSampler) -> VertexDataTexture {
        let id = gl::gen_textures(1)[0];

        gl::active_texture(gl::TEXTURE0 + sampler as gl::GLuint);
        gl::bind_texture(gl::TEXTURE_2D, id);

        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::GLint);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::GLint);

        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::GLint);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::GLint);

        gl::bind_texture(gl::TEXTURE_2D, 0);

        VertexDataTexture {
            id: id,
            sampler: sampler,
        }
    }

    fn update_and_bind<T>(&self, data: &mut Vec<T>, format: VertexTextureFormat) {
        if data.is_empty() {
            return;
        }

        let item_size = mem::size_of::<T>();
        debug_assert!(item_size % 16 == 0);
        let vecs_per_item = item_size / 16;

        let items_per_row = MAX_VERTEX_TEXTURE_WIDTH / vecs_per_item;

        // Extend the data array to be a multiple of the row size.
        // This ensures memory safety when the array is passed to
        // OpenGL to upload to the GPU.
        let mut dummy_items = 0;
        while data.len() % items_per_row != 0 {
            data.push(unsafe { mem::uninitialized() });
            dummy_items += 1;
        }

        let width = items_per_row * vecs_per_item;
        let height = data.len() / items_per_row;

        gl::active_texture(gl::TEXTURE0 + self.sampler as gl::GLuint);
        gl::bind_texture(gl::TEXTURE_2D, self.id);

        match format {
            VertexTextureFormat::I32 => {
                gl::tex_image_2d(gl::TEXTURE_2D,
                                 0,
                                 gl::RGBA32I as gl::GLint,
                                 width as gl::GLint,
                                 height as gl::GLint,
                                 0,
                                 gl::RGBA_INTEGER,
                                 gl::INT,
                                 Some(unsafe { mem::transmute(data.as_slice()) } ));
            }
            VertexTextureFormat::F32 => {
                gl::tex_image_2d(gl::TEXTURE_2D,
                                 0,
                                 gl::RGBA32F as gl::GLint,
                                 width as gl::GLint,
                                 height as gl::GLint,
                                 0,
                                 gl::RGBA,
                                 gl::FLOAT,
                                 Some(unsafe { mem::transmute(data.as_slice()) } ));
            }
        }

        // Remove dummy items
        for _ in 0..dummy_items {
            data.pop();
        }
    }
}

pub struct GfxContext {
    resource_path: PathBuf,
    shared_path: PathBuf,
    quad_vao_id: gl::GLuint,
    quad_ibo: gl::GLuint,
    offset_x: f32,
    offset_y: f32,
    scale_x: f32,
    scale_y: f32,
    next_id: usize,
    programs: HashMap<ProgramId, Program>,
    watch_rx: Receiver<String>,
}

impl GfxContext {
    pub fn new(window_proxy: WindowProxy) -> GfxContext {
        let res_path = PathBuf::from("res/");

        let mut shared_path = res_path.clone();
        shared_path.push("shared.glsl");

        let (watch_tx, watch_rx) = channel();
        let watch_path = res_path.clone();
        thread::spawn(move || {
            let mut ino = INotify::init().unwrap();
            ino.add_watch(&watch_path, IN_CLOSE_WRITE).unwrap();
            loop {
                let events = ino.wait_for_events().unwrap();
                for event in events.iter() {
                    watch_tx.send(event.name.display().to_string()).unwrap();
                }
                window_proxy.wakeup_event_loop();
            }
        });

        let x0 = 0.0;
        let y0 = 0.0;
        let x1 = 1.0;
        let y1 = 1.0;

        let quad_indices: [u16; 6] = [ 0, 1, 2, 2, 1, 3 ];
        let quad_vertices = [
            QuadVertex {
                _pos: [x0, y0],
            },
            QuadVertex {
                _pos: [x1, y0],
            },
            QuadVertex {
                _pos: [x0, y1],
            },
            QuadVertex {
                _pos: [x1, y1],
            },
        ];

        let quad_buffer_ids = gl::gen_buffers(2);
        let quad_ibo = quad_buffer_ids[0];
        let quad_vbo = quad_buffer_ids[1];

        let vao_ids = gl::gen_vertex_arrays(1);
        let vao_id = vao_ids[0];

        gl::pixel_store_i(gl::UNPACK_ALIGNMENT, 1);

        gl::bind_vertex_array(vao_id);

        gl::bind_buffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::buffer_data(gl::ARRAY_BUFFER, &quad_vertices, gl::STATIC_DRAW);

        gl::enable_vertex_attrib_array(VertexAttribute::Position as gl::GLuint);
        let vertex_stride = mem::size_of::<QuadVertex>() as gl::GLuint;
        gl::vertex_attrib_pointer(VertexAttribute::Position as gl::GLuint,
                                2,
                                gl::FLOAT,
                                false,
                                vertex_stride as gl::GLint,
                                0);

        gl::bind_vertex_array(0);

        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, quad_ibo);
        gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, &quad_indices, gl::STATIC_DRAW);

        GfxContext {
            resource_path: res_path,
            shared_path: shared_path,
            quad_vao_id: vao_id,
            quad_ibo: quad_ibo,
            offset_x: 0.0,
            offset_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            next_id: 0,
            programs: HashMap::new(),
            watch_rx: watch_rx,
        }
    }

    pub fn begin_frame(&mut self, width: u32, height: u32) {
        let mut files_modified = false;
        while let Ok(_changed_file) = self.watch_rx.try_recv() {
            files_modified = true;
            break;
        }
        if files_modified {
            self.refresh_shaders();
        }

        gl::depth_mask(false);
        gl::disable(gl::STENCIL_TEST);
        gl::disable(gl::BLEND);
        gl::bind_vertex_array(self.quad_vao_id);
        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.quad_ibo);

        self.offset_x = -1.0;
        self.offset_y = 1.0;
        self.scale_x = 2.0 / width as f32;
        self.scale_y = -2.0 / height as f32;
    }

    pub fn bind_vertex_texture<T>(&mut self,
                                  texture: &VertexDataTexture,
                                  data: &mut Vec<T>,
                                  format: VertexTextureFormat) {
        texture.update_and_bind(data, format);
    }

    pub fn create_vertex_texture(&mut self, sampler: TextureSampler) -> VertexDataTexture {
        VertexDataTexture::new(sampler)
    }

    pub fn create_program(&mut self, name: &str) -> ProgramId {
        let mut shader_path = self.resource_path.clone();
        shader_path.push(name);
        let program = new_program(shader_path, &self.shared_path).expect("Failed to compile!");
        let id = ProgramId(self.next_id);
        self.next_id += 1;
        self.programs.insert(id, program);
        id
    }

    pub fn bind_program(&mut self, id: ProgramId) {
        let program = &self.programs[&id];
        gl::use_program(program.program);
        gl::uniform_4f(program.u_transform, self.offset_x, self.offset_y, self.scale_x, self.scale_y);
    }

    pub fn clear(&self, color: ColorF) {
        gl::clear_color(color.r, color.g, color.b, color.a);
        gl::clear(gl::COLOR_BUFFER_BIT);

    }

    pub fn draw_quads(&mut self, count: usize) {
        gl::draw_elements_instanced(gl::TRIANGLES,
                                    6,
                                    gl::UNSIGNED_SHORT,
                                    0,
                                    count as gl::GLint);
    }

    pub fn refresh_shaders(&mut self) {
        for (_, program) in &mut self.programs {
            let new_program = new_program(program.path.clone(), &self.shared_path);
            if let Some(new_program) = new_program {
                gl::delete_program(program.program);
                *program = new_program;
            } else {
                println!("Failed to compile {:?}, using old shader!", program.path);
            }
        }
    }

    pub fn end_frame(&mut self) {
    }
}

fn compile_shader(source: &str,
                  defines: &str,
                  shared: &str,
                  shader_type: gl::GLenum) -> Option<gl::GLuint> {
    let version = "#version 150\n";
    let id = gl::create_shader(shader_type);

    let mut src = Vec::new();
    src.extend_from_slice(version.as_bytes());
    src.extend_from_slice(defines.as_bytes());
    src.extend_from_slice(shared.as_bytes());
    src.extend_from_slice(source.as_bytes());
    gl::shader_source(id, &[&src[..]]);

    gl::compile_shader(id);
    if gl::get_shader_iv(id, gl::COMPILE_STATUS) == (0 as gl::GLint) {
        println!("Failed to compile shader: {}", gl::get_shader_info_log(id));
        None
    } else {
        Some(id)
    }
}

fn create_program(source: &str,
                  shared: &str) -> Option<gl::GLuint> {
    let vs_defines = "#define VERTEX_SHADER\n";
    let fs_defines = "#define FRAGMENT_SHADER\n";
    let mut program = None;

    let vs_id = compile_shader(source,
                               vs_defines,
                               shared,
                               gl::VERTEX_SHADER);

    if let Some(vs_id) = vs_id {
        let fs_id = compile_shader(source,
                                   fs_defines,
                                   shared,
                                   gl::FRAGMENT_SHADER);

        if let Some(fs_id) = fs_id {
            let pid = gl::create_program();

            gl::attach_shader(pid, vs_id);
            gl::attach_shader(pid, fs_id);

            gl::bind_attrib_location(pid, VertexAttribute::Position as gl::GLuint, "aPosition");

            gl::link_program(pid);

            if gl::get_program_iv(pid, gl::LINK_STATUS) == (0 as gl::GLint) {
                println!("Failed to link shader program: {}", gl::get_program_info_log(pid));
                gl::detach_shader(pid, vs_id);
                gl::detach_shader(pid, fs_id);
            } else {
                program = Some(pid);
            }
        }
    }

    program
}

fn new_program(path: PathBuf, shared_path: &PathBuf) -> Option<Program> {
    let shared = util::load_text_file(shared_path);
    let shader_source = util::load_text_file(&path);
    let program = create_program(&shader_source, &shared);
    program.map(|program| {
        gl::use_program(program);
        let u_transform = gl::get_uniform_location(program, "uTransform");
        let u_float0 = gl::get_uniform_location(program, "sFloat0");
        if u_float0 != -1 {
            gl::uniform_1i(u_float0, TextureSampler::Float0 as gl::GLint);
        }
        let u_float1 = gl::get_uniform_location(program, "sFloat1");
        if u_float1 != -1 {
            gl::uniform_1i(u_float1, TextureSampler::Float1 as gl::GLint);
        }
        gl::use_program(0);
        Program {
            path: path,
            program: program,
            u_transform: u_transform,
        }
    })
}
