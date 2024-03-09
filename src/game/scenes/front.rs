use crate::game::clickableobject::ObjectSprite;
use crate::game::main_plant::MainPlantStage;
use crate::game::GameData;
use crate::{
    clickable,
    game::{
        clickableobject::{ClickableObject, ObjectAction},
        constants::{RESOLUTION_X, RESOLUTION_Y},
        sprite_renderer::SpriteRenderer,
    },
};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::{ActiveScene, Scene};

pub struct FrontScene {
    market: ClickableObject,
    garden: ClickableObject,
    house: ClickableObject,
}

impl FrontScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let market = clickable!(-336.0, 118.0, "front_market", sprite_renderer);
        let garden = clickable!(300.0, 60.0, "front_garden", sprite_renderer);
        let house = clickable!(-60.0, -261.0, "front_house", sprite_renderer);
        Self {
            market,
            garden,
            house,
        }
    }
}

impl Scene for FrontScene {
    fn refresh(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {}
    fn update(
        &mut self,
        context: &mut EngineContext,
        sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        self.market.update(context, data.input);
        self.garden.update(context, data.input);
        self.house.update(context, data.input);

        if matches!(data.main_plant.stage, MainPlantStage::Final) {
            self.garden.swap_textures(
                ObjectSprite::Frame("front_gardenfucked"),
                ObjectSprite::Frame("front_gardenfucked_hovered"),
                sprite_renderer,
            );
            self.garden.position.y = 170.0;
        }

        if self.market.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Market));
        }
        if self.garden.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Garden));
        }
        if self.house.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::House));
        }

        None
    }

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                scale: Vec2::new(RESOLUTION_X as f32, RESOLUTION_Y as f32),
                ..Default::default()
            },
            "front_bg",
            0,
        );

        self.market.render(sprite_renderer);
        self.garden.render(sprite_renderer);
        self.house.render(sprite_renderer);
    }
}
