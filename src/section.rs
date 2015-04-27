use na::*;
use std::ascii::AsciiExt;
use spline::*;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Spline {
    Bspline,
    Catmull,
    Natural,
}

pub struct Section {
    pub cont_pos: Vec<Vec2<f32>>,
    pub scale: f32,
    pub rot: UnitQuat<f32>,
    pub pos: Vec3<f32>
}

pub struct Object {
    pub name: String,
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

impl Object{
    pub fn make_swept_file(&self, s_part: i32, b_part: i32){
        let ref sect = self.sect;
        let path_str = &format!("{}.obj",self.name);
        let path = Path::new(path_str);
        let mut file = match File::create(&path){
            Err(why) => panic!("Couldn't create {}", Error::description(&why)),
            Ok(file) => file,
        };
        file.write_all(format!("{} {}\n", (self.sect_n -1) * s_part + 1,
                               self.point_n * b_part).as_bytes());
        if self.sect_n == 2{
            let spline_s= Section::make_spline(s_part, &sect[0], &sect[0], &sect[1], &sect[1]);
            for s in spline_s {
                let _ = match (*self).spline {
                    Spline::Bspline => file.write_all(s.to_vertex_bs(b_part).as_bytes()),
                    Spline::Catmull => file.write_all(s.to_vertex_cm(b_part).as_bytes()),
                    Spline::Natural => file.write_all(s.to_vertex_nt(b_part).as_bytes()),
                };
            }
        } else if self.sect_n == 3 {
            let spline_s0 = Section::make_spline(s_part, &sect[0], &sect[0], &sect[1], &sect[2]);
            let spline_s1 = Section::make_spline(s_part, &sect[0], &sect[1], &sect[2], &sect[2]);
            for s in spline_s0 {
               let _ =  match (*self).spline {
                    Spline::Bspline => file.write_all(s.to_vertex_bs(b_part).as_bytes()),
                    Spline::Catmull => file.write_all(s.to_vertex_cm(b_part).as_bytes()),
                    Spline::Natural => file.write_all(s.to_vertex_nt(b_part).as_bytes()),
                };
            }
            for s in spline_s1 {
                let _ = match (*self).spline {
                    Spline::Bspline => file.write_all(s.to_vertex_bs(b_part).as_bytes()),
                    Spline::Catmull => file.write_all(s.to_vertex_cm(b_part).as_bytes()),
                    Spline::Natural => file.write_all(s.to_vertex_nt(b_part).as_bytes()),
                };
            }
        } else {
            for j in (0..(self.sect_n-3)){
                let i = j as usize;
                let spline_s = Section::make_spline(s_part, &sect[i], &sect[i+1],
                                                    &sect[i+2], &sect[i+3]);
                for s in spline_s {
                    let _ = match (*self).spline {
                        Spline::Bspline => file.write_all(s.to_vertex_bs(b_part).as_bytes()),
                        Spline::Catmull => file.write_all(s.to_vertex_cm(b_part).as_bytes()),
                        Spline::Natural => file.write_all(s.to_vertex_nt(b_part).as_bytes()),
                    };
                }
            }
        }
        let _ = match (*self).spline {
            Spline::Bspline => file.write_all(sect[(self.sect_n -1) as usize].to_vertex_bs(b_part).as_bytes()),
            Spline::Catmull => file.write_all(sect[(self.sect_n -1) as usize].to_vertex_cm(b_part).as_bytes()),
            Spline::Natural => file.write_all(sect[(self.sect_n -1) as usize].to_vertex_nt(b_part).as_bytes()),
        };
    }
}

impl Section{
    pub fn make_spline(part: i32, s0: &Section, s1: &Section,
                       s2: &Section, s3: &Section)-> Vec<Section> {
        let mut sects: Vec<Section> = Vec::with_capacity(part as usize);
        let mut cont_pos: Vec<Vec<Vec2<f32>>> = Vec::with_capacity(part as usize);
        let s = (*s0).cont_pos.len();
        for _ in (0..part){
            cont_pos.push(Vec::with_capacity(s));
        }
        for j in (0..s){
            let mut i: usize = 0;
            for p in catmull_dots2(part, &((*s0).cont_pos[j])
                                       , &((*s1).cont_pos[j])
                                       , &((*s2).cont_pos[j])
                                       , &((*s3).cont_pos[j])){
                cont_pos[i].push(p);
                i += 1;
            }
        }
        let scale = catmull_size(part, &(*s0).scale, &(*s1).scale,
                                      &(*s2).scale, &(*s3).scale);
        let rot = catmull_quats(part, &(*s0).rot, &(*s1).rot,
                                      &(*s2).rot, &(*s3).rot);
        let pos = catmull_dots3(part, &(*s0).pos, &(*s1).pos,
                                      &(*s2).pos, &(*s3).pos);
        for i in (0..(part as usize)){
            let mut cont_copy: Vec<Vec2<f32>> = Vec::with_capacity(s);
            for cc in cont_pos[i].iter(){
                cont_copy.push(*cc);
            }
            sects.push(
                Section{
                    cont_pos: cont_copy,
                    scale: scale[i],
                    rot: rot[i],
                    pos: pos[i]
                });
        }

        sects
    }
    pub fn to_vertex_bs(&self, b_part: i32) -> String {
        let c_len = (*self).cont_pos.len();
        let mut cont_b: Vec<Vec3<f32>> = Vec::with_capacity(c_len);
        let ref c_pos = (*self).cont_pos;
        for v in c_pos {
            let v3 = (*self).rot.rotate(&Vec3::new(v.x, 0f32, v.y));
            cont_b.push((*self).pos + (v3 * (*self).scale));
        }

        let mut vertex = String::new();
        for i0 in (0..c_len){
            let (i1, i2, i3) = ((c_len+i0+1)%c_len, (c_len+i0+2)%c_len, (c_len+i0+3)%c_len);
            let v_part = bspline_dots3(b_part, &cont_b[i0], &cont_b[i1], &cont_b[i2], &cont_b[i3]);
            for v in v_part{
                vertex.push_str(&format!("{} {} {}\n", v.x, v.y, v.z));
            }
        }
        vertex
    }
    pub fn to_vertex_cm(&self, b_part: i32) -> String {
        let c_len = (*self).cont_pos.len();
        let mut cont_b: Vec<Vec3<f32>> = Vec::with_capacity(c_len);
        let ref c_pos = (*self).cont_pos;
        for v in c_pos {
            let v3 = (*self).rot.rotate(&Vec3::new(v.x, 0f32, v.y));
            cont_b.push((*self).pos + (v3 * (*self).scale));
        }

        let mut vertex = String::new();
        for i0 in (0..c_len){
            let (i1, i2, i3) = ((c_len+i0+1)%c_len, (c_len+i0+2)%c_len, (c_len+i0+3)%c_len);
            let v_part = catmull_dots3(b_part, &cont_b[i0], &cont_b[i1], &cont_b[i2], &cont_b[i3]);
            for v in v_part{
                vertex.push_str(&format!("{} {} {}\n", v.x, v.y, v.z));
            }
        }
        vertex

    }
    pub fn to_vertex_nt(&self, b_part:i32) -> String{
        unimplemented!();
        //String::new()
    }
}
