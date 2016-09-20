#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

use gfx::traits::FactoryExt;
use gfx::Device;

// this is a macro that allows use to describe the forms of data we will use
gfx_defines! {
    // this defines the form of data we will be passing to our vertex shader
    vertex Vertex {
        position: [f32; 2] = "position",
        color: [f32; 3] = "color",
    }

    // this is the form of the pipe we will use to communicate to the gpu with
    pipeline pipe {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<ColorFormat> = "out_color",
    }
}

const ORANGE: [f32; 3] = [1.0, 0.5, 0.0];
const CLEAR_COLOR: [f32; 4] = [0.8, 1.0, 0.6, 1.0];

// here we define our triangle object in terms of points and colors
// note that the position is using 'normalized device coordinates',
// running from -1.0 to 1.0 in both the x and y axis
const TRIANGLE: [Vertex; 3] = [Vertex {
                                   position: [-0.5, -0.5],
                                   color: ORANGE,
                               },
                               Vertex {
                                   position: [0.5, -0.5],
                                   color: ORANGE,
                               },
                               Vertex {
                                   position: [0.0, 0.5],
                                   color: ORANGE,
                               }];

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub fn main() {
    // set our application to have the attributes we want, such as title and size
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_title("Triangle Basic")
        .with_vsync();

    // init everything provided by gfx_window_glutin
    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    // build an encoder from the factory provided by gfx_window_glutin,
    // this encoder 'encodes' the commands we will be passing to the gpu
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // create a pipeline simple object by providing it the source to our shaders
    let pso = factory.create_pipeline_simple(include_bytes!("shaders/vert.glsl"),
                                include_bytes!("shaders/frag.glsl"),
                                pipe::new())
        .unwrap();

    // generate the actual vertex buffer and a slice of data will use
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    // supply the pipeline with actual data, remember this is using the
    // definition we supplied in the gfx_defines! macro near the beginning
    let data = pipe::Data {
        vertex_buffer: vertex_buffer,
        out_color: main_color,
    };

    'main: loop {
        // quit our main loop if the window is closed or the escape key pressed
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        // clear the screen using our CLEAR_COLOR defined before main
        encoder.clear(&data.out_color, CLEAR_COLOR);

        // use the encoder to queue up a draw command with all of our data and the pipeline object
        encoder.draw(&slice, &pso, &data);

        // send the commands and clear the encoder
        encoder.flush(&mut device);

        // swap the new buffer onto our screen
        window.swap_buffers().unwrap();

        // provide any additional cleanup neccessary
        device.cleanup();
    }
}
