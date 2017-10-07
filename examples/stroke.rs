// stroke.rs        Example stroking a figure
//
// Copyright (c) 2017  Douglas P Lau
//
extern crate footile;

use footile::PlotterBuilder;

fn main() {
    let mut p = PlotterBuilder::new()
                               .width(64)
                               .height(64)
                               .build();
    p.pen_width(5f32, false)
     .move_to(16f32, 48f32)
     .line_to(32f32, 0f32)
     .line_to(-16f32, -32f32)
     .close()
     .stroke();
    p.mask().write_png("./stroke.png").unwrap();
}
