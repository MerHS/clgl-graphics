use na::{Vec2, Vec3, UnitQuat, Norm};
use std::borrow::ToOwned;
use std::fs::File;
use std::io::{Read,Result};
//use std::path::{Path, PathBuf};
use std::path::Path;
use std::env::current_exe;
use std::error::Error;

use section::{Section, Object, Spline};

pub fn load<T: AsRef<Path>>(name: &T) -> Result<File>{
    let c_file = current_exe().unwrap();
    let c_dir = c_file.parent().unwrap();
    let mut res_release = c_dir.to_path_buf();
    res_release.push(&name);

    let f = File::open(&*res_release);
    if f.is_ok(){
        return f;
    }

    // Debug channel
    let mut res_debug = c_dir.to_path_buf();
    res_debug.push("..");
    res_debug.push("..");
    res_debug.push("res");
    res_debug.push(&name);
    File::open(&*res_debug)
}

fn parse_str(args: &str) -> &str{
    args.split("#").next().unwrap().trim()
}

fn parse_num(args: &str) -> Vec<f32>{
    parse_str(args).split(' ')
                   .map(|x| x.parse::<f32>().unwrap())
                   .collect()
}

pub fn parse_file(mut f: File, name: &str) -> Object {
    let mut text = String::new();
    match f.read_to_string(&mut text){
        Err(why) => panic!("couldn't read {}:",
                           Error::description(&why)),
        Ok(_) => (),
    }

    let mut lns = text.lines_any();

    let spline = Spline::new(parse_str(lns.next().unwrap()));
    let sect_n = parse_str(lns.next().unwrap())
                 .parse::<i32>().unwrap();
    let point_n = parse_str(lns.next().unwrap())
                 .parse::<i32>().unwrap();
    let mut sect = Vec::with_capacity(sect_n as usize);

    for _ in 0..sect_n {
        let mut cont_pos = Vec::with_capacity(point_n as usize);
        for _ in 0..point_n {
            let c_pos: Vec<f32> = parse_num(lns.next().unwrap());
            cont_pos.push(Vec2::new(c_pos[0], c_pos[1]));
        }
        let scale = (parse_num(lns.next().unwrap()))[0];
        let rot = parse_num(lns.next().unwrap());
        let t_pos = parse_num(lns.next().unwrap());
        sect.push(
            Section{ cont_pos: cont_pos,
                     scale: scale,
                     rot: UnitQuat::new(
                         Vec3::new(rot[1], rot[2], rot[3]).normalize()
                         * (rot[0] as f32)),
                     pos: Vec3::new(t_pos[0], t_pos[1], t_pos[2])});
    }
    Object { name: (*name).to_owned(),
             spline: spline,
             sect_n: sect_n,
             point_n: point_n,
             sect: sect}
}

