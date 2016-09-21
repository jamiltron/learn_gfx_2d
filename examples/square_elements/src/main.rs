#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx::traits::FactoryExt;
use gfx::Device;

gfx_defines! {
    vertex Vertex {
        position: [f32; 2] = "position",
        color: [f32; 3] = "color",
    }

    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

const CLEAR_COLOR: [f32; 4] = [0.15, 0.15, 0.15, 1.0];

// because we will specify the indices, we only need to define four vertices here
const SQUARE_VERTICES: [Vertex; 4] = [Vertex {
                                          position: [0.5, 0.5],
                                          color: [1.0, 1.0, 0.0],
                                      },
                                      Vertex {
                                          position: [0.5, -0.5],
                                          color: [0.0, 1.0, 0.0],
                                      },
                                      Vertex {
                                          position: [-0.5, -0.5],
                                          color: [1.0, 0.0, 0.0],
                                      },
                                      Vertex {
                                          position: [-0.5, 0.5],
                                          color: [0.0, 0.0, 1.0],
                                      }];

// here we define the two triangles by saying which indices to use in what order
const SQUARE_INDICES: [u16; 6] = [0, 3, 1, 1, 3, 2];

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_title("Square Elements")
        .with_vsync();

    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(include_bytes!("shaders/vert.glsl"),
                                include_bytes!("shaders/frag.glsl"),
                                pipe::new())
        .unwrap();

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&SQUARE_VERTICES, &SQUARE_INDICES[..]);

    let data = pipe::Data {
        vertex_buffer: vertex_buffer,
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

        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
