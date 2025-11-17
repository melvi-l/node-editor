use wgpu::SurfaceTargetUnsafe;

struct RenderSize {
    width: u32,
    height: u32,
}

impl From<winit::dpi::PhysicalSize<u32>> for RenderSize {
    fn from(ps: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: ps.width,
            height: ps.height,
        }
    }
}

pub struct Ctx {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: RenderSize,
    // msaa_sample: u32,
    // msaa_color: Option<wgpu::Texture>,
}

pub struct FrameCtx {
    pub encoder: wgpu::CommandEncoder,
    pub swap_view: wgpu::TextureView,
}

impl Ctx {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size: RenderSize = window.inner_size().into();

        let descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };

        let instance = wgpu::Instance::new(&descriptor);

        // J'adore le danger (skill issue)
        let target = unsafe {
            SurfaceTargetUnsafe::from_window(window).expect("Unable to create unsafe target")
        };
        let surface: wgpu::Surface<'static> = unsafe {
            instance
                .create_surface_unsafe(target)
                .expect("Unable to create unsafe surface")
        };

        let adapter_options = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance.request_adapter(&adapter_options).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface_capatibilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capatibilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| surface_capatibilities.formats.first().copied())
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capatibilities
                .present_modes
                .iter()
                .copied()
                .find(|m| matches!(m, wgpu::PresentMode::AutoVsync | wgpu::PresentMode::Fifo))
                .unwrap_or(wgpu::PresentMode::Fifo),
            alpha_mode: surface_capatibilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Ctx {
            instance,
            surface,
            config,
            device,
            queue,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size.into();
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        // msaa
    }

    pub fn begin_frame(&mut self) -> Result<(wgpu::SurfaceTexture, FrameCtx), wgpu::SurfaceError> {
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(e) => match e {
                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                    self.surface.configure(&self.device, &self.config);
                    self.surface.get_current_texture()?
                }
                _ => return Err(e.into()),
            },
        };

        let swap_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("main-encoder"),
            });

        Ok((frame, FrameCtx { encoder, swap_view }))
    }

    pub fn submit(&mut self, frame: wgpu::SurfaceTexture, encoder: wgpu::CommandEncoder) -> () {
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present()
    }

    pub fn begin_render_pass<'a>(
        &'a self,
        fctx: &'a mut FrameCtx,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &fctx.swap_view,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear),
                store: wgpu::StoreOp::Store,
            },
        };
        fctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("main render pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        })
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let (frame, mut fctx) = self.begin_frame()?;

        let _ = self.begin_render_pass(
            &mut fctx,
            wgpu::Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        );

        self.submit(frame, fctx.encoder);

        Ok(())
    }
}
