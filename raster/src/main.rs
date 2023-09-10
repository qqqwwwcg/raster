use fltk::{prelude::*, window::Window};

const WIDTH:i32 = 1200;
const HEIGHT:i32 = 800;

//app: application for run event loop
//window: surface and persation image

fn main() {
let app = fltk::app::App::default();
let mut window = Window::new(100,100,WIDTH,HEIGHT,"raster");

window.end();
window.show();

//run event loop
app.run().unwrap();
}
