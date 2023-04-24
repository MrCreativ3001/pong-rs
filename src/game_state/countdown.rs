use crate::game_state::{GameImpl, GameOptions, GameStateTrait, GraphicsOptions};
use crate::{GameState, SCORE_COUNTDOWN_COLOR, SCORE_COUNTDOWN_SIZE};
use graphics::Transformed;
use piston::{Button, RenderArgs, UpdateArgs};
use std::time::Duration;

pub struct CountdownState {
    duration: Duration,
    next: Option<GameState>,
}
impl CountdownState {
    pub fn new(duration: Duration, next: GameState) -> Self {
        Self {
            duration,
            next: Some(next),
        }
    }
}
impl<Impl: GameImpl> GameStateTrait<Impl> for CountdownState {
    fn update(mut self, args: &UpdateArgs, _: &mut GameOptions<Impl>) -> Result<Self, GameState> {
        self.duration = self
            .duration
            .saturating_sub(Duration::from_secs_f64(args.dt));

        if self.duration.is_zero() {
            Err(self
                .next
                .take()
                .expect("Unable to find the next state after the countdown has ended!"))
        } else {
            Ok(self)
        }
    }

    fn render(
        &mut self,
        ctx: &mut GraphicsOptions<Impl::GraphicsImpl>,
        args: &RenderArgs,
        _: &mut GameOptions<Impl>,
    ) {
        let secs = self.duration.as_secs();
        let countdown = format!("{}", secs);

        let transform = ctx
            .ctx
            .transform
            .trans(args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        graphics::text(
            SCORE_COUNTDOWN_COLOR,
            SCORE_COUNTDOWN_SIZE,
            &countdown,
            ctx.character_cache,
            transform,
            ctx.graphics,
        )
        .expect("Unable to draw text!");
    }

    fn button_press(&mut self, _: &Button, _: &GameOptions<Impl>) {}

    fn button_release(&mut self, _: &Button, _: &GameOptions<Impl>) {}
}
