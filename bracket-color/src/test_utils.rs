use crate::prelude::{HSV, RGB, RGBA};

pub fn assert_approx_eq(left: f32, right: f32) {
    assert!((left - right).abs() < 1e-6);
}

pub fn assert_rgb_eq(rgb: RGB, r: f32, g: f32, b: f32) {
    assert_approx_eq(rgb.r, r);
    assert_approx_eq(rgb.g, g);
    assert_approx_eq(rgb.b, b);
}

pub fn assert_rgba_eq(rgba: RGBA, r: f32, g: f32, b: f32, a: f32) {
    assert_approx_eq(rgba.r, r);
    assert_approx_eq(rgba.g, g);
    assert_approx_eq(rgba.b, b);
    assert_approx_eq(rgba.a, a);
}

pub fn assert_hsv_eq(hsv: HSV, h: f32, s: f32, v: f32) {
    assert_approx_eq(hsv.h, h);
    assert_approx_eq(hsv.s, s);
    assert_approx_eq(hsv.v, v);
}
