use cgmath::{vec3, vec4, Matrix4, Vector3, Vector4};
use std::ops::Mul;

use sdl2::rect::Point;

pub fn get_clipped(
    point: (i32, i32, i32),
    global_state_vector: Vector4<f64>,
    rot_y: Matrix4<f64>,
    rot_x: Matrix4<f64>,
    projection_matrix: Matrix4<f64>,
) -> Vector3<f64> {
    let rotated_global_state_vector = rot_y.mul(global_state_vector);
    let vectorised_point = from_edge_tuple(point);
    let point_rotated_around_y = rot_y.mul(vectorised_point);
    let projected_point =
        projection_matrix.mul(rot_x.mul(point_rotated_around_y + rotated_global_state_vector));
    return clip(projected_point);
}
fn from_edge_tuple(v: (i32, i32, i32)) -> Vector4<f64> {
    return vec4(v.0 as f64, v.1 as f64, v.2 as f64, 1.);
}

fn clip(v: Vector4<f64>) -> Vector3<f64> {
    return vec3(v.x / v.w, v.y / v.w, v.z / v.w);
}

pub fn project_onto(v: Vector3<f64>, height: f64, width: f64, projection_r: f64) -> Point {
    return Point::new(
        ((v.x * width) / (projection_r * v.z) + width / 2.) as i32,
        ((v.y * height) / (projection_r * v.z) + height / 2.) as i32,
    );
}

pub fn not_drawable(edge: (Vector3<f64>, Vector3<f64>)) -> bool {
    return edge.0.z < 1. || edge.1.z < 1.
}