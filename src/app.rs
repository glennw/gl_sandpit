use gfx::GfxContext;

pub enum Event {
}

#[allow(dead_code)]
pub enum AppKind {
    Benchmark,      // Run for fixed # of frames, check timing.
    Test,           // Only call draw() after each UI event.
}

pub trait App {
    fn kind(&self) -> AppKind {
        AppKind::Test
    }

    fn on_event(&mut self, _: Event) {
    }

    fn draw(&mut self,
            gfx: &mut GfxContext,
            width: u32,
            height: u32);

    fn deinit(&mut self, _: &mut GfxContext) {
    }
}
