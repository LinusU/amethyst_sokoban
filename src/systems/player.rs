use amethyst::{
    core::{Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};

use crate::sokoban::{Box, Direction, Movable, PlayState, Player};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Box>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        Read<'s, PlayState>,
    );

    fn run(
        &mut self,
        (mut transforms, mut movables, mut sprite_renders, players, boxes, input, time, state): Self::SystemData,
    ) {
        for (player, movable, transform, sprite_render) in
            (&players, &mut movables, &transforms, &mut sprite_renders).join()
        {
            if movable.moving_to.is_some() {
                let base = ((sprite_render.sprite_number as f32) / 4.0) as usize;
                let offset = ((time.frame_number() as f32) / 8.0) as usize;

                sprite_render.sprite_number = (base * 4) + (offset % 4);
                continue;
            } else {
                let base = ((sprite_render.sprite_number as f32) / 4.0) as usize;
                sprite_render.sprite_number = base * 4;
            }

            let get_box_at = |x: usize, y: usize| -> Option<&Box> {
                for (transform, r#box) in (&transforms, &boxes).join() {
                    let cx = (transform.translation().x / 16.0) as usize;
                    let cy = (transform.translation().y / 16.0) as usize;

                    if cx == x && cy == y {
                        return Some(r#box);
                    }
                }

                None
            };

            let level = state.level.as_ref().unwrap();

            let x = (transform.translation().x / 16.0) as usize;
            let y = (transform.translation().y / 16.0) as usize;

            if input.action_is_down("up").unwrap() {
                if !level.is_wall(x, y + 1) {
                    if let Some(r#box) = get_box_at(x, y + 1) {
                        if !level.is_wall(x, y + 2) {
                            if get_box_at(x, y + 2).is_none() {
                                movable.moving_to = Some((x, y + 1, Direction::Up));
                                sprite_render.sprite_number = 8;
                            }
                        }
                    } else {
                        movable.moving_to = Some((x, y + 1, Direction::Up));
                        sprite_render.sprite_number = 8;
                    }
                }
            }

            if input.action_is_down("down").unwrap() {
                if !level.is_wall(x, y - 1) {
                    if let Some(r#box) = get_box_at(x, y - 1) {
                        if !level.is_wall(x, y - 2) {
                            if get_box_at(x, y - 2).is_none() {
                                movable.moving_to = Some((x, y - 1, Direction::Down));
                                sprite_render.sprite_number = 0;
                            }
                        }
                    } else {
                        movable.moving_to = Some((x, y - 1, Direction::Down));
                        sprite_render.sprite_number = 0;
                    }
                }
            }

            if input.action_is_down("left").unwrap() {
                if !level.is_wall(x - 1, y) {
                    if let Some(r#box) = get_box_at(x - 1, y) {
                        if !level.is_wall(x - 2, y) {
                            if get_box_at(x - 2, y).is_none() {
                                movable.moving_to = Some((x - 1, y, Direction::Left));
                                sprite_render.sprite_number = 12;
                            }
                        }
                    } else {
                        movable.moving_to = Some((x - 1, y, Direction::Left));
                        sprite_render.sprite_number = 12;
                    }
                }
            }

            if input.action_is_down("right").unwrap() {
                if !level.is_wall(x + 1, y) {
                    if let Some(r#box) = get_box_at(x + 1, y) {
                        if !level.is_wall(x + 2, y) {
                            if get_box_at(x + 2, y).is_none() {
                                movable.moving_to = Some((x + 1, y, Direction::Right));
                                sprite_render.sprite_number = 4;
                            }
                        }
                    } else {
                        movable.moving_to = Some((x + 1, y, Direction::Right));
                        sprite_render.sprite_number = 4;
                    }
                }
            }
        }
    }
}
