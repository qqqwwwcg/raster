use fltk::{app, enums::Mode, prelude::*, window::Window};

use raster::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 800;

fn main() {
    let app = fltk::app::App::default();
    let mut window = Window::new(100, 100, WIDTH, HEIGHT, "raster");

    let mut render = Render::new(WIDTH as u32, HEIGHT as u32);
    let mesh = Mesh::load("assets/Red.obj");
    mesh.into_iter().for_each(|triangle| {
        render.draw_triangle(
            50.0 * triangle[0].position.downgrade() + Vec2::new(500.0, 500.0),
            50.0 * triangle[1].position.downgrade() + Vec2::new(500.0, 500.0),
            50.0 * triangle[2].position.downgrade() + Vec2::new(500.0, 500.0),
            Vec3::new(255.0, 255.0, 255.0),
        )
    });
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
