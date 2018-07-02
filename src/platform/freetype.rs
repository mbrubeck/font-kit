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
    FT_Get_Char_Index,
    FT_Init_FreeType,
    FT_Library,
    FT_New_Memory_Face,
};
use lyon_path::builder::PathBuilder;
use std::{
    fs::File,
    ptr,
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
    pub fn from_bytes(bytes: Arc<Vec<u8>>) -> Result<Font, ()> {
        let font_index = 0; // TODO: Support other font indices.
        let library = Library::new()?; // TODO: Cache the FT_Library.
        let mut face = ptr::null_mut();

        unsafe {
            let result = FT_New_Memory_Face(
                library.library,
                bytes.as_ptr(),
                bytes.len() as i64,
                font_index,
                &mut face,
            );
            if result != 0 || face.is_null() {
                return Err(());
            }
        }
        Ok(Font {
            face,
            font_data: Some(bytes),
        })
    }

    // TODO: Change to take a filename instead of a File?
    pub fn from_file(mut file: File) -> Result<Font, ()> {
        unimplemented!()
    }

    pub fn from_native_font(face: NativeFont) -> Font {
        assert!(!face.is_null());
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
        let idx = unsafe { FT_Get_Char_Index(self.face, character as u64) };
        if idx == 0 {
            return None
        }
        Some(idx)
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

struct Library {
    library: FT_Library,
}

impl Library {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let mut library = ptr::null_mut();
            let result = FT_Init_FreeType(&mut library);
            if result != 0 {
                return Err(())
            }
            Ok(Self { library })
        }
    }
}
