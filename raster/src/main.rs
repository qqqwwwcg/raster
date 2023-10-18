use fltk::{app, enums::Mode, prelude::*, window::Window};

mod mesh;
use mesh::*;
mod math;
use math::*;
mod buffer;
use buffer::*;
mod render;
use render::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 800;

//app: application for run event loop
//window: surface and persation image

fn main() {
    let app = fltk::app::App::default();
    let mut window = Window::new(100, 100, WIDTH, HEIGHT, "raster");

    //let mut framer_buffer = FrameBuffer::new(WIDTH as u32, HEIGHT as u32);
    // framer_buffer.draw_pixel(
    //     (WIDTH as u32 / 2, HEIGHT as u32 / 2),
    //     Vec3::new(255.0, 0.0, 0.0),
    // );
    //let data = framer_buffer.flatten();

    let mut render = Render::new(WIDTH as u32, HEIGHT as u32);
    //draw X
    render.draw_line(
        Vec2::new(10.0, 10.0),
        Vec2::new(400.0, 400.0),
        Vec3::new(255.0, 0.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(400.0, 10.0),
        Vec2::new(10.0, 400.0),
        Vec3::new(255.0, 0.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );

    //draw Orient::CW rect
    render.draw_line(
        Vec2::new(10.0, 10.0),
        Vec2::new(400.0, 10.0),
        Vec3::new(0.0, 255.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(400.0, 10.0),
        Vec2::new(400.0, 400.0),
        Vec3::new(0.0, 255.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(400.0, 400.0),
        Vec2::new(10.0, 400.0),
        Vec3::new(0.0, 255.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(10.0, 400.0),
        Vec2::new(10.0, 10.0),
        Vec3::new(0.0, 255.0, 0.0),
        render::DrawLineAlgorithm::DDA,
    );

    //draw Orient::CCW rect
    render.draw_line(
        Vec2::new(100.0, 100.0),
        Vec2::new(100.0, 300.0),
        Vec3::new(0.0, 0.0, 255.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(100.0, 300.0),
        Vec2::new(300.0, 300.0),
        Vec3::new(0.0, 0.0, 255.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(300.0, 300.0),
        Vec2::new(300.0, 100.0),
        Vec3::new(0.0, 0.0, 255.0),
        render::DrawLineAlgorithm::DDA,
    );
    render.draw_line(
        Vec2::new(300.0, 100.0),
        Vec2::new(100.0, 100.0),
        Vec3::new(0.0, 0.0, 255.0),
        render::DrawLineAlgorithm::DDA,
    );
    let data = render.get_frame();

    window.draw(move |_| {
        fltk::draw::draw_image(&data, 0, 0, WIDTH, HEIGHT, fltk::enums::ColorDepth::Rgb8).unwrap();
    });

    window.end();
    app::set_visual(Mode::Rgb8).unwrap();
    window.show();

    //run event loop
    app.run().unwrap();
}
