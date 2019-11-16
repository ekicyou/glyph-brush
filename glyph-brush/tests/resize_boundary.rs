// tests/resize_boundary.rs
use glyph_brush::{
    *, rusttype::*,
};

const FONT: &[u8] = include_bytes!("../../fonts/DejaVuSans.ttf");

#[test]
fn glyph_resize_boundary_102() {
    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(FONT.to_vec()).build();

    let section = Section {
        text: "Lorem ipsum\nQuality: 103",
        // Change these from 102 to 103 to see the differences
        // Numbers 104, 105,.. are also inconsistent
        scale: Scale::uniform(102.0),
        ..<_>::default()
    };
    glyph_brush.queue(section);

    let mut resized = None;

    let mut texels = vec![];
    match glyph_brush.process_queued(
        |bounding, texel| {
            texels.push((bounding, texel.to_owned()));
        },
        |_| {},
    ) {
        Ok(_) => {}
        Err(BrushError::TextureTooSmall { suggested }) => {
            resized = Some(dbg!(suggested));
        }
    }

    assert_eq!(texels.len(), 21);

    assert_eq!(resized, None);
}


#[test]
fn glyph_resize_boundary_103() {
    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(FONT.to_vec()).build();

    let section = Section {
        text: "Lorem ipsum\nQuality: 103",
        // Change these from 102 to 103 to see the differences
        // Numbers 104, 105,.. are also inconsistent
        scale: Scale::uniform(103.0),
        ..<_>::default()
    };
    glyph_brush.queue(section);

    let mut resized = None;

    let mut texels = vec![];
    match glyph_brush.process_queued(
        |bounding, texel| {
            texels.push((bounding, texel.to_owned()));
        },
        |_| {},
    ) {
        Ok(_) => {}
        Err(BrushError::TextureTooSmall { suggested }) => {
            resized = Some(dbg!(suggested));
        }
    }

    assert_eq!(texels.len(), 0);

    assert_eq!(resized, Some((512, 512)));
    glyph_brush.resize_texture(512, 512);

    match glyph_brush.process_queued(
        |bounding, texel| {
            texels.push((bounding, texel.to_owned()));
        },
        |_| {},
    ) {
        Ok(_) => {}
        Err(BrushError::TextureTooSmall { .. }) => panic!("should fit"),
    }

    assert_eq!(texels.len(), 21);
}
