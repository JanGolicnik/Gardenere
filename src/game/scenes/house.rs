use crate::frame;
use crate::game::clickableobject::{ObjectFrame, ObjectSprite};
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

pub struct HouseScene {
    door: ClickableObject,
    bed: ClickableObject,
    guy: ClickableObject,
}

impl HouseScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let door = clickable!(-336.0, 118.0, "house_door", sprite_renderer);
        let bed = clickable!(375.0, -85.0, "house_bed", sprite_renderer);
        let guy = ClickableObject::new(
            Vec2::new(0.0, 0.0),
            ObjectSprite::Frame("house_guy"),
            ObjectSprite::Frames(vec![frame!("house_guy_1", 30), frame!("house_guy_2", 30)]),
            sprite_renderer,
        );

        Self { door, bed, guy }
    }
}

impl Scene for HouseScene {
    fn refresh(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {}

    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        sprite_renderer: &mut SpriteRenderer,
        player: &mut Player,
    ) -> Option<ObjectAction> {
        self.door.update(context, input);
        self.bed.update(context, input);
        if self.door.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Front));
        }
        if self.bed.is_clicked {
            return Some(ObjectAction::Sleep);
        }
        self.guy.update(context, input);
        None
    }

    fn render(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                scale: Vec2::new(RESOLUTION_X as f32, RESOLUTION_Y as f32),
                ..Default::default()
            },
            "house_bg",
            0,
        );

        self.door.render(sprite_renderer);
        self.bed.render(sprite_renderer);
        self.guy.render(sprite_renderer);
    }
}
