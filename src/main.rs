extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use graphics::color::{BLACK, WHITE};
use graphics::types::{Color, FontSize};
use graphics::{clear};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};

use piston::{
    Button, EventSettings, Events, Key, PressEvent, ReleaseEvent, RenderArgs,
    RenderEvent, UpdateArgs, UpdateEvent, WindowSettings,
};
use rand::rngs::ThreadRng;
use rand::{thread_rng};
use std::mem::swap;

use std::time::Duration;


use crate::game_state::countdown::CountdownState;
use crate::game_state::play::PlayState;
use crate::game_state::{
    GameImpl, GameOptions, GameState, GameStateTrait, GraphicsImpl, GraphicsOptions, Invalid,
};


pub mod ball;
pub mod game_state;
pub mod paddle;

#[cfg(test)]
mod test;

/// Game implementation details
struct DefaultGameImpl;
impl GameImpl for DefaultGameImpl {
    type Rng = ThreadRng;
    type GraphicsImpl = DefaultGraphicsImpl;
}

/// The OpenGL version to use.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
struct DefaultGraphicsImpl;
impl GraphicsImpl for DefaultGraphicsImpl {
    type Graphics = GlGraphics;
    type CharacterCache = GlyphCache<'static>;
}

/// The size of the window.
const WINDOW_SIZE: (u32, u32) = (1000, 500);

/// The background color
const BACKGROUND_COLOR: Color = BLACK;

/// The color of every paddle
const PADDLE_COLOR: Color = WHITE;
/// The gap between the window border with the smallest distance to the player.
/// If it is the first player this is the left border and for the second player the right border.
/// For more information on rendering a paddle look at [`render::render_paddle`]
const PADDLE_BORDER_GAP: f64 = 50.0;
/// The size of the paddle.
/// For more information on rendering a paddle look at [`render::render_paddle`]
const PADDLE_SIZE: (u32, u32) = (20, 50);

/// The speed of the paddle if the up or down button was pressed.
const PADDLE_SPEED: f64 = 225.0;

// Controls
const BUTTON_PLAYER_1_UP: Key = Key::W;
const BUTTON_PLAYER_1_DOWN: Key = Key::S;
const BUTTON_PLAYER_2_UP: Key = Key::Up;
const BUTTON_PLAYER_2_DOWN: Key = Key::Down;

/// The color of the ball
const BALL_COLOR: Color = WHITE;
const BALL_SIZE: (u32, u32) = (10, 10);
/// The starting velocity of the ball for the x axis.
/// The y axis will be random between START_BALL_VELOCITY and -START_BALL_VELOCITY at the start
const START_BALL_VELOCITY: f64 = 200.0;
/// The multiplier of the ball velocity after it has hit a paddle.
const BALL_MULTIPLIER: f64 = 1.1;

/// The font used for the score and the countdown
const FONT: &[u8] = include_bytes!("../roboto-font/Roboto-Regular.ttf");

/// The color for the score of player one and two
const SCORE_COLOR: Color = WHITE;
/// The size of the score for player one and two
const SCORE_SIZE: FontSize = 30;
/// The gap between the top of the window and the score.
/// Note that the text will be rendered from a bottom corner.
const SCORE_Y_GAP: f64 = 50.0;

/// The countdown after someone has scored in seconds.
const SCORE_COUNTDOWN: f64 = 4.0;
/// Color for the countdown after someone has scored.
const SCORE_COUNTDOWN_COLOR: Color = WHITE;
/// Font size for the countdown after someone has scored.
const SCORE_COUNTDOWN_SIZE: FontSize = 30;

struct Game {
    graphics: GlGraphics,
    character_cache: GlyphCache<'static>,

    options: GameOptions<DefaultGameImpl>,
    state: GameState,
}

impl Game {
    fn update(&mut self, args: &UpdateArgs) {
        let mut state = GameState::Invalid(Box::new(Invalid));
        swap(&mut self.state, &mut state);

        match state.update(args, &mut self.options) {
            Ok(state) => self.state = state,
            Err(state) => self.state = state,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        self.graphics.draw(args.viewport(), |mut context, gl| {
            clear(BACKGROUND_COLOR, gl);

            let mut graphic_options = GraphicsOptions {
                graphics: gl,
                character_cache: &mut self.character_cache,
                ctx: &mut context,
            };
            self.state
                .render(&mut graphic_options, args, &mut self.options);
        });
    }

    fn button_press(&mut self, button: &Button) {
        self.state.button_press(button, &self.options);
    }
    fn button_release(&mut self, button: &Button) {
        self.state.button_release(button, &self.options);
    }
}

fn main() {
    let mut window: Window = WindowSettings::new("Pong", WINDOW_SIZE)
        .graphics_api(OPENGL_VERSION)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Unable to create Glutin Window!");

    let mut options: GameOptions<DefaultGameImpl> = GameOptions { rng: thread_rng() };

    let mut game = Game {
        graphics: GlGraphics::new(OPENGL_VERSION),
        character_cache: GlyphCache::from_bytes(FONT, (), TextureSettings::new())
            .expect("Unable to create font!"),
        state: create_start_state(&mut options),
        options,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            game.render(&render_args);
        }

        if let Some(update_args) = event.update_args() {
            game.update(&update_args);
        }

        if let Some(button) = event.press_args() {
            game.button_press(&button);
        }
        if let Some(button) = event.release_args() {
            game.button_release(&button);
        }
    }
}

fn create_start_state(options: &mut GameOptions<DefaultGameImpl>) -> GameState {
    GameState::Countdown(Box::new(CountdownState::new(
        Duration::from_secs(3),
        GameState::Play(Box::new(PlayState::new(options))),
    )))
}
