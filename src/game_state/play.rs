use crate::ball::Ball;
use crate::game_state::countdown::CountdownState;
use crate::game_state::{
    GameImpl, GameOptions, GameState, GameStateTrait, GraphicsImpl, GraphicsOptions,
};
use crate::paddle::{Paddle, PaddleInput};
use crate::{
    BALL_MULTIPLIER, BALL_SIZE, BUTTON_PLAYER_1_DOWN, BUTTON_PLAYER_1_UP, BUTTON_PLAYER_2_DOWN,
    BUTTON_PLAYER_2_UP, SCORE_COLOR, SCORE_COUNTDOWN_SIZE, SCORE_SIZE, SCORE_Y_GAP,
};
use crate::{PADDLE_BORDER_GAP, PADDLE_SIZE, START_BALL_VELOCITY, WINDOW_SIZE};
use graphics::Transformed;
use piston::Button::Keyboard;
use piston::{Button, RenderArgs, UpdateArgs};
use rand::Rng;
use std::time::Duration;

pub enum PlayerId {
    One,
    Two,
}

pub struct Player {
    pub paddle: Paddle,
    pub score: u32,
    pub id: PlayerId,
}

pub struct PlayState {
    player_one: Player,
    player_two: Player,
    ball: Ball,
}

impl PlayState {
    pub fn new<Impl: GameImpl>(options: &mut GameOptions<Impl>) -> Self {
        Self {
            player_one: Player {
                paddle: Paddle::new(PADDLE_BORDER_GAP, (WINDOW_SIZE.1 as f64) / 2.0),
                score: 0,
                id: PlayerId::One,
            },
            player_two: Player {
                paddle: Paddle::new(
                    (WINDOW_SIZE.0 as f64) - PADDLE_BORDER_GAP - (PADDLE_SIZE.0 as f64),
                    (WINDOW_SIZE.1 as f64) / 2.0,
                ),
                score: 0,
                id: PlayerId::Two,
            },
            ball: Ball {
                x: (WINDOW_SIZE.0 as f64) / 2.0,
                x_velocity: START_BALL_VELOCITY,
                y: (WINDOW_SIZE.1 as f64) / 2.0,
                y_velocity: options
                    .rng
                    .gen_range(-START_BALL_VELOCITY..START_BALL_VELOCITY),
            },
        }
    }

    fn render_score<GImpl: GraphicsImpl>(
        player: &PlayerId,
        score: u32,
        ctx: &mut GraphicsOptions<GImpl>,
    ) {
        let transform = match player {
            PlayerId::One => ctx
                .ctx
                .transform
                .trans((WINDOW_SIZE.0 as f64) / 4.0, SCORE_Y_GAP),
            PlayerId::Two => ctx
                .ctx
                .transform
                .trans((WINDOW_SIZE.0 as f64) / 4.0 * 3.0, SCORE_Y_GAP),
        };

        let score = format!("{}", score);
        graphics::text(
            SCORE_COLOR,
            SCORE_SIZE,
            &score,
            ctx.character_cache,
            transform,
            ctx.graphics,
        )
        .expect("Unable to draw text!");
    }

    fn check_ball_paddle_collide<Impl: GameImpl>(
        paddle: &Paddle,
        ball: &mut Ball,
        options: &mut GameOptions<Impl>,
    ) {
        if !paddle.is_colliding_with_ball(ball) {
            return;
        }

        // Invert the x velocity of the ball
        ball.x_velocity *= -1.0;
        // Increase x velocity
        ball.x_velocity *= BALL_MULTIPLIER;
        // Calculate a new y velocity
        ball.y_velocity = options
            .rng
            .gen_range(-START_BALL_VELOCITY..START_BALL_VELOCITY);
    }

