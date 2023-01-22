use std::mem::swap;
use std::ops::Range;

use graphics::{rectangle, Context, Graphics};
use piston::UpdateArgs;

use crate::{PADDLE_BORDER_GAP, PADDLE_COLOR, PADDLE_SIZE, PADDLE_SPEED, WINDOW_SIZE};

pub enum Player {
    One,
    Two,
}

pub enum PaddleInput {
    Up,
    Down,
    UpDown,
    None,
}

impl PaddleInput {
    pub fn press_up(&mut self) {
        match self {
            Self::Down => swap(self, &mut PaddleInput::UpDown),
            _ => swap(self, &mut PaddleInput::Up),
        }
    }
    pub fn release_up(&mut self) {
        match self {
            Self::UpDown => swap(self, &mut PaddleInput::Down),
            _ => swap(self, &mut PaddleInput::None),
        }
    }

    pub fn press_down(&mut self) {
        match self {
            Self::Up => swap(self, &mut PaddleInput::UpDown),
            _ => swap(self, &mut PaddleInput::Down),
        }
    }
    pub fn release_down(&mut self) {
        match self {
            Self::UpDown => swap(self, &mut PaddleInput::Up),
            _ => swap(self, &mut PaddleInput::None),
        }
    }
}

pub struct Paddle {
    player: Player,
    y: f32,
    input: PaddleInput,
}

impl Paddle {
    pub fn new(player: Player, y: f32) -> Self {
        Self {
            player,
            y,
            input: PaddleInput::None,
        }
    }

    pub fn calculate_x(&self) -> f32 {
        match self.player() {
            // The first player is on the left side of the screen
            Player::One => PADDLE_BORDER_GAP,
            // The second player is on the right side of the screen
            Player::Two => (WINDOW_SIZE.0 as f32) - PADDLE_BORDER_GAP - (PADDLE_SIZE.0 as f32),
        }
    }

    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn change_y(&mut self, y_difference: f32, y_range: Range<f32>) {
        let mut new_y = self.y() + y_difference;

        if new_y < y_range.start {
            // If the y position is smaller than the lower range
            // Set the y position to the lower range
            new_y = y_range.start;
        } else if new_y > y_range.end {
            // If the y position is bigger than the upper range
            // Set the y position to the upper range
            new_y = y_range.end;
        }

        // Update the y position
        self.set_y(new_y);
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn input_mut(&mut self) -> &mut PaddleInput {
        &mut self.input
    }

    /// Updates the paddle by moving it in the direction the input is.
    pub fn update(&mut self, update_args: &UpdateArgs, mut y_range: Range<f32>) {
        // Subtract the paddle height from the range so that the paddle won't go off the screen
        y_range.end -= PADDLE_SIZE.1 as f32;

        let change = match self.input {
            PaddleInput::Up => PADDLE_SPEED * (update_args.dt as f32),
            PaddleInput::Down => -PADDLE_SPEED * (update_args.dt as f32),
            _ => 0.0,
        };
        self.change_y(change, y_range);
    }

    /// Renders the paddle.
    pub fn render<G>(&self, context: &Context, graphics: &mut G) where G: Graphics {
        let x = self.calculate_x();

        // Create a rectangle using the paddle size
        let rect = [
            x as f64,
            self.y as f64,
            PADDLE_SIZE.0 as f64,
            PADDLE_SIZE.1 as f64,
        ];
        // Render the paddle as a rectangle at the position of the transform
        rectangle(PADDLE_COLOR, rect, context.transform, graphics);
    }
}
