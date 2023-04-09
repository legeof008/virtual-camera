extern crate sdl2;

mod integration;

use cgmath::{perspective, Matrix2, Matrix3, Matrix4, Perspective, Point2, Point3, Rad, Vector2, Vector3, Vector4, Deg};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

fn main() {
    let v = Vector2::new(1.0, 2.0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("vcam", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut scale = 300;
    let mut transform_x = 3;
    let mut transform_y = 3;
    let mut transform_z = 0;
    let step = 1;
    let step_z = 1;

    let view = Matrix4::look_to_rh(Point3::new(4, 3, 3), Vector3::new(0, 0, 0), Vector3::new(0, 1, 0));
    let proj = perspective( PI, 800. / 600. , 0.1, 100.0);

    'game: loop {
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => { break 'game; }
                Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => { transform_x -= step }
                Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => { transform_x += step }
                Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => { transform_z -= step_z }
                Event::KeyDown {
                    keycode: Some(Keycode::Down), ..
                } => { transform_z += step_z }

                _ => {}
            }
        }
        // Here I'll try to get a 3d rectangle to a 2d view
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let _p1 = Point3::new(5, 5, 5);
        let _p2 = Point3::new(0, 5, 5);
        let _p3 = Point3::new(5, 0, 5);
        let _p4 = Point3::new(0, 0, 5);
        let _p5 = Point3::new(5, 0, 10);
        let _p6 = Point3::new(5, 5, 10);
        let _p7 = Point3::new(5, 0, 10);
        let _p8 = Point3::new(0, 0, 10);

        let v1 = ((0, 0, 5), (5, 0, 5));
        let v2 = ((5, 0, 5), (5, 5, 5));
        let v3 = ((0, 5, 5), (5, 5, 5));
        let v4 = ((0, 5, 5), (0, 0, 5));

        let v5 = ((0, 0, 10), (5, 0, 10));
        let v6 = ((5, 0, 10), (5, 5, 10));
        let v7 = ((5, 5, 10), (0, 5, 10));
        let v8 = ((0, 0, 10), (0, 5, 10));

        let v9 = ((0, 5, 5), (0, 5, 10));
        let v10 = ((5, 5, 5), (5, 5, 10));
        let v11 = ((5, 0, 5), (5, 0, 10));
        let v12 = ((0, 0, 5), (0, 0, 10));

        let vertices = [v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12];

        'drawing: for vertice in vertices {
            let z_divider_1 = vertice.0.2 + transform_z;
            let z_divider_2 = vertice.1.2 + transform_z;
            if transform_z > z_divider_1 {
                continue;
            }
            canvas
                .draw_line(
                    Point::new(
                        (transform_x + vertice.0.0) * scale / z_divider_1,
                        (transform_y + vertice.0.1) * scale / z_divider_1,
                    ),
                    Point::new(
                        (transform_x + vertice.1.0) * scale / z_divider_2,
                        (transform_y + vertice.1.1) * scale / z_divider_2,
                    ),
                )
                .unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
