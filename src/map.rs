/*
  Map module
*/

use crate::rect::Rect;
use rltk::RandomNumberGenerator;
use std::cmp::{min, max};

/// Possible floor tiles
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
  Wall,
  Floor,
}

/// Transform a 2d map coordinate to an index
pub fn map_2d_to_vec_idx(x: i32, y: i32) -> usize {
  (y as usize * 80) + x as usize
}

/// Transform a index to a 2d map coordinate
pub fn vec_idx_to_map_2d(index: usize) -> (i32, i32) {
  let x: usize = index % 80;
  (x as i32, ((index - x) / 80) as i32)
}

pub struct Map(pub Vec<TileType>);

impl Map {
  pub fn new_test() -> Self {
    let mut map = Map(vec![TileType::Floor; 80 * 50]);

    // Make the boundaries walls
    for x in 0..80 {
      map.0[map_2d_to_vec_idx(x, 0)] = TileType::Wall;
      map.0[map_2d_to_vec_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
      map.0[map_2d_to_vec_idx(0, y)] = TileType::Wall;
      map.0[map_2d_to_vec_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
      let x = rng.roll_dice(1, 79);
      let y = rng.roll_dice(1, 49);
      let idx = map_2d_to_vec_idx(x, y);
      if idx != map_2d_to_vec_idx(40, 25) {
        map.0[idx] = TileType::Wall;
      }
    }

    map
  }

  pub fn new() -> Self {
    // Create empty map full of wals
    let mut map = Map(vec![TileType::Wall; 80 * 50]);

    // Create a room vector
    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    // Create random generator
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
      // Create a room with random dimensions and random position
      let w = rng.range(MIN_SIZE, MAX_SIZE);
      let h = rng.range(MIN_SIZE, MAX_SIZE);
      let x = rng.roll_dice(1, 80 - w - 1) - 1;
      let y = rng.roll_dice(1, 50 - h - 1) - 1;
      let new_room = Rect::new(x, y, w, h);
      // Check if the room doesn't collide with previous rooms
      let mut ok = true;
      for other_room in rooms.iter() {
        if new_room.aabb(other_room) { ok = false }
      }
      // If doesn't collide
      if ok {
        // Add room
        apply_room_to_map(&new_room, &mut map);

        // If there is a previous room make a corridor
        if !rooms.is_empty() {
          let (x1, y1) = new_room.center();
          let (x2, y2) = rooms[rooms.len() - 1].center();
          // Roll a d100 and chose which corridor make first
          if rng.roll_dice(1, 100) < 50 {
            apply_horizontal_tunnel(&mut map, x1, x2, y2);
            apply_vertical_tunnel(&mut map, y1, y2, x1);
          } else {
            apply_vertical_tunnel(&mut map, y1, y2, x2);
            apply_horizontal_tunnel(&mut map, x1, x2, y1);
          }
        }

        // Push room to roms array
        rooms.push(new_room);
      }
    }

    map
  }
}

fn apply_room_to_map(room: &Rect, map: &mut Map) {
  for y in room.y1 + 1..=room.y2 {
    for x in room.x1 + 1..=room.x2 {
      map.0[map_2d_to_vec_idx(x, y)] = TileType::Floor;
    }
  }
}

fn apply_horizontal_tunnel(map: &mut Map, x1:i32, x2:i32, static_y:i32) {
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = map_2d_to_vec_idx(x, static_y);
        if idx > 0 && idx < 80*50 {
            map.0[idx as usize] = TileType::Floor;
        }
    }
}


fn apply_vertical_tunnel(map: &mut Map, y1:i32, y2:i32, static_x:i32) {
    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = map_2d_to_vec_idx(static_x, y);
        if idx > 0 && idx < 80*50 {
            map.0[idx as usize] = TileType::Floor;
        }
    }
}
