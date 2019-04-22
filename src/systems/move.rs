use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::sokoban::{Direction, Movable};

pub struct MoveSystem;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut movables, time): Self::SystemData) {
        for (movable, transform) in (&mut movables, &mut transforms).join() {
            if let Some((tx, ty, direction)) = movable.moving_to {
                let (dx, dy) = direction.to_velocity(48.0 * time.delta_seconds());
                let target = (tx as f32 * 16.0, ty as f32 * 16.0);

                transform.translate_x(dx);
                transform.translate_y(dy);

                match direction {
                    Direction::Up => {
                        if transform.translation().y >= target.1 {
                            transform.set_y(target.1);
                            movable.moving_to = None
                        }
                    }
                    Direction::Down => {
                        if transform.translation().y <= target.1 {
                            transform.set_y(target.1);
                            movable.moving_to = None
                        }
                    }
                    Direction::Left => {
                        if transform.translation().x <= target.0 {
                            transform.set_x(target.0);
                            movable.moving_to = None
                        }
                    }
                    Direction::Right => {
                        if transform.translation().x >= target.0 {
                            transform.set_x(target.0);
                            movable.moving_to = None
                        }
                    }
                }
            }
        }
    }
}
