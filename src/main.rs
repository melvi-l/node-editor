mod core;
mod interactor;
mod render;
mod window;

use crate::{core::test_core, window::run};

struct State {
    graph: core::Graph,
    interactor: interactor::Interactor,
    ctx: Option<render::Ctx>,
}

impl State {
    pub fn new() -> Self {
        Self {
            graph: core::Graph::new(),
            interactor: interactor::Interactor::new(),
            ctx: None,
        }
    }
}

impl window::Controller for State {
    fn on_window_created(&mut self, window: &winit::window::Window) {
        self.ctx = Some(pollster::block_on(render::Ctx::new(window)));
        window.request_redraw();
    }

    fn on_window_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) {
        println!("todo");
    }

    fn on_resize(
        &mut self,
        window: &winit::window::Window,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        println!("todo");
    }

    fn on_redraw(&mut self, window: &winit::window::Window) {
        println!("todo");
        let Some(ctx) = self.ctx.as_mut() else {
            return;
        };

        ctx.draw();
    }

    fn on_dispose(&mut self, window: &winit::window::Window) {
        println!("todo");
    }
}

fn main() {
    let state = State::new();
    run(state); // does not open window on wayland because nothing is rendered
}
