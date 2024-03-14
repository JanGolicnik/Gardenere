use crate::game::GameData;
use crate::game::{clickableobject::ObjectAction, sprite_renderer::SpriteRenderer};
use jandering_engine::{engine::EngineContext, object::D2Instance};

use super::Scene;

pub struct CuttingScene {
    stage: u32,
    fade_time: f32,
}

impl CuttingScene {
    pub fn new(_sprite_renderer: &mut SpriteRenderer) -> Self {
        Self {
            stage: 0,
            fade_time: -1.0,
        }
    }
}

impl Scene for CuttingScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {}
    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        let dt = context.dt as f32;
        if self.fade_time >= 0.0 {
            data.popr.distortion += dt * 2.0;
            self.fade_time += dt;
            if self.fade_time > 1.0 && self.fade_time - dt < 1.0 {
                self.stage += 1;
            }
            data.popr.darkness = 1.0 - (self.fade_time - 1.0).abs();
            if self.fade_time >= 2.0 {
                self.fade_time = -1.0;
            }
        } else if data.input.left_pressed {
            self.fade_time = 0.0;
        } else if data.popr.darkness > 0.0 {
            data.popr.darkness -= data.popr.darkness * dt;
        }

        if self.stage >= 4 {
            return Some(ObjectAction::Goto(super::ActiveScene::Title));
        }

        None
    }

    fn render(&mut self, _data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        match self.stage {
            0 => {
                sprite_renderer.render(D2Instance::default(), "mainplant_cutting1", 0);
            }
            1 => {
                sprite_renderer.render(D2Instance::default(), "mainplant_cutting2", 0);
            }
            2 => {
                sprite_renderer.render(D2Instance::default(), "mainplant_cutting3", 0);
            }
            3 => {
                sprite_renderer.render(D2Instance::default(), "mainplant_cutting4", 0);
            }
            _ => {}
        }
    }
}
