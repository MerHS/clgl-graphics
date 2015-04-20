use na::*;

pub trait Slerp<N: BaseFloat> {
    fn slerp(&self, &UnitQuat<N>, N) -> UnitQuat<N>;
}

impl<N: BaseFloat> Slerp<N> for UnitQuat<N>{
    fn slerp(&self, q2: &UnitQuat<N>, t: N) -> UnitQuat<N> {
        (*self) * UnitQuat::new(((*self).inv().unwrap() * *q2).rotation() * t)
    }
}

pub fn bezier_dots2(part: i32, c0: &Vec2<f32>, c1: &Vec2<f32>,
                    c2: &Vec2<f32>, c3: &Vec2<f32>) -> Vec<Vec2<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| *c0 * ((1.0 - t).powi(3))
                      + *c1 * 3.0*t*(1.0-t)*(1.0-t)
                      + *c2 * 3.0*t*t*(1.0-t)
                      + *c3 * (1.0*t).powi(3))
             .collect()
}
pub fn bezier_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| *c0 * ((1.0 - t).powi(3))
                      + *c1 * 3.0*t*(1.0-t)*(1.0-t)
                      + *c2 * 3.0*t*t*(1.0-t)
                      + *c3 * (1.0*t).powi(3))
             .collect()
}

pub fn catmull_dots2(part: i32, c0: &Vec2<f32>, c1: &Vec2<f32>,
                    c2: &Vec2<f32>, c3: &Vec2<f32>) -> Vec<Vec2<f32>>{
    bezier_dots2(part, c1, &(*c1 + (*c2-*c0) * 0.5), &(*c2 + (*c1-*c3) * 0.5), c2)
}

pub fn catmull_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    bezier_dots3(part, c1, &(*c1 + (*c2-*c0) * 0.5), &(*c2 + (*c1-*c3) * 0.5), c2)
}

pub fn bspline_dots2(part: i32, c0: &Vec2<f32>, c1: &Vec2<f32>,
                    c2: &Vec2<f32>, c3: &Vec2<f32>) -> Vec<Vec2<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| (*c0 * (1.0 - 3.0*t + 3.0*t*t - t*t*t)
                      + *c1 * (4.0 - 6.0*t*t + 3.0*t*t*t)
                      + *c2 * (1.0 + 3.0*t + 3.0*t*t + 3.0*t*t*t)
                      + *c3 * (t*t*t)) / 6.0)
             .collect()
}
pub fn bspline_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| (*c0 * (1.0 - 3.0*t + 3.0*t*t - t*t*t)
                      + *c1 * (4.0 - 6.0*t*t + 3.0*t*t*t)
                      + *c2 * (1.0 + 3.0*t + 3.0*t*t + 3.0*t*t*t)
                      + *c3 * (t*t*t)) / 6.0)
             .collect()
}

pub fn catmull_quats(part: i32, c0: &UnitQuat<f32>, c1: &UnitQuat<f32>,
                     c2: &UnitQuat<f32>, c3: &UnitQuat<f32>) -> Vec<UnitQuat<f32>>{
    let pf = part as f32;
    let c0_1 = ((*c0).inv().unwrap() * *c1).rotation();
    let c1_2 = ((*c1).inv().unwrap() * *c2).rotation();
    let c2_3 = ((*c2).inv().unwrap() * *c3).rotation();
    let mut c_seg = (0..part).map(|t| 1.0 * (t as f32) / pf)
                             .map(|t: f32| (*c0 * UnitQuat::new(c0_1 * t),
                                            *c1 * UnitQuat::new(c1_2 * t),
                                            *c2 * UnitQuat::new(c2_3 * t)));
    let mut ret_val: Vec<UnitQuat<f32>> = Vec::new();
    for t in (0..part) {
        let p = (t as f32) / pf;
        let (q1, q2, q3) = c_seg.next().unwrap();
        let q12 = q1.slerp(&q2, p);
        let q23 = q2.slerp(&q3, p);
        let q123 = q12.slerp(&q23, p);
        ret_val.push(q123);
    }
    ret_val
}









