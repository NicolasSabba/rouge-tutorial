use rltk::{Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;
use rouge_tutorial::render::Renderable;
use rouge_tutorial::components::{Position, Player};
use rouge_tutorial::game_state::State;

fn main() -> rltk::BError {
  // Create render terminal
  use rltk::RltkBuilder;
  let context = RltkBuilder::simple80x50()
    .with_title("Rouge")
    .build()?;

  // Create world
  let mut gs = State {
    ecs: World::new()
  };

  // Register components
  gs.ecs.register::<Position>();
  gs.ecs.register::<Renderable>();
  gs.ecs.register::<Player>();

  // Register entities
  gs.ecs
    .create_entity()
    .with(Position { x: 40, y: 25 })
    .with(
      Renderable::new(
        rltk::to_cp437('@'),
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
      )
    ).with(Player {}
  ).build();

  // Run loop
  rltk::main_loop(context, gs)
}
