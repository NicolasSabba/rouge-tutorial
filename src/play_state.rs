/*
  Game state module -> Play state.
*/
use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use crate::components::{Position, Player, Renderable};
use crate::map::{Map, TileType};

// ecs set up
pub struct PlayState {
  pub ecs: World
}

impl PlayState {
  pub fn new() -> Self {
    // Create world
    let mut play_state = PlayState {
      ecs: World::new()
    };

    // Register components
    play_state.ecs.register::<Position>();
    play_state.ecs.register::<Renderable>();
    play_state.ecs.register::<Player>();

    // Register resources
    play_state.ecs.insert(Map::new());

    // Register entities
    play_state.ecs
      .create_entity()
      .with(Position { x: 40, y: 25 })
      .with(Renderable {
        glyph: rltk::to_cp437('@'),
        fg: RGB::named(rltk::YELLOW),
        bg: RGB::named(rltk::BLACK),
      }
      ).with(Player {}
    ).build();

    return play_state;
  }
}

impl GameState for PlayState {
  fn tick(&mut self, ctx: &mut Rltk) {
    // Clear screen
    ctx.cls();

    // Check user input
    player_input(self, ctx);

    // Draw map
    let map = self.ecs.fetch::<Map>();
    draw_map(&map, ctx);

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

fn player_input(gs: &mut PlayState, ctx: &mut Rltk) {
  // Player movement
  match ctx.key {
    None => {} // Nothing happened
    Some(key) => match key {
      VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
      VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
      VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
      VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
      VirtualKeyCode::Escape => ctx.quit(),
      _ => {}
    },
  }
}


fn draw_map(map: &Map, ctx: &mut Rltk) {
  let mut y = 0;
  let mut x = 0;
  for tile in map.0.iter() {
    // Render a tile depending upon the tile type
    match tile {
      TileType::Floor => {
        ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
      }
      TileType::Wall => {
        ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('█'));
      }
    }

    // Move the coordinates
    x += 1;
    if x > 79 {
      x = 0;
      y += 1;
    }
  }
}