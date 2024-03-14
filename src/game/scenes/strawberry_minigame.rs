use crate::game::constants::{RESOLUTION_X, RESOLUTION_Y};
use crate::game::GameData;
use crate::game::{clickableobject::ObjectAction, sprite_renderer::SpriteRenderer};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};
use rand::Rng;

use super::{MinigameFingers, Scene};

struct FallingObject {
    pos: Vec2,
    velocity: Vec2,
    rotation: f32,
    collected: bool,
}

pub struct StrawberryMinigameScene {
    falling_objects: Vec<FallingObject>,
    time: f32,
    fingers: MinigameFingers,
}

impl StrawberryMinigameScene {
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

impl Scene for StrawberryMinigameScene {
    fn refresh(&mut self, data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {
        self.falling_objects.clear();
        self.fingers.pos.x = 0.0;
        self.time = 0.0;
        (1..18).for_each(|i| {
            let x = (data.rng.gen::<f32>() - 0.5) * 0.5 * RESOLUTION_X as f32;
            let pos = Vec2::new(x, RESOLUTION_Y as f32 * 0.5 + i as f32 * 300.0);
            self.falling_objects.push(FallingObject {
                pos,
                rotation: 0.0,
                velocity: Vec2::ZERO,
                collected: false,
            })
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

        self.falling_objects.iter_mut().for_each(|e| {
            if e.collected {
                e.velocity.y += 500.0 * dt;
            } else {
                if self.fingers.collides(data, e.pos) {
                    e.collected = true;
                    data.player.coins += 1;
                    e.velocity.y = 100.0;
                    e.velocity.x = 0.0;
                    data.player.total_coins += 1;
                }
                e.velocity.y -= 120.0 * dt;
            }
        });

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
                } else {
                    "garden_falling_strawberry"
                },
                2,
            )
        })
    }
}
