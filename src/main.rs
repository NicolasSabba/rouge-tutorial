use rouge_tutorial::play_state::PlayState;

fn main() -> rltk::BError {
  // Create render terminal
  use rltk::RltkBuilder;
  let context = RltkBuilder::simple80x50()
    .with_title("Rouge")
    .build()?;

  // Create game states
  let gs = PlayState::new();

  // Run loop
  rltk::main_loop(context, gs)
}
