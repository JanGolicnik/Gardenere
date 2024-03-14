use crate::{
    clickable_nohover,
    game::{
        clickableobject::{ClickableObject, ObjectSprite},
        sprite_renderer::SpriteRenderer,
    },
};
use jandering_engine::{object::D2Instance, types::Vec2};

use super::{player::Player, post_processing::PostProcessing};

const BLOOD_POS: Vec2 = Vec2::new(-90.0, 0.0);

pub struct MainPlant {
    pub growth: u32,
    pub object: ClickableObject,
    pub stage: MainPlantStage,
    pub requires_blood: bool,
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
            growth: 0,
            object,
            stage: MainPlantStage::Planted,
            requires_blood: false,
        }
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

        if self.requires_blood
            && !matches!(self.stage, MainPlantStage::Final | MainPlantStage::Gone)
        {
            sprite_renderer.render(
                D2Instance {
                    position: BLOOD_POS,
                    ..Default::default()
                },
                "mainplant_blood",
                4,
            );
        }
    }

    pub fn new_day(&mut self, player: &mut Player, popr: &mut PostProcessing) {
        self.growth += if self.requires_blood { 100 } else { 1 };
        let next_stage = match self.stage {
            MainPlantStage::Planted => {
                if player.total_coins > 7 || self.growth > 10 {
                    Some(MainPlantStage::Second)
                } else {
                    None
                }
            }
            MainPlantStage::Second => {
                if self.growth > 3 {
                    Some(MainPlantStage::Third)
                } else {
                    None
                }
            }
            MainPlantStage::Third => {
                if self.growth > 3 {
                    Some(MainPlantStage::Blood)
                } else {
                    if self.growth == 2 {
                        self.requires_blood = true;
                    }
                    None
                }
            }
            MainPlantStage::Blood => {
                if self.growth > 2 {
                    popr.distortion += 0.3;
                    Some(MainPlantStage::Scary)
                } else {
                    if self.growth == 1 {
                        self.requires_blood = true;
                    }
                    None
                }
            }
            MainPlantStage::Scary => {
                if self.growth > 2 {
                    popr.distortion += 0.3;
                    Some(MainPlantStage::Overgrown)
                } else {
                    if self.growth == 1 {
                        self.requires_blood = true;
                    }
                    None
                }
            }
            MainPlantStage::Overgrown => {
                popr.distortion += 0.4;
                Some(MainPlantStage::Gone)
            }
            MainPlantStage::Gone | MainPlantStage::Final => {
                popr.distortion += 0.4;
                Some(MainPlantStage::Final)
            }
        };
        if let Some(stage) = next_stage {
            self.growth = 0;
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
        }
    }

    pub fn feed(&mut self, player: &mut Player, popr: &mut PostProcessing) {
        if self.requires_blood {
            if player.cut_finger && !player.used_finger {
                player.used_finger = true;
                popr.distortion += 0.5;
                self.requires_blood = false;
            } else if player.cut_eye && !player.used_eye {
                player.used_eye = true;
                self.requires_blood = false;
                popr.distortion += 0.5;
            }
        }
    }
}
