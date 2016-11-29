use app::App;
use gfx::{GfxContext, ProgramId, VertexDataTexture, TextureSampler, VertexTextureFormat};
use types::ColorF;

pub struct Test {
    program: ProgramId,
    instances: VertexDataTexture,
}

impl Test {
    pub fn new(gfx: &mut GfxContext) -> Test {
        Test {
            program: gfx.create_program("test.glsl"),
            instances: gfx.create_vertex_texture(TextureSampler::Float0),
        }
    }
}

impl App for Test {
    fn draw(&mut self,
            gfx: &mut GfxContext,
            _: u32,
            _: u32) {
        let mut instances: Vec<[f32; 4]> = vec![
            [100.0, 100.0, 100.0, 100.0],
            [100.0, 300.0, 200.0, 50.0],
        ];

        gfx.clear(ColorF::new(0.0, 0.0, 0.0, 1.0));
        gfx.bind_vertex_texture(&self.instances, &mut instances, VertexTextureFormat::F32);
        gfx.bind_program(self.program);
        gfx.draw_quads(instances.len());
    }
}