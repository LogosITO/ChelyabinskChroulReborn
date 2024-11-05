mod game;

pub(crate) use game::engine;
pub(crate) use ggez::{ContextBuilder};
pub(crate) use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = engine::Game::new(&mut ctx);
    // Run!
    event::run(ctx, event_loop, my_game);
}