extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::color::{BLACK, BLUE, GREEN, RED};
use graphics::types::{Color, FontSize};
use graphics::{clear, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::Button::Keyboard;
use piston::{
    Button, EventSettings, Events, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent,
    UpdateArgs, UpdateEvent, WindowSettings,
};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use crate::ball::Ball;
use crate::paddle::{Paddle, Player};

mod ball;
mod paddle;

#[cfg(test)]
mod test;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// The size of the window.
const WINDOW_SIZE: (u32, u32) = (1000, 500);

/// The background color
const BACKGROUND_COLOR: Color = BLACK;

/// The color of every paddle
const PADDLE_COLOR: Color = RED;
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
const BALL_COLOR: Color = BLUE;
const BALL_SIZE: (u32, u32) = (10, 10);
/// The starting velocity of the ball for the x axis.
/// The y axis will be random between START_BALL_VELOCITY and -START_BALL_VELOCITY at the start
const START_BALL_VELOCITY: f64 = 200.0;
/// The multiplier of the ball velocity after it has hit a paddle.
const BALL_MULTIPLIER: f64 = 1.1;

/// The font used for the score and the countdown
const FONT: &[u8] = include_bytes!("../roboto-font/Roboto-Regular.ttf");

/// The color for the score of player one and two
const SCORE_COLOR: Color = GREEN;
/// The size of the score for player one and two
const SCORE_SIZE: FontSize = 30;
/// The gap between the top of the window and the score.
/// Note that the text will be rendered from a bottom corner.
const SCORE_Y_GAP: f64 = 50.0;

/// The countdown after someone has scored in seconds.
const SCORE_COUNTDOWN: f64 = 4.0;
/// Color for the countdown after someone has scored.
const SCORE_COUNTDOWN_COLOR: Color = GREEN;
/// Font size for the countdown after someone has scored.
const SCORE_COUNTDOWN_SIZE: FontSize = 30;

struct Game {
    // OpenGL Backend
    gl: GlGraphics,
    glyph_cache: GlyphCache<'static>,
    // Game State
    rng: ThreadRng,
    state: GameState,
    player_1: Paddle,
    player_1_score: u32,
    player_2: Paddle,
    player_2_score: u32,
    ball: Ball,
}

impl Game {
    fn button_press(&mut self, button: &Button) {
        if let GameState::Pause = &self.state {
            self.state = GameState::Play;
        }

        if let Keyboard(BUTTON_PLAYER_1_UP) = button {
            self.player_1.input_mut().press_up();
        }
        if let Keyboard(BUTTON_PLAYER_1_DOWN) = button {
            self.player_1.input_mut().press_down();
        }
        if let Keyboard(BUTTON_PLAYER_2_UP) = button {
            self.player_2.input_mut().press_up();
        }
        if let Keyboard(BUTTON_PLAYER_2_DOWN) = button {
            self.player_2.input_mut().press_down();
        }
    }

    fn button_release(&mut self, button: &Button) {
        if let Keyboard(BUTTON_PLAYER_1_UP) = button {
            self.player_1.input_mut().release_up();
        }
        if let Keyboard(BUTTON_PLAYER_1_DOWN) = button {
            self.player_1.input_mut().release_down();
        }
        if let Keyboard(BUTTON_PLAYER_2_UP) = button {
            self.player_2.input_mut().release_up();
        }
        if let Keyboard(BUTTON_PLAYER_2_DOWN) = button {
            self.player_2.input_mut().release_down();
        }
    }

    /// Updates the ball
    fn update(&mut self, update_args: &UpdateArgs) {
        if let GameState::Pause = &self.state {
            return;
        } else if let GameState::Scored { timer } = &mut self.state {
            *timer -= update_args.dt;
            if *timer <= 0.0 {
                self.state = GameState::Play;
            }
            return;
        }

        // Update all objects
        let y_range = 0.0..(WINDOW_SIZE.1 as f64);
        self.player_1.update(update_args, y_range.clone());
        self.player_2.update(update_args, y_range.clone());
        self.ball.update(update_args, y_range);

        // See what paddle the ball is bouncing against
        if self.ball.x_velocity > 0.0 {
            // The ball is going towards the right paddle (player two)
            if Self::is_paddle_colliding_with_ball(&self.player_2, &self.ball) {
                self.ball_hit_paddle();
            }
        } else {
            // The ball is going towards the left paddle (player one)
            if Self::is_paddle_colliding_with_ball(&self.player_1, &self.ball) {
                self.ball_hit_paddle();
            }
        }

        // See if an someone has scored
        if self.ball.x <= 0.0 - (BALL_SIZE.0 as f64) {
            self.scored(&Player::Two);
        } else if self.ball.x >= (WINDOW_SIZE.0 as f64) {
            self.scored(&Player::One);
        }
    }

    fn scored(&mut self, player: &Player) {
        match player {
            Player::One => self.player_1_score += 1,
            Player::Two => self.player_2_score += 1,
        }
        self.state = GameState::Scored {
            timer: SCORE_COUNTDOWN,
        };
        // reset ball position
        self.ball.x = (WINDOW_SIZE.0 as f64) / 2.0;
        self.ball.y = (WINDOW_SIZE.1 as f64) / 2.0;
        // reset ball velocity
        self.ball.x_velocity = START_BALL_VELOCITY;
    }

    fn ball_hit_paddle(&mut self) {
        // Invert the x velocity of the ball
        self.ball.x_velocity *= -1.0;
        // Increase x velocity
        self.ball.x_velocity *= BALL_MULTIPLIER;
        // Calculate a new y velocity
        self.ball.y_velocity = self
            .rng
            .gen_range(-START_BALL_VELOCITY..START_BALL_VELOCITY);
    }

    fn is_paddle_colliding_with_ball(paddle: &Paddle, ball: &Ball) -> bool {
        return Self::is_box_colliding_with_box(
            paddle.x(),
            paddle.y(),
            PADDLE_SIZE.0 as f64,
            PADDLE_SIZE.1 as f64,
            ball.x,
            ball.y,
            BALL_SIZE.0 as f64,
            BALL_SIZE.1 as f64,
        );
    }
    fn is_box_colliding_with_box(
        b1x: f64,
        b1y: f64,
        b1w: f64,
        b1h: f64,
        b2x: f64,
        b2y: f64,
        b2w: f64,
        b2h: f64,
    ) -> bool {
        return b1x <= b2x + b2w && b1x + b1w >= b2x && b1y <= b2y + b2h && b1y + b1h >= b2y;
    }

    /// Renders the paddle for player one and two and a ball.
    /// If the state is Scored a timer with a countdown will be displayed.
    fn render(&mut self, render_args: &RenderArgs) {
        self.gl.draw(render_args.viewport(), |mut context, gl| {
            // Clear screen
            clear(BACKGROUND_COLOR, gl);

            // Because rendering is easier if 0,0 is at the bottom left we'll have to modify the transform
            context.transform = context
                .transform
                .flip_v()
                .trans(0.0, -(WINDOW_SIZE.1 as f64));

            // Render paddles
            self.player_1.render(&context, gl);
            self.player_2.render(&context, gl);
            // Render ball
            self.ball.render(&context, gl);

            // Render score
            let score_base_transform = context
                .transform
                .trans(0.0, (WINDOW_SIZE.1 as f64) - SCORE_Y_GAP)
                .flip_v();
            // Player one
            let transform = score_base_transform.trans((WINDOW_SIZE.0 as f64) / 4.0, 0.0);
            let score = format!("{}", self.player_1_score);
            graphics::text(
                SCORE_COLOR,
                SCORE_COUNTDOWN_SIZE,
                &score,
                &mut self.glyph_cache,
                transform,
                gl,
            )
            .expect("Unable to draw text!");
            // Player two
            let transform = score_base_transform.trans((WINDOW_SIZE.0 as f64) / 4.0 * 3.0, 0.0);
            let score = format!("{}", self.player_2_score);
            graphics::text(
                SCORE_COLOR,
                SCORE_SIZE,
                &score,
                &mut self.glyph_cache,
                transform,
                gl,
            )
            .expect("Unable to draw text!");

            // Render countdown if needed
            if let GameState::Scored { timer } = &self.state {
                let countdown = format!("{}", timer.floor());
                let transform = context
                    .transform
                    .trans(
                        ((WINDOW_SIZE.0 as f64) / 2.0) - ((BALL_SIZE.0 as f64) / 2.0),
                        ((WINDOW_SIZE.1 as f64) / 2.0) + ((BALL_SIZE.1 as f64) * 2.0),
                    )
                    .flip_v();
                graphics::text(
                    SCORE_COUNTDOWN_COLOR,
                    SCORE_COUNTDOWN_SIZE,
                    &countdown,
                    &mut self.glyph_cache,
                    transform,
                    gl,
                )
                .expect("Unable to draw text!");
            }
        })
    }
}

enum GameState {
    /// The starting game state. It waits for any input and moves on to the play state.
    Pause,
    /// This game state happens after someone has scored.
    /// After the timer hits 0 it'll switch to the play game state.
    Scored { timer: f64 },
    /// In this game state the paddles can be moved an the ball is moving.
    Play,
}

fn main() {
    let mut window: Window = WindowSettings::new("Pong", WINDOW_SIZE)
        .graphics_api(OPENGL_VERSION)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Unable to create Glutin Window!");

    let mut rng = thread_rng();

    let mut game = Game {
        gl: GlGraphics::new(OPENGL_VERSION),
        glyph_cache: GlyphCache::from_bytes(FONT, (), TextureSettings::new())
            .expect("Unable to create font!"),

        state: GameState::Pause,

        player_1: Paddle::new(PADDLE_BORDER_GAP, (WINDOW_SIZE.1 as f64) / 2.0),
        player_1_score: 0,
        player_2: Paddle::new((WINDOW_SIZE.0 as f64) - PADDLE_BORDER_GAP - (PADDLE_SIZE.0 as f64), (WINDOW_SIZE.1 as f64) / 2.0),
        player_2_score: 0,

        ball: Ball {
            x: (WINDOW_SIZE.0 as f64) / 2.0,
            x_velocity: START_BALL_VELOCITY,
            y: (WINDOW_SIZE.1 as f64) / 2.0,
            y_velocity: rng.gen_range(-START_BALL_VELOCITY..START_BALL_VELOCITY),
        },
        rng,
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

// pub fn calculate_x(&self) -> f32 {
//     match self.player() {
//         // The first player is on the left side of the screen
//         Player::One => PADDLE_BORDER_GAP,
//         // The second player is on the right side of the screen
//         Player::Two => (WINDOW_SIZE.0 as f32) - PADDLE_BORDER_GAP - (PADDLE_SIZE.0 as f32),
//     }
// }