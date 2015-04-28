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
                      + *c3 * (t as f32).powi(3))
             .collect()
}
pub fn bezier_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| *c0 * ((1.0 - t).powi(3))
                      + *c1 * 3.0*t*(1.0-t)*(1.0-t)
                      + *c2 * 3.0*t*t*(1.0-t)
                      + *c3 * (t as f32).powi(3))
             .collect()
}

pub fn catmull_size(part: i32, c0: &f32, c1: &f32, c2: &f32, c3: &f32) -> Vec<f32>{
    let p1 = *c1 + (*c2-*c0) / 6.0;
    let p2 = *c2 + (*c1-*c3) / 6.0;
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| *c1 * ((1.0 - t).powi(3))
                        + p1 * 3.0*t*(1.0-t)*(1.0-t)
                        + p2 * 3.0*t*t*(1.0-t)
                        + *c2 * (t as f32).powi(3))
             .collect()
}

pub fn catmull_dots2(part: i32, c0: &Vec2<f32>, c1: &Vec2<f32>,
                    c2: &Vec2<f32>, c3: &Vec2<f32>) -> Vec<Vec2<f32>>{
    bezier_dots2(part, c1, &(*c1 + (*c2-*c0) / 6.0), &(*c2 + (*c1-*c3) / 6.0), c2)
}

pub fn catmull_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    bezier_dots3(part, c1, &(*c1 + (*c2-*c0) / 6.0), &(*c2 + (*c1-*c3) / 6.0), c2)
}

pub fn bspline_dots2(part: i32, c0: &Vec2<f32>, c1: &Vec2<f32>,
                    c2: &Vec2<f32>, c3: &Vec2<f32>) -> Vec<Vec2<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| (*c0 * ((1.0 - t).powi(3))
                      + *c1 * (3.0*t*t*t - 6.0*t*t +4.0)
                      + *c2 * (1.0 + 3.0*t + 3.0*t*t - 3.0*t*t*t)
                      + *c3 * (t*t*t)) / 6.0)
             .collect()
}
pub fn bspline_dots3(part: i32, c0: &Vec3<f32>, c1: &Vec3<f32>,
                    c2: &Vec3<f32>, c3: &Vec3<f32>) -> Vec<Vec3<f32>>{
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| (*c0 * ((1.0 - t).powi(3))
                      + *c1 * (3.0*t*t*t - 6.0*t*t +4.0)
                      + *c2 * (1.0 + 3.0*t + 3.0*t*t - 3.0*t*t*t)
                      + *c3 * (t*t*t)) / 6.0)
             .collect()
}

pub fn bezier_quats(part: i32, c0: &UnitQuat<f32>, c1: &UnitQuat<f32>,
                     c2: &UnitQuat<f32>, c3: &UnitQuat<f32>) -> Vec<UnitQuat<f32>>{
    let c0_1 = ((*c0).inv().unwrap() * *c1).rotation();
    let c1_2 = ((*c1).inv().unwrap() * *c2).rotation();
    let c2_3 = ((*c2).inv().unwrap() * *c3).rotation();
    (0..part).map(|t| 1.0 * (t as f32) / (part as f32))
             .map(|t: f32| (t,
                            *c0 * UnitQuat::new(c0_1 * t),
                            *c1 * UnitQuat::new(c1_2 * t),
                            *c2 * UnitQuat::new(c2_3 * t)))
             .map(|(t, q1, q2, q3)| (t,
                                     q1.slerp(&q2, t),
                                     q2.slerp(&q3, t)))
             .map(|(t, q12, q23)| q12.slerp(&q23, t))
             .collect()
}

pub fn catmull_quats(part: i32, c0: &UnitQuat<f32>, c1: &UnitQuat<f32>,
                     c2: &UnitQuat<f32>, c3: &UnitQuat<f32>) -> Vec<UnitQuat<f32>>{
    let a = (*c1) * UnitQuat::new(((*c0).inv().unwrap() * (*c2)).rotation() / 6.0);
    let b = (*c2) * UnitQuat::new(((*c1).inv().unwrap() * (*c3)).rotation() / -6.0);
    bezier_quats(part, c1, &a, &b, c2)
}
