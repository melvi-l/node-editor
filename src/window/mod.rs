use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

pub trait Controller {
    fn on_window_created(&mut self, window: &Window);
    fn on_window_event(&mut self, window: &Window, event: &WindowEvent);
    fn on_resize(&mut self, window: &Window, new_size: PhysicalSize<u32>);
    fn on_redraw(&mut self, window: &Window);
    fn on_dispose(&mut self, window: &Window);
}

#[derive(Default)]
pub struct App<C> {
    pub window: Option<Window>,
    pub ctrl: C,
}

impl<C: Controller> ApplicationHandler for App<C> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = event_loop
            .create_window(Window::default_attributes().with_title("winit exemple"))
            .expect("Unable to create window");
        self.ctrl.on_window_created(&window);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                println!("Stopping");
                self.ctrl.on_dispose(window);
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                println!("Resize {:?}", new_size);
                self.ctrl.on_resize(window, new_size);
            }
            _ => {
                self.ctrl.on_window_event(window, &event);
            }
        }
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                println!("Keyboard {:?}", event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                println!("Cursor moved {:?}", position)
            }
            WindowEvent::MouseInput { state, button, .. } => {
                println!("Mouse btn:{:?} state:{:?}", button, state);
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw");
            }
            _ => (),
        };
    }
}

pub fn run<C: Controller>(mut ctrl: C) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App { window: None, ctrl };
    event_loop.run_app(&mut app)?;
    Ok(())
}
