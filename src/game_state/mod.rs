use crate::game_state::countdown::CountdownState;
use crate::game_state::play::PlayState;
use graphics::{CharacterCache, Context, Graphics};
use piston::{Button, RenderArgs, UpdateArgs};
use rand::Rng;

pub mod countdown;
pub mod play;

pub enum GameState {
    Invalid(Box<Invalid>),
    Countdown(Box<CountdownState>),
    Play(Box<PlayState>),
}

impl<Impl: GameImpl> GameStateTrait<Impl> for GameState {
    fn update(self, args: &UpdateArgs, options: &mut GameOptions<Impl>) -> Result<Self, GameState> {
        match self {
            GameState::Invalid(mut state) => state
                .update(args, options)
                .map(|state| GameState::Invalid(Box::new(state))),

            GameState::Countdown(mut state) => state
                .update(args, options)
                .map(|state| GameState::Countdown(Box::new(state))),

            GameState::Play(mut state) => state
                .update(args, options)
                .map(|state| GameState::Play(Box::new(state))),
        }
    }

    fn render(
        &mut self,
        ctx: &mut GraphicsOptions<Impl::GraphicsImpl>,
        args: &RenderArgs,
        options: &mut GameOptions<Impl>,
    ) {
        match self {
            GameState::Invalid(state) => state.render(ctx, args, options),
            GameState::Countdown(state) => state.render(ctx, args, options),
            GameState::Play(state) => state.render(ctx, args, options),
        }
    }

    fn button_press(&mut self, button: &Button, options: &GameOptions<Impl>) {
        match self {
            GameState::Invalid(state) => state.button_press(button, options),
            GameState::Countdown(state) => state.button_press(button, options),
            GameState::Play(state) => state.button_press(button, options),
        }
    }

    fn button_release(&mut self, button: &Button, options: &GameOptions<Impl>) {
        match self {
            GameState::Invalid(state) => state.button_release(button, options),
            GameState::Countdown(state) => state.button_release(button, options),
            GameState::Play(state) => state.button_release(button, options),
        }
    }
}

pub trait GameImpl {
    type Rng: Rng;
    type GraphicsImpl: GraphicsImpl;
}
pub struct GameOptions<Impl: GameImpl> {
    pub rng: Impl::Rng,
}

pub trait GraphicsImpl {
    type Graphics: Graphics<
        Texture = <<Self as GraphicsImpl>::CharacterCache as CharacterCache>::Texture,
    >;
    type CharacterCache: CharacterCache;
}
pub struct GraphicsOptions<'a, G: GraphicsImpl> {
    pub graphics: &'a mut G::Graphics,
    pub character_cache: &'a mut G::CharacterCache,
    pub ctx: &'a mut Context,
}

pub trait GameStateTrait<Impl: GameImpl>: Sized {
    /// Updates the game.
    fn update(self, args: &UpdateArgs, options: &mut GameOptions<Impl>) -> Result<Self, GameState>;
    /// Render the game.
    fn render(
        &mut self,
        ctx: &mut GraphicsOptions<Impl::GraphicsImpl>,
        args: &RenderArgs,
        options: &mut GameOptions<Impl>,
    );

    fn button_press(&mut self, button: &Button, options: &GameOptions<Impl>);
    fn button_release(&mut self, button: &Button, options: &GameOptions<Impl>);
}

// invalid state
pub struct Invalid;
impl<Impl: GameImpl> GameStateTrait<Impl> for Invalid {
    fn update(self, args: &UpdateArgs, options: &mut GameOptions<Impl>) -> Result<Self, GameState> {
        unimplemented!("Test State")
    }

    fn render(
        &mut self,
        _: &mut GraphicsOptions<Impl::GraphicsImpl>,
        _: &RenderArgs,
        _: &mut GameOptions<Impl>,
    ) {
        unimplemented!("Test State")
    }

    fn button_press(&mut self, _: &Button, _: &GameOptions<Impl>) {
        unimplemented!("Test State")
    }

    fn button_release(&mut self, _: &Button, _: &GameOptions<Impl>) {
        unimplemented!("Test State")
    }
}
