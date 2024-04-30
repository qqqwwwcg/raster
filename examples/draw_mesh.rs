use fltk::{
    app::{self, event_key_down},
    enums::{Key, Mode},
    prelude::*,
    window::Window,
};

use raster::*;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 720;
const AMBIENT_LIGHT_INTENSITY: f32 = 0.2;
fn main() {
    let app = fltk::app::App::default();
    let mut window = Window::new(100, 100, WIDTH, HEIGHT, "raster");

    //diablo3
    let (vertexes, indices) = load_model("assets/diablo3_pose.obj");
    let base = BaseColorMap::new("assets/diablo3_pose_diffuse.bmp");
    let normal = NormalMap::new("assets/diablo3_pose_nm.bmp");
    let specular = SpecularMap::new("assets/diablo3_pose_spec.bmp");
    let material = Material::new(
        PhongMaterial::default(),
        BaseColor::Map(base),
        Some(normal),
        Some(specular),
    );
    let mesh = Mesh::new(vertexes, indices, material);

    // //red ball
    // let (vertexes, indices) = load_model("assets/RedBall.obj");
    // let material = Material::new(
    //     PhongMaterial::default(),
    //     BaseColor::Color([255, 0, 0]),
    //     None,
    //     None,
    // );
    // let mesh = Mesh::new(vertexes, indices, material);

    let bounding = mesh.get_bounding();
    let target: Vec3 = 0.5 * (bounding.0 + bounding.1);
    let frustum = Frustum::new(
        1.0,
        1000.0,
        60.0f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
    );
    let camera = Camera::new(frustum, Vec3::new(0.0, 0.0, 2.0), target);

    let vertex_shader = Box::new(|vertex: &Vertex, mvp: &Matrix4, model_mat: &Matrix4| {
        let world_position = model_mat
            .mul(vertex.position.upgrade())
            .perspective_divide();
        let world_normal = model_mat
            .inv()
            .unwrap()
            .transpose()
            .mul(vertex.normal.upgrade())
            .perspective_divide();

        let ndc_position = mvp.mul(vertex.position.upgrade()).perspective_divide();
        Uniform::new(world_position, world_normal, ndc_position, vertex.texcoord)
    });
    let fragment_shader = Box::new(
        |fragment: &Fragment,
         material: &Material,
         light: &PointLight,
         camera_position: &Vec3,
         model_mat: &Matrix4| {
            let n = if let Some(normal_map) = &material.normal {
                let model_normal = normal_map.get_normal(fragment.texcoord);
                model_mat
                    .inv()
                    .unwrap()
                    .transpose()
                    .mul(model_normal.upgrade())
                    .perspective_divide()
                    .normalize()
            } else {
                fragment.world_normal.normalize()
            };
            let l = (fragment.world_position - light.position).normalize();
            let length2 = (fragment.world_position - light.position).length2();
            let h = ((fragment.world_position - *camera_position).normalize() + l).normalize();

            let ambient_intensity = AMBIENT_LIGHT_INTENSITY * material.material.ambient;
            let diffuse_intensity =
                (light.intensity / length2) * n.dot(&l).max(0.0) * material.material.diffuse;
            let specular = if let Some(specular_map) = &material.specular {
                specular_map.get_specular(fragment.texcoord)
            } else {
                material.material.specular
            };

            let specular_intensity = (light.intensity / length2)
                * (n.dot(&h).max(0.0)).powf(material.material.specular_shininess)
                * specular;

            let light_intensity = ambient_intensity + diffuse_intensity + specular_intensity;
            let base_color = match &material.base {
                BaseColor::Color(color) => (*color).into(),
                BaseColor::Map(map) => map.get_color(fragment.texcoord),
            };
            let color = [
                (light_intensity.x * base_color.x * 255.0) as u8,
                (light_intensity.y * base_color.y * 255.0) as u8,
                (light_intensity.z * base_color.z * 255.0) as u8,
            ];

            ShadedFragment::new(fragment.screen_pos, fragment.depth, color)
        },
    );
    let shader = Shader::new(vertex_shader, fragment_shader);
    let mut render = Render::new(WIDTH as u32, HEIGHT as u32, camera, shader);

    let light = PointLight::new(Vec3::new(-5.0, 5.0, 5.0), 200.0);
    let mut dirty = true;
    window.draw(move |_| {
        // event handle
        {
            //rotation
            let camera = render.get_camera();
            if event_key_down(Key::from_char('a')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::Y, -0.1));
                dirty = true;
            }
            if event_key_down(Key::from_char('d')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::Y, 0.1));
                dirty = true;
            }
            if event_key_down(Key::from_char('s')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::X, -0.1));
                dirty = true;
            }
            if event_key_down(Key::from_char('w')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::X, 0.1));
                dirty = true;
            }
            if event_key_down(Key::from_char('q')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::Z, -0.1));
                dirty = true;
            }
            if event_key_down(Key::from_char('e')) {
                camera.rotation_around(target, Quat::from_axis_angle(Vec3::Z, 0.1));
                dirty = true;
            }

            //scale
            if event_key_down(Key::from_char('r')) {
                camera.scale(-0.1);
                dirty = true;
            }
            if event_key_down(Key::from_char('t')) {
                camera.scale(0.1);
                dirty = true;
            }
        }

        if dirty {
            render.reset();
            render.draw(&mesh, &light, Matrix4::ident());
        }
        dirty = false;

        let data = render.get_frame();
        fltk::draw::draw_image(&data, 0, 0, WIDTH, HEIGHT, fltk::enums::ColorDepth::Rgb8).unwrap();
    });

    window.end();
    app::set_visual(Mode::Rgb8).unwrap();
    window.show();

    fltk::app::add_idle3(move |_| {
        window.redraw();
    });

    //run event loop
    app.run().unwrap();
}
