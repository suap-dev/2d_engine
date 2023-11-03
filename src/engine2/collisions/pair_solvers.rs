use std::f32::consts::PI;

use nalgebra_glm::Vec2;

use crate::engine2::verlet_object::VerletObject;

fn solve_owned(
    mut obj0: VerletObject,
    mut obj1: VerletObject,
) -> Option<(VerletObject, VerletObject)> {
    if let Some((adjustment0, adjustment1)) = get_adjustments(obj0.into(), obj1.into()) {
        obj0.shift(adjustment0);
        obj1.shift(adjustment1);
        Some((obj0, obj1))
    } else {
        None
    }
}

fn solve_ref_mut(obj0: &mut VerletObject, obj1: &mut VerletObject) {
    if let Some((adjustment0, adjustment1)) = get_adjustments((*obj0).into(), (*obj1).into()) {
        obj0.shift(adjustment0);
        obj1.shift(adjustment1);
    }
}

fn solve_indexed(objects: &mut [VerletObject], obj0_idx: usize, obj1_idx: usize) {
    let obj0 = &objects[obj0_idx];
    let obj1 = &objects[obj1_idx];

    if let Some((adjustment0, adjustment1)) = get_adjustments((*obj0).into(), (*obj1).into()) {
        objects[obj0_idx].shift(adjustment0);
        objects[obj1_idx].shift(adjustment1);
    }
}

pub struct Ball {
    center: Vec2,
    radius: f32,
}
impl From<VerletObject> for Ball {
    fn from(value: VerletObject) -> Self {
        Self {
            center: value.get_center(),
            radius: value.get_radius(),
        }
    }
}

pub fn get_adjustments(b0: Ball, b1: Ball) -> Option<(Vec2, Vec2)> {
    let centers_distance = b1.center.metric_distance(&b0.center);
    let radius_sum = b1.radius + b0.radius;

    if centers_distance < radius_sum {
        let delta_versor = (b1.center - b0.center).normalize();
        let mass0 = PI * b0.radius.powi(2);
        let mass1 = PI * b1.radius.powi(2);

        let adjustment_vector = delta_versor * (radius_sum - centers_distance);

        let adjustment0 = -(mass1 / (mass0 + mass1)) * adjustment_vector;
        let adjustment1 = (mass0 / (mass0 + mass1)) * adjustment_vector;

        Some((adjustment0, adjustment1))
    } else {
        None
    }
}
