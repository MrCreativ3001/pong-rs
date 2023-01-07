use graphics::{rectangle, Context};
use opengl_graphics::GlGraphics;
use std::ops::Range;

use crate::{BALL_COLOR, BALL_SIZE};
use piston::UpdateArgs;

pub struct Ball {
    pub x: f32,
    pub x_velocity: f32,
    pub y: f32,
    pub y_velocity: f32,
}

impl Ball {
    /// Moves the ball with the velocity and bounces the ball of the "walls" specified using the y_range if they hit a wall.
    pub fn update(&mut self, update_args: &UpdateArgs, mut y_range: Range<f32>) {
        // Move the ball the amount of velocity multiplied by the delta time.
        self.x += self.x_velocity * (update_args.dt as f32);
        self.y += self.y_velocity * (update_args.dt as f32);

        // Subtract the ball height from the range so that the ball won't go off the screen
        y_range.end -= BALL_SIZE.1 as f32;

        if self.y < y_range.start {
            // If the y position is smaller
            // Invert the velocity so that the ball will go in the other direction
            self.y_velocity *= -1.0;
            // Calculate the distance the ball is under the range and set its position to that distances
            self.y = (self.y - y_range.start) * -1.0;
        } else if self.y > y_range.end {
            // If the y position is bigger than the upper range
            // Invert the velocity so that the ball will go in the other direction
            self.y_velocity *= -1.0;
            // Calculate the distance the ball is over the range and set its position to that distances.
            self.y = y_range.end - ((self.y - y_range.end) * -1.0);
        }
    }

    /// Renders the ball.
    pub fn render(&self, context: &Context, gl: &mut GlGraphics) {
        let rect = [
            self.x as f64,
            self.y as f64,
            BALL_SIZE.0 as f64,
            BALL_SIZE.1 as f64,
        ];
        rectangle(BALL_COLOR, rect, context.transform, gl);
    }
}
