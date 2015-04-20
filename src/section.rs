use na::{Vec2, Vec3};
use std::ascii::AsciiExt;

#[derive(Debug)]
pub enum Spline {
    Bspline,
    Catmull,
    Natural,
}

pub struct Section {
    pub cont_pos: Vec<Vec2<f32>>,
    pub scale: f32,
    pub rot_angle: f32,
    pub rot_vec: Vec3<f32>,
    pub pos: Vec3<f32>
}

pub struct Object {
    pub spline: Spline,
    pub sect_n: i32,
    pub point_n: i32,
    pub sect: Vec<Section>
}

impl Spline{
    pub fn new(s: &str) -> Self {
        match &*(s.to_ascii_uppercase()) {
            "BSPLINE" => Spline::Bspline,
            "CATMULL_ROM" | "CATMULL" => Spline::Catmull,
            "NATNURAL" => Spline::Natural,
            t @ _ => { println!("there is no spline type {} / default type: BSPLINE",t);
                        Spline::Bspline},
        }
    }
}
