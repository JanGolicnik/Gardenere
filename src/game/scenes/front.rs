use crate::game::clickableobject::ObjectSprite;
use crate::game::player::Player;
use crate::game::InputInfo;
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
    fn refresh(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {}
    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        sprite_renderer: &mut SpriteRenderer,
        player: &mut Player,
    ) -> Option<ObjectAction> {
        self.market.update(context, input);
        self.garden.update(context, input);
        self.house.update(context, input);

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

    fn render(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {
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
