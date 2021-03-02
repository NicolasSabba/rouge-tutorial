/*
  Game state module -> Play state.
*/
use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
use crate::components::{Position, Player};
use crate::render::Renderable;

// ecs set up
pub struct State {
  pub ecs: World
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    // Clear screen
    ctx.cls();

    player_input(self, ctx);

    // Draw function
    let positions = self.ecs.read_storage::<Position>();
    let renderables = self.ecs.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
      ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
  }
}

// Game state functions
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();

  for (_player, pos) in (&mut players, &mut positions).join() {
    pos.x = min(79, max(0, pos.x + delta_x));
    pos.y = min(49, max(0, pos.y + delta_y));
  }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
  // Player movement
  match ctx.key {
    None => {} // Nothing happened
    Some(key) => match key {
      VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
      VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
      VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
      VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
      _ => {}
    },
  }
}
