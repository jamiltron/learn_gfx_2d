extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;
extern crate image;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::Depth;

use gfx::{Device, texture};
use gfx::traits::FactoryExt;
use std::io::Cursor;

gfx_defines!{
    constant View {
        model: [[f32; 4]; 4] = "model",
        projection: [[f32; 4]; 4] = "projection",
    }

    vertex Vertex {
        position: [f32; 2] = "position",
        tex_coord: [f32; 2] = "texture_coord",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        view: gfx::ConstantBuffer<View> = "View",
        texture_sampler: gfx::TextureSampler<[f32; 4]> = "texture_sampler",
        out: gfx::BlendTarget<ColorFormat> = ("color", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

const CLEAR_COLOR: [f32; 4] = [0.59, 0.93, 0.59, 1.0];

// each texture quad's position and what point on the texture to tie it to
const TEX_QUAD: [Vertex; 4] = [Vertex {
                                   position: [0.5, 0.5],
                                   tex_coord: [1.0, 1.0],
                               },
                               Vertex {
                                   position: [0.5, -0.5],
                                   tex_coord: [1.0, 0.0],
                               },
                               Vertex {
                                   position: [-0.5, -0.5],
                                   tex_coord: [0.0, 0.0],
                               },
                               Vertex {
                                   position: [-0.5, 0.5],
                                   tex_coord: [0.0, 1.0],
                               }];

// the indices used by the element buffer
const TEX_INDICES: [u16; 6] = [0, 3, 1, 1, 3, 2];

const NEAR_PLANE: f32 = -1.0;
const FAR_PLANE: f32 = 10.0;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

const SPRITE_X: f32 = WINDOW_WIDTH as f32 / 2.0;
const SPRITE_Y: f32 = WINDOW_HEIGHT as f32 / 2.0;
const SPRITE_WIDTH: f32 = 266.0;
const SPRITE_HEIGHT: f32 = 266.0;

// stolen from: https://github.com/gfx-rs/gfx/tree/master/examples/blend
fn load_texture<R, F>(factory: &mut F,
                      data: &[u8])
                      -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = texture::Kind::D2(width as texture::Size,
                                 height as texture::Size,
                                 texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(kind, &[&img])
        .unwrap();
    Ok(view)
}

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Bouncing Kitten")
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync();

    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let pso = factory.create_pipeline_simple(include_bytes!("shaders/sprite_vert.glsl"),
                                include_bytes!("shaders/sprite_frag.glsl"),
                                pipe::new())
        .unwrap();

    let (vertex_buffer, slice) =
        factory.create_vertex_buffer_with_slice(&TEX_QUAD, &TEX_INDICES[..]);

    let kitty_texture = load_texture(&mut factory, &include_bytes!("../images/kitty.png")[..])
        .unwrap();
    let sampler = factory.create_sampler_linear();

    let projection = cgmath::ortho(0.0,
                                   WINDOW_WIDTH as f32,
                                   0.0,
                                   WINDOW_HEIGHT as f32,
                                   NEAR_PLANE,
                                   FAR_PLANE);

    let mut translation =
        cgmath::Matrix4::from_translation(cgmath::Vector3::new(SPRITE_X, SPRITE_Y, 0.0));
    let scale = cgmath::Matrix4::from_nonuniform_scale(SPRITE_WIDTH, SPRITE_HEIGHT, 1.0);

    let mut view = View {
        model: (translation * scale).into(),
        projection: projection.into(),
    };

    let data = pipe::Data {
        view: factory.create_constant_buffer(1),
        texture_sampler: (kitty_texture, sampler),
        out: main_color,
        vbuf: vertex_buffer,
    };

    'main: loop {
        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    break 'main
                }
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        let seconds = time::precise_time_s() as f32;
        let y_offset = seconds.sin().abs();
        translation = cgmath::Matrix4::from_translation(cgmath::Vector3::new(SPRITE_X,
                                                                             SPRITE_HEIGHT / 2.0 +
                                                                             (WINDOW_HEIGHT as f32 -
                                                                              SPRITE_HEIGHT) *
                                                                             y_offset,
                                                                             0.0));


        view.model = (translation * scale).into();

        // render everything
        encoder.update_constant_buffer(&data.view, &view);
        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
