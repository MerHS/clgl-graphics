//use std::error::Error;
//use std::fs::File;
//use std::io::prelude::*;
//#[macro_use]
//extern crate glium;
extern crate nalgebra as na;
#[macro_use]
extern crate glium;
extern crate glutin;

use std::env;

pub mod parse_dat;
pub mod section;
pub mod spline;

use section::Object;
use spline::*;
use na::Vec2;

fn main() {
    use glium::DisplayBuild;

    // parsing arguments
    let mut args: Vec<String> = Vec::new();

    for arg in env::args_os().skip(1) {
        match arg.into_string(){
            Ok(arg_str) => args.push(arg_str),
            Err(_) => continue,
        }
    }

    let mut objs: Vec<Object> = Vec::new();
    for arg in args{
        match parse_dat::load(&arg) {
            Ok(f) => objs.push(parse_dat::parse_file(f)),
            Err(_) => println!("not exists {}", arg),
        }
    }

    if objs.len() < 1 {
        panic!("cannot find the file of object from filenames");
    }

    for obj in objs.iter(){
        println!("spl : {:?}", obj.spline);
        println!("sectn : {:?}", obj.sect_n);
        println!("point: {:?}", obj.point_n);
        for sect in obj.sect.iter(){
            println!("scale : {:?}", sect.scale);
            println!("rot_angle : {:?}", sect.rot_angle);
            println!("{:?}",bezier_dots2(4,
                                        &sect.cont_pos[0],
                                        &sect.cont_pos[1],
                                        &sect.cont_pos[2],
                                        &sect.cont_pos[3]));
        }
    }

    // building Display
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
//        .with_dimensions(1024,1024)
//        .with_depth_buffer(32)
//        .with_title(format!("Rust Sweeper"))
//        .build_glium().unwrap();

}
