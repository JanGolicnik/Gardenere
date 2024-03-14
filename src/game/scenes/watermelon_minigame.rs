use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};
use crate::game::GameData;
use crate::game::{clickableobject::ObjectAction, sprite_renderer::SpriteRenderer};
use jandering_engine::types::DEG_TO_RAD;
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};
use rand::Rng;

use super::{MinigameFingers, Scene};

struct FallingObject {
    pos: Vec2,
    velocity: Vec2,
    rotation: f32,
    collected: bool,
}

pub struct WatermelonMinigameScene {
    falling_objects: Vec<FallingObject>,
    time: f32,
    fingers: MinigameFingers,
}

impl WatermelonMinigameScene {
    pub fn new(_sprite_renderer: &mut SpriteRenderer) -> Self {
        let fingers = MinigameFingers {
            pos: Vec2::new(0.0, RESOLUTION_Y as f32 * -0.5 * 0.9),
            vel_x: 0.0,
        };
        Self {
            falling_objects: Vec::new(),
            time: 0.0,
            fingers,
        }
    }
}

impl Scene for WatermelonMinigameScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {
        self.falling_objects.clear();
        self.fingers.pos.x = 0.0;
        self.time = 0.0;
        self.falling_objects.push(FallingObject {
            pos: Vec2::new(0.0, RESOLUTION_Y as f32 * 0.5 + 300.0),
            rotation: 0.0,
            velocity: Vec2::new(0.0, RESOLUTION_Y as f32 / -5.0),
            collected: false,
        })
    }
    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        let dt = context.dt as f32;
        self.time += dt;

        self.fingers.update(data, context);

        if self.falling_objects.len() == 1 {
            let object = &mut self.falling_objects[0];
            object.velocity.y -= 500.0 * dt;
            if object.pos.y < RESOLUTION_Y as f32 / -2.0 {
                let pos_y = object.pos.y;
                self.falling_objects.clear();
                (0..30).for_each(|_| {
                    const ANGLE: f32 = 12.5;
                    let random_num = data.rng.gen::<f32>() - 0.5;
                    let angle = (90.0 + random_num * ANGLE) * DEG_TO_RAD;
                    let dir = Vec2::from_angle(angle);
                    let vel =
                        (1.0 - random_num.abs()) * 1000.0 * (1.0 - data.rng.gen::<f32>() * 0.6);
                    let pos = Vec2::new(0.0, pos_y);
                    self.falling_objects.push(FallingObject {
                        pos,
                        rotation: 0.0,
                        velocity: dir * vel,
                        collected: false,
                    })
                })
            }
        } else {
            self.falling_objects.iter_mut().for_each(|e| {
                if e.collected {
                    e.velocity.y += 500.0 * dt;
                } else {
                    if e.velocity.y.is_sign_negative() && self.fingers.collides(data, e.pos) {
                        e.collected = true;
                        data.player.coins += 1;
                        e.velocity.y = 100.0;
                        e.velocity.x = 0.0;
                        data.player.total_coins += 1;
                    }
                    e.velocity.y -= 300.0 * dt;
                }
            });
        }

        if !self
            .falling_objects
            .iter()
            .any(|e| !e.collected && e.pos.y > RESOLUTION_Y as f32 / -2.0 - 100.0)
        {
            return Some(ObjectAction::Goto(super::ActiveScene::Garden));
        }

        self.falling_objects
            .iter_mut()
            .for_each(|e| e.pos += e.velocity * dt);

        None
    }

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                scale: Vec2::new(RESOLUTION_X as f32, RESOLUTION_Y as f32),
                ..Default::default()
            },
            "garden_bg",
            0,
        );

        self.fingers.render(data, sprite_renderer);

        self.falling_objects.iter().for_each(|e| {
            sprite_renderer.render(
                D2Instance {
                    position: e.pos,
                    rotation: e.rotation,
                    ..Default::default()
                },
                if e.collected {
                    "market_coin"
                } else if self.falling_objects.len() == 1 {
                    "garden_falling_watermelon"
                } else {
                    "garden_watermelon_piece"
                },
                2,
            )
        })
    }
}
