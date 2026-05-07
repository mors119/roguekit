use crate::prelude::RGB;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

/// Structure representing the components of one color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpColor {
    /// Red component 0..255
    pub r: u8,
    /// Green component 0..255
    pub g: u8,
    /// Blue component 0..255
    pub b: u8,
}

impl From<RGB> for XpColor {
    fn from(rgb: RGB) -> Self {
        rgb.to_xp()
    }
}

impl XpColor {
    /// deepest black
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    /// color 0xff00ff (hot pink) is regarded as transparent
    pub const TRANSPARENT: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };

    /// Construct a new color from r,g,b values
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Return whether this color is considered transparent (if this is the background color of a
    /// cell, the layer below it will see through)
    #[inline]
    #[must_use]
    pub fn is_transparent(self) -> bool {
        self == Self::TRANSPARENT
    }

    /// Read a RGB color from a `ReadBytesExt`
    ///
    /// # Errors
    #[inline]
    pub fn read<T: ReadBytesExt>(rdr: &mut T) -> io::Result<Self> {
        let r = rdr.read_u8()?;
        let g = rdr.read_u8()?;
        let b = rdr.read_u8()?;
        Ok(Self { r, g, b })
    }

    /// Write a RGB color to a `WriteBytesExt`
    ///
    /// # Errors
    #[inline]
    pub fn write<T: WriteBytesExt>(self, wr: &mut T) -> io::Result<()> {
        wr.write_u8(self.r)?;
        wr.write_u8(self.g)?;
        wr.write_u8(self.b)?;
        Ok(())
    }
}

impl From<XpColor> for RGB {
    fn from(xp: XpColor) -> Self {
        RGB::from_xp(xp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn new_xp_color_stores_components() {
        let color = XpColor::new(1, 2, 3);

        assert_eq!(color.r, 1);
        assert_eq!(color.g, 2);
        assert_eq!(color.b, 3);
    }

    #[test]
    fn transparent_color_is_detected() {
        assert!(XpColor::TRANSPARENT.is_transparent());
        assert!(!XpColor::BLACK.is_transparent());
        assert!(!XpColor::new(255, 0, 254).is_transparent());
    }

    #[test]
    fn read_xp_color_reads_three_bytes() {
        let mut input = &[1_u8, 2, 3][..];

        let color = XpColor::read(&mut input).expect("valid RGB bytes");

        assert_eq!(color, XpColor::new(1, 2, 3));
    }

    #[test]
    fn read_xp_color_returns_error_on_short_input() {
        let mut input = &[1_u8, 2][..];

        let result = XpColor::read(&mut input);

        assert!(result.is_err());
    }

    #[test]
    fn write_xp_color_writes_three_bytes() {
        let color = XpColor::new(1, 2, 3);
        let mut output = Vec::new();

        color.write(&mut output).expect("write should succeed");

        assert_eq!(output, vec![1, 2, 3]);
    }

    #[test]
    fn convert_rgb_to_xp_color() {
        let rgb = RGB::from_f32(1.0, 128.0 / 255.0, 0.0);
        let xp = XpColor::from(rgb);

        assert_eq!(xp, XpColor::new(255, 128, 0));
    }

    #[test]
    fn convert_xp_color_to_rgb() {
        let xp = XpColor::new(255, 128, 0);
        let rgb = RGB::from(xp);

        assert_rgb_eq(rgb, 1.0, 128.0 / 255.0, 0.0);
    }
}
