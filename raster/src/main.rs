use fltk::{prelude::*, window::Window};

const WIDTH:i32 = 1200;
const HEIGHT:i32 = 800;

fn main() {
let app = fltk::app::App::default();
let window = Window::new(100,100,WIDTH,HEIGHT,"raster");

app.run().unwrap();
}
