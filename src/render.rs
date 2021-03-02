/*
  Render module
*/

use specs::prelude::*;
use specs_derive::Component;
use rltk::RGB;

// Render component
#[derive(Component)]
pub struct Renderable {
  pub glyph: rltk::FontCharType,
  pub fg: RGB,
  pub bg: RGB,
}

impl Renderable {
  pub fn new(glyph: rltk::FontCharType, fg: RGB, bg: RGB) -> Self {
    Self {
      glyph,
      fg,
      bg,
    }
  }
}
