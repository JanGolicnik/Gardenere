use crate::game::clickableobject::ObjectSprite;
use crate::game::constants::SLEEP_LENGTH;
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

pub struct HouseScene {
    door: ClickableObject,
    bed: ClickableObject,
    table: ClickableObject,
    sleep_timer: f32,
}

impl HouseScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let door = clickable!(-336.0, 118.0, "house_door", sprite_renderer);
        let bed = clickable!(375.0, -85.0, "house_bed", sprite_renderer);
        let table = clickable!(-422.0, -160.0, "house_table", sprite_renderer);
        Self {
            door,
            bed,
            table,
            sleep_timer: 0.0,
        }
    }
}

impl Scene for HouseScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {}

    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        if self.sleep_timer > 0.0 {
            let over_half = self.sleep_timer > SLEEP_LENGTH * 0.5;
            self.sleep_timer -= context.dt as f32;

            if self.sleep_timer <= 0.0 {
                data.popr.darkness = 0.0;
            } else {
                data.popr.darkness = 1.0 - (0.5 - self.sleep_timer / SLEEP_LENGTH).abs() * 2.0;
                if over_half && self.sleep_timer <= SLEEP_LENGTH * 0.5 {
                    return Some(ObjectAction::NewDay);
                }
            }

            return None;
        }

        self.door.update(context, data);
        self.bed.update(context, data);
        self.table.update(context, data);
        if self.door.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Front));
        }
        if self.table.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Table));
        }
        if self.bed.is_clicked {
            self.sleep_timer = SLEEP_LENGTH;
        }

        None
    }

    fn render(&mut self, _data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
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
        self.table.render(sprite_renderer);
    }
}
