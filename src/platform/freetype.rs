// font-kit/src/platform/freetype.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use euclid::{Point2D, Rect, Vector2D};
use freetype::freetype::{
    FT_Done_Face,
    FT_Face,
};
use lyon_path::builder::PathBuilder;
use std::{
    fs::File,
    sync::Arc,
};

use descriptor::Descriptor;
use font::Metrics;

pub type NativeFont = FT_Face;

pub struct Font {
    face: FT_Face,
    font_data: Option<FontData>,
}

impl Font {
    pub fn from_bytes(font_data: Arc<Vec<u8>>) -> Result<Font, ()> {
        unimplemented!()
    }

    pub fn from_file(mut file: File) -> Result<Font, ()> {
        unimplemented!()
    }

    pub fn from_native_font(face: NativeFont) -> Font {
        Font {
            face,
            font_data: None,
        }
    }

    #[inline]
    pub fn as_native_font(&self) -> NativeFont {
        self.face
    }

    pub fn descriptor(&self) -> Descriptor {
        unimplemented!()
    }

    pub fn glyph_for_char(&self, character: char) -> Option<u32> {
        unimplemented!()
    }

    pub fn outline<B>(&self, glyph_id: u32, path_builder: &mut B) -> Result<(), ()>
    where B: PathBuilder {
        unimplemented!()
    }

    pub fn typographic_bounds(&self, glyph_id: u32) -> Rect<f32> {
        unimplemented!()
    }

    pub fn advance(&self, glyph_id: u32) -> Vector2D<f32> {
        unimplemented!()
    }

    pub fn origin(&self, glyph_id: u32) -> Point2D<f32> {
        unimplemented!()
    }

    pub fn metrics(&self) -> Metrics {
        unimplemented!()
    }

    #[inline]
    pub fn font_data(&self) -> Option<FontData> {
        self.font_data.clone()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            FT_Done_Face(self.face);
        }
    }
}

pub type FontData = Arc<Vec<u8>>;
