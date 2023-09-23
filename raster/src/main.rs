use fltk::{prelude::*, window::Window};

const WIDTH:i32 = 1200;
const HEIGHT:i32 = 800;

//app: application for run event loop
//window: surface and persation image

fn main() {
let app = fltk::app::App::default();
let mut window = Window::new(100,100,WIDTH,HEIGHT,"raster");

let data:Vec<u8> = (0..WIDTH*HEIGHT*3).into_iter().map(|channle|channle as u8).collect();
window.draw(move |_| {
    fltk::draw::draw_image(
        &data,
        0,
        0,
        WIDTH ,
        HEIGHT ,
        fltk::enums::ColorDepth::Rgb8,
    )
    .unwrap();
});

window.end();
window.show();

//run event loop
app.run().unwrap();
}
