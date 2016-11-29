use app::{App, AppKind};
use gfx::{GfxContext, ProgramId, VertexDataTexture, TextureSampler, VertexTextureFormat};
use gleam::gl;
use types::ColorF;

pub struct QuadBench {
    program: ProgramId,
    instances: VertexDataTexture,
    clear_count: usize,
    quad_count: usize,
}

impl QuadBench {
    pub fn new(gfx: &mut GfxContext,
               clear_count: usize,
               quad_count: usize) -> QuadBench {
        if clear_count == 0 && quad_count == 0 {
            println!("{}", gl::get_string(gl::RENDERER));
            println!("{}", gl::get_string(gl::VERSION));
            println!("{}", gl::get_string(gl::VENDOR));
        }

        QuadBench {
            clear_count: clear_count,
            quad_count: quad_count,
            program: gfx.create_program("test.glsl"),
            instances: gfx.create_vertex_texture(TextureSampler::Float0),
        }
    }
}

impl App for QuadBench {
    fn kind(&self) -> AppKind {
        AppKind::Benchmark
    }

    fn draw(&mut self,
            gfx: &mut GfxContext,
            width: u32,
            height: u32) {
        for _ in 0..self.clear_count {
            gfx.clear(ColorF::new(0.0, 0.0, 0.0, 1.0));
        }

        if self.quad_count > 0 {
            let mut instances: Vec<[f32; 4]> = Vec::new();

            for _ in 0..self.quad_count {
                instances.push([0.0, 0.0, width as f32, height as f32]);
            }

            gfx.bind_vertex_texture(&self.instances, &mut instances, VertexTextureFormat::F32);
            gfx.bind_program(self.program);
            gfx.draw_quads(instances.len());
        }
    }
}