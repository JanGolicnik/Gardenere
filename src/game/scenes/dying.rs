use crate::game::GameData;
use crate::game::{clickableobject::ObjectAction, sprite_renderer::SpriteRenderer};
use jandering_engine::{engine::EngineContext, object::D2Instance};
use rand::Rng;

use super::Scene;

pub struct DyingScene {
    stage: u32,
    next_black_timer: f32,
    black_timer: f32,
    n_fades: u32,
    plants: Option<&'static str>,
}

impl DyingScene {
    pub fn new(_sprite_renderer: &mut SpriteRenderer) -> Self {
        Self {
            stage: 0,
            next_black_timer: 2.0,
            black_timer: 0.0,
            n_fades: 0,
            plants: None,
        }
    }
}

impl Scene for DyingScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {}
    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        let dt = context.dt as f32;

        data.popr.distortion +=
            data.popr.distortion * dt * (data.popr.distortion / 100.0).clamp(0.15, 5.0);

        if self.black_timer > 0.0 {
            self.black_timer -= dt;
            data.popr.darkness = 1.0;
            return None;
        }

        data.popr.darkness = 0.0;
        self.next_black_timer -= dt;
        if self.next_black_timer < 0.0 {
            self.next_black_timer = 2.0;
            self.black_timer = 0.1;
            self.n_fades += 1;
            let random = data.rng.gen::<u32>() % 9;
            self.plants = if random < self.n_fades {
                Some("mainplant_killed_plants2")
            } else if random > self.n_fades + 3 {
                None
            } else {
                Some("mainplant_killed_plants1")
            };
        }

        if self.stage >= 8 || data.popr.distortion > 7587092000.0 {
            return Some(ObjectAction::Goto(super::ActiveScene::Title));
        }

        None
    }

    fn render(&mut self, _data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(D2Instance::default(), "mainplant_killed_bg", 0);

        if let Some(plants) = self.plants {
            sprite_renderer.render(D2Instance::default(), plants, 2);
        }
    }
}
