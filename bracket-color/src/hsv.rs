use crate::prelude::{RGB, RGBA};
use std::convert::From;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an H/S/V triplet, in the range 0..1 (32-bit float)
/// This can provide for a more natural color progression, and provides
/// compatibility with HSV-based color systems.
pub struct HSV {
    /// Hue (range 0..1)
    pub h: f32,
    /// Saturation (range 0..1)
    pub s: f32,
    /// Value (range 0..1)
    pub v: f32,
}

/// Support conversion from RGB
impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        rgb.to_hsv()
    }
}

/// Support conversion from RGBA
impl From<RGBA> for HSV {
    fn from(rgba: RGBA) -> Self {
        rgba.to_rgb().to_hsv()
    }
}

impl HSV {
    /// Constructs a new, zeroed (black) HSV triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    /// Constructs a new HSV color, from 3 32-bit floats
    ///
    /// # Arguments
    ///
    /// * `h` - The hue (0..1) to use.
    /// * `s` - The saturation (0..1) to use.
    /// * `v` - The value (0..1) to use.
    #[inline]
    #[must_use]
    pub const fn from_f32(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
    }

    /// Converts to an RGBA value with a specified alpha level
    #[inline]
    #[must_use]
    pub fn to_rgba(&self, alpha: f32) -> RGBA {
        self.to_rgb().to_rgba(alpha)
    }

    /// Converts an HSV triple to an RGB triple
    #[inline]
    #[must_use]
    #[allow(clippy::many_single_char_names)] // I like my short names for this one
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_rgb(&self) -> RGB {
        let h = self.h;
        let s = self.s;
        let v = self.v;

        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;

        let i = f32::floor(h * 6.0) as i32;
        let f = h * 6.0 - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            // Catch-all; this shouldn't happen
            _ => {}
        }

        RGB::from_f32(r, g, b)
    }

    /// Progress smoothly between two colors, in the HSV color space.
    ///
    /// # Arguments
    ///
    /// * `color` - the target color.
    /// * `percent` - the percentage (0..1) of the starting (self) and target color to use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::named(RED);
    /// let blue = RGB::named(YELLOW);
    /// let color = red.lerp(blue, 0.5);
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (color.h - self.h, color.s - self.s, color.v - self.v);
        Self {
            h: self.h + range.0 * percent,
            s: self.s + range.1 * percent,
            v: self.v + range.2 * percent,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_utils::*;
    use rstest::rstest;

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn make_hsv_minimal() {
        let black = HSV::new();
        assert!(black.h < f32::EPSILON);
        assert!(black.s < f32::EPSILON);
        assert!(black.v < f32::EPSILON);
    }

    #[test]
    fn construct_hsv_from_f32() {
        let hsv = HSV::from_f32(0.5, 0.25, 0.75);
        assert_hsv_eq(hsv, 0.5, 0.25, 0.75);
    }

    #[rstest]
    #[case(HSV::from_f32(0.0, 1.0, 1.0), 1.0, 0.0, 0.0)]
    #[case(HSV::from_f32(120.0 / 360.0, 1.0, 1.0), 0.0, 1.0, 0.0)]
    #[case(HSV::from_f32(240.0 / 360.0, 1.0, 1.0), 0.0, 0.0, 1.0)]
    fn convert_hsv_to_rgb_primary_colors(
        #[case] hsv: HSV,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
    ) {
        assert_rgb_eq(hsv.to_rgb(), r, g, b);
    }

    #[rstest]
    #[case(0.0)]
    #[case(0.5)]
    #[case(1.0)]
    fn convert_hsv_to_rgba_preserves_alpha(#[case] alpha: f32) {
        let hsv = HSV::from_f32(0.0, 1.0, 1.0);
        let rgba = hsv.to_rgba(alpha);

        assert_rgba_eq(rgba, 1.0, 0.0, 0.0, alpha);
    }

    #[rstest]
    #[case(HSV::from_f32(0.0, 0.0, 0.0), 0.0, 0.0, 0.0)]
    #[case(HSV::from_f32(0.0, 0.0, 1.0), 1.0, 1.0, 1.0)]
    #[case(HSV::from_f32(0.0, 0.0, 0.5), 0.5, 0.5, 0.5)]
    fn convert_hsv_grayscale_to_rgb(
        #[case] hsv: HSV,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
    ) {
        assert_rgb_eq(hsv.to_rgb(), r, g, b);
    }

    #[test]
    fn from_rgb_and_rgba_delegate_to_hsv_conversion() {
        let rgb = RGB::from_f32(1.0, 0.0, 0.0);
        let rgba = RGBA::from_f32(1.0, 0.0, 0.0, 0.5);

        assert_hsv_eq(HSV::from(rgb), 0.0, 1.0, 1.0);
        assert_hsv_eq(HSV::from(rgba), 0.0, 1.0, 1.0);
    }

    #[test]
    // Test the lerp function
    fn test_lerp() {
        let black = RGB::named(BLACK).to_hsv();
        let white = RGB::named(WHITE).to_hsv();
        assert!(black.lerp(white, 0.0) == black);
        assert!(black.lerp(white, 1.0) == white);
    }
}
