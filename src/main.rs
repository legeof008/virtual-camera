extern crate sdl2;

mod integration;

use crate::integration::coversions::{get_clipped, not_drawable, project_onto};
use cgmath::{perspective, vec3, vec4, Deg, Matrix4};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ops::Mul;
use std::time::Duration;

const MAP_OF_VIEW: [((i32, i32, i32), (i32, i32, i32)); 48] = [
    ((0, 0, 5), (10, 0, 5)),
    ((10, 0, 5), (10, 20, 5)),
    ((0, 20, 5), (10, 20, 5)),
    ((0, 20, 5), (0, 0, 5)),
    ((0, 0, 10), (10, 0, 10)),
    ((10, 0, 10), (10, 20, 10)),
    ((10, 20, 10), (0, 20, 10)),
    ((0, 0, 10), (0, 20, 10)),
    ((0, 20, 5), (0, 20, 10)),
    ((10, 20, 5), (10, 20, 10)),
    ((10, 0, 5), (10, 0, 10)),
    ((0, 0, 5), (0, 0, 10)),
    ((20, 0, 5), (30, 0, 5)),
    ((30, 0, 5), (30, 20, 5)),
    ((20, 20, 5), (30, 20, 5)),
    ((20, 20, 5), (20, 0, 5)),
    ((20, 0, 10), (30, 0, 10)),
    ((30, 0, 10), (30, 20, 10)),
    ((30, 20, 10), (20, 20, 10)),
    ((20, 0, 10), (20, 20, 10)),
    ((20, 20, 5), (20, 20, 10)),
    ((30, 20, 5), (30, 20, 10)),
    ((30, 0, 5), (30, 0, 10)),
    ((20, 0, 5), (20, 0, 10)),
    ((0, 0, 25), (10, 0, 25)),
    ((10, 0, 25), (10, 10, 25)),
    ((0, 10, 25), (10, 10, 25)),
    ((0, 10, 25), (0, 0, 25)),
    ((0, 0, 30), (10, 0, 30)),
    ((10, 0, 30), (10, 10, 30)),
    ((10, 10, 30), (0, 10, 30)),
    ((0, 0, 30), (0, 10, 30)),
    ((0, 10, 25), (0, 10, 30)),
    ((10, 10, 25), (10, 10, 30)),
    ((10, 0, 25), (10, 0, 30)),
    ((0, 0, 25), (0, 0, 30)),
    ((20, 0, 25), (30, 0, 25)),
    ((30, 0, 25), (30, 10, 25)),
    ((20, 10, 25), (30, 10, 25)),
    ((20, 10, 25), (20, 0, 25)),
    ((20, 0, 30), (30, 0, 30)),
    ((30, 0, 30), (30, 10, 30)),
    ((30, 10, 30), (20, 10, 30)),
    ((20, 0, 30), (20, 10, 30)),
    ((20, 10, 25), (20, 10, 30)),
    ((30, 10, 25), (30, 10, 30)),
    ((30, 0, 25), (30, 0, 30)),
    ((20, 0, 25), (20, 0, 30)),
];

fn main() {
    let width = 800.;
    let height = 600.;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("vcam", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut global_state_vector = vec4(3., -2., 5., 0.);
    let mut projection_radius = 2.;
    let mut rotate_y = 0.;
    let mut rotate_x = 0.;

    let rotation_step = 45.;

    let proj = perspective(Deg(45.), 800. / 600., 0.1, 100.0);
    'game: loop {
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        let rot_x = Matrix4::from_axis_angle(vec3(1., 0., 0.), Deg(rotate_x));
        let rot_y = Matrix4::from_axis_angle(vec3(0., 1., 0.), Deg(-(rotate_y)));
        let rot_transform = Matrix4::from_axis_angle(vec3(0., 1., 0.), Deg(rotate_y));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => global_state_vector -= rot_transform.mul(vec4(1., 0., 0., 0.)),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => global_state_vector += rot_transform.mul(vec4(1., 0., 0., 0.)),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => global_state_vector -= rot_transform.mul(vec4(0., 0., 1., 0.)),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => global_state_vector += rot_transform.mul(vec4(0., 0., 1., 0.)),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => rotate_x += rotation_step,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => rotate_x -= rotation_step,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => rotate_y -= rotation_step,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => rotate_y += rotation_step,
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => projection_radius -= 0.1,
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => projection_radius += 0.1,
                _ => {}
            }
        }
        // Here I'll try to get a 3d rectangle to a 2d view
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for edge in MAP_OF_VIEW {
            let p1 = get_clipped(edge.0, global_state_vector, rot_y, rot_x, proj);
            let p1_projected = project_onto(p1, height, width, projection_radius);

            let p2 = get_clipped(edge.1, global_state_vector, rot_y, rot_x, proj);
            let p2_projected = project_onto(p2, height, width, projection_radius);

            if not_drawable((p1,p2)) {
                continue;
            }
            canvas.draw_line(p1_projected, p2_projected).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
