use crate::{
    clickable_nohover,
    game::{
        clickableobject::{ClickableObject, ObjectSprite},
        sprite_renderer::SpriteRenderer,
    },
};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::{player::Player, InputInfo};

pub struct MainPlant {
    pub growth: f32,
    pub hunger: f32,
    pub object: ClickableObject,
    pub stage: MainPlantStage,
}

pub enum MainPlantStage {
    Planted,
    Second,
    Third,
    Blood,
    Scary,
    Overgrown,
    Gone,
    Final,
}

impl MainPlant {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let object = clickable_nohover!(0.0, 0.0, "mainplant_growth0", sprite_renderer);
        Self {
            growth: 0.0,
            hunger: 0.0,
            object,
            stage: MainPlantStage::Planted,
        }
    }

    pub fn update(
        &mut self,
        context: &EngineContext,
        input: &mut InputInfo,
        sprite_renderer: &mut SpriteRenderer,
    ) {
        self.object.update(context, input);
    }

    pub fn render(&mut self, sprite_renderer: &mut SpriteRenderer) {
        self.object.render(sprite_renderer);

        sprite_renderer.render(
            D2Instance {
                position: Vec2::new(-10.0, -170.0),
                ..Default::default()
            },
            "garden_pot",
            3,
        );
    }

    pub fn new_day(&mut self, player: &mut Player) {
        self.growth += 1.0 + self.hunger * self.hunger;
        let next_stage = match self.stage {
            MainPlantStage::Planted => {
                if player.total_coins > 7 {
                    Some(MainPlantStage::Second)
                } else {
                    None
                }
            }
            MainPlantStage::Second => {
                if self.growth > 8.0 {
                    Some(MainPlantStage::Third)
                } else {
                    None
                }
            }
            MainPlantStage::Third => {
                if self.growth > 7.0 {
                    Some(MainPlantStage::Blood)
                } else {
                    None
                }
            }
            MainPlantStage::Blood => {
                if self.growth > 6.0 {
                    Some(MainPlantStage::Scary)
                } else {
                    None
                }
            }
            MainPlantStage::Scary => {
                if self.growth > 5.0 {
                    Some(MainPlantStage::Overgrown)
                } else {
                    None
                }
            }
            MainPlantStage::Overgrown => Some(MainPlantStage::Gone),
            MainPlantStage::Gone | MainPlantStage::Final => Some(MainPlantStage::Final),
        };
        if let Some(stage) = next_stage {
            self.growth = 0.0;
            self.hunger = 0.0;
            self.stage = stage;
            let (tex, hovered) = match self.stage {
                MainPlantStage::Planted => ("mainplant_growth0", None),
                MainPlantStage::Second => ("mainplant_growth1", None),
                MainPlantStage::Third => ("mainplant_growth2", None),
                MainPlantStage::Blood => ("mainplant_growth3", None),
                MainPlantStage::Scary => ("mainplant_growth4", None),
                MainPlantStage::Overgrown => ("mainplant_growth5", None),
                MainPlantStage::Gone => ("empty", None),
                MainPlantStage::Final => ("mainplant_growth6", None),
            };
            self.object.texture = ObjectSprite::Frame(tex);
            self.object.hovered_texture = hovered.unwrap_or(self.object.texture.clone());
        } else {
            self.hunger += 0.5 + player.total_coins as f32 / 100.0;
        }
    }
}
