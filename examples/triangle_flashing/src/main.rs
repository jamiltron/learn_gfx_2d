#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx::traits::FactoryExt;
use gfx::Device;
use std::f32;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "position",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        color: gfx::Global<[f32; 3]> = "color",
        out_color: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

const CLEAR_COLOR: [f32; 4] = [0.7, 0.85, 0.7, 1.0];

const TRIANGLE: [Vertex; 3] = [Vertex { position: [-0.5, -0.5] },
                               Vertex { position: [0.5, -0.5] },
                               Vertex { position: [0.0, 0.5] }];

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_title("Triangle Basic")
        .with_vsync();

    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(include_bytes!("shaders/vert.glsl"),
                                include_bytes!("shaders/frag.glsl"),
                                pipe::new())
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    // this time our pipe is mutable, because we will be changing the color value
    let mut data = pipe::Data {
        vertex_buffer: vertex_buffer,
        color: [1.0, 1.0, 1.0],
        out_color: main_color,
    };

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        // flash the triangle's color based on time
        let seconds = time::precise_time_s() as f32;
        let color_value = seconds.sin().abs();
        data.color = [color_value, color_value, color_value];

        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