    fn check_ball_scored<Impl: GameImpl>(
        self,
        options: &mut GameOptions<Impl>,
    ) -> Result<Self, GameState> {
        // See if an someone has scored
        if self.ball.x <= 0.0 - (BALL_SIZE.0 as f64) {
            return Err(self.scored(&PlayerId::Two, options));
        } else if self.ball.x >= (WINDOW_SIZE.0 as f64) {
            return Err(self.scored(&PlayerId::One, options));
        }
        return Ok(self);
    }

    fn scored<Impl: GameImpl>(
        mut self,
        player: &PlayerId,
        options: &mut GameOptions<Impl>,
    ) -> GameState {
        // Reset the ball position
        self.ball.x = (WINDOW_SIZE.0 as f64) / 2.0;
        self.ball.y = (WINDOW_SIZE.1 as f64) / 2.0;
        // Reset ball velocity
        self.ball.x_velocity = START_BALL_VELOCITY;

        // Increase the score of the player
        match player {
            PlayerId::One => self.player_one.score += 1,
            PlayerId::Two => self.player_two.score += 1,
        }

        GameState::Countdown(Box::new(CountdownState::new(
            Duration::from_secs(3),
            GameState::Play(Box::new(self)),
        )))
    }

    pub fn is_box_colliding_with_box(
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
}

impl<Impl: GameImpl> GameStateTrait<Impl> for PlayState {
    fn update(
        mut self,
        args: &UpdateArgs,
        options: &mut GameOptions<Impl>,
    ) -> Result<Self, GameState> {
        let y_range = 0f64..(WINDOW_SIZE.1 as f64);

        self.player_one.paddle.update(args, y_range.clone());
        Self::check_ball_paddle_collide(&self.player_one.paddle, &mut self.ball, options);

        self.player_two.paddle.update(args, y_range.clone());
        Self::check_ball_paddle_collide(&self.player_two.paddle, &mut self.ball, options);

        self.ball.update(args, y_range);

        self.check_ball_scored(options)
    }

    fn render(
        &mut self,
        ctx: &mut GraphicsOptions<Impl::GraphicsImpl>,
        args: &RenderArgs,
        _: &mut GameOptions<Impl>,
    ) {
        // flip the screen vertically because the origin is in the top left corner
        let transform = ctx.ctx.transform;
        ctx.ctx.transform = transform.trans(0.0, args.window_size[1] as f64).flip_v();

        self.player_one.paddle.render(ctx.ctx, ctx.graphics);
        self.player_two.paddle.render(ctx.ctx, ctx.graphics);
        self.ball.render(ctx.ctx, ctx.graphics);

        // unflip the screen
        ctx.ctx.transform = transform;

        // Render score
        Self::render_score(&self.player_one.id, self.player_one.score, ctx);
        Self::render_score(&self.player_two.id, self.player_two.score, ctx);
    }

    fn button_press(&mut self, button: &Button, _: &GameOptions<Impl>) {
        if let Keyboard(BUTTON_PLAYER_1_UP) = button {
            self.player_one.paddle.input_mut().press_up();
        }
        if let Keyboard(BUTTON_PLAYER_1_DOWN) = button {
            self.player_one.paddle.input_mut().press_down();
        }
        if let Keyboard(BUTTON_PLAYER_2_UP) = button {
            self.player_two.paddle.input_mut().press_up();
        }
        if let Keyboard(BUTTON_PLAYER_2_DOWN) = button {
            self.player_two.paddle.input_mut().press_down();
        }
    }

    fn button_release(&mut self, button: &Button, _: &GameOptions<Impl>) {
        if let Keyboard(BUTTON_PLAYER_1_UP) = button {
            self.player_one.paddle.input_mut().release_up();
        }
        if let Keyboard(BUTTON_PLAYER_1_DOWN) = button {
            self.player_one.paddle.input_mut().release_down();
        }
        if let Keyboard(BUTTON_PLAYER_2_UP) = button {
            self.player_two.paddle.input_mut().release_up();
        }
        if let Keyboard(BUTTON_PLAYER_2_DOWN) = button {
            self.player_two.paddle.input_mut().release_down();
        }
    }
}
