use crate::{clickable_nohover, game::clickableobject::ObjectSprite};
use jandering_engine::types::Vec2;

use crate::clickable;

use super::{
    clickableobject::ClickableObject,
    constants::{FLOWER_VALUE, STRAWBERRY_VALUE, WATERMELON_VALUE},
    sprite_renderer::SpriteRenderer,
    GameData,
};

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
pub enum PlantType {
    Strawberry,
    Flower,
    Watermelon,
}

#[derive(Clone)]
pub struct Plant {
    pub object: ClickableObject,
    pub plant_type: PlantType,
    pub growth: u32,
    update_sprite: bool,
}

impl Plant {
    pub fn new(plant_type: PlantType, sprite_renderer: &mut SpriteRenderer) -> Self {
        let object = match plant_type {
            PlantType::Strawberry => {
                clickable_nohover!(0.0, 0.0, "plants_strawberry", sprite_renderer)
            }
            PlantType::Flower => clickable_nohover!(0.0, 0.0, "plants_flower", sprite_renderer),
            PlantType::Watermelon => {
                clickable_nohover!(0.0, 0.0, "plants_watermelon1", sprite_renderer)
            }
        };
        Self {
            object,
            plant_type,
            growth: 0,
            update_sprite: false,
        }
    }

    pub fn update(&mut self, sprite_renderer: &mut SpriteRenderer) {
        if self.update_sprite {
            self.object.size = sprite_renderer
                .get_sprite(self.object.get_current_frame())
                .size;
            self.update_sprite = false;
        }
    }

    pub fn grow(&mut self) {
        self.set_growth(self.growth + 1)
    }

    fn set_growth(&mut self, val: u32) {
        self.growth = val;
        let mut hovered = None;
        let tex = match self.plant_type {
            PlantType::Strawberry => match self.growth {
                0 => "plants_strawberry",
                1..=3 => "plants_strawberry1",
                4 => {
                    hovered = Some("plants_strawberry2_hovered");
                    "plants_strawberry2"
                }
                _ => "plants_strawberry3",
            },
            PlantType::Flower => match self.growth {
                0 => "plants_flower",
                1 => {
                    hovered = Some("plants_flower1_hovered");
                    "plants_flower1"
                }
                _ => "plants_flower2",
            },
            PlantType::Watermelon => match self.growth {
                0 => "plants_watermelon1",
                1..=2 => "plants_watermelon2",
                3..=4 => "plants_watermelon3",
                5 => {
                    hovered = Some("plants_watermelon4_hovered");
                    "plants_watermelon4"
                }
                _ => "plants_watermelon5",
            },
        };
        self.object.texture = ObjectSprite::Frame(tex);
        self.object.hovered_texture =
            ObjectSprite::Frame(if let Some(t) = hovered { t } else { tex });
        self.update_sprite = true;
    }

    pub fn value(&self) -> u32 {
        match self.plant_type {
            PlantType::Strawberry => STRAWBERRY_VALUE,
            PlantType::Flower => FLOWER_VALUE,
            PlantType::Watermelon => WATERMELON_VALUE,
        }
    }

    pub fn can_harvest(&self) -> bool {
        match self.plant_type {
            PlantType::Strawberry => self.growth == 4,
            PlantType::Flower => self.growth == 1,
            PlantType::Watermelon => self.growth == 5,
        }
    }

    pub fn harvest(&mut self, data: &mut GameData) -> bool {
        let val = self.value();
        data.player.coins += val;
        data.player.total_coins += val;
        match self.plant_type {
            PlantType::Strawberry => {
                self.growth = 1;
                false
            }
            PlantType::Flower => true,
            PlantType::Watermelon => {
                self.growth = 2;
                false
            }
        }
    }
}

pub fn seed_packet_from_plant(
    plant_type: PlantType,
    sprite_renderer: &mut SpriteRenderer,
) -> ClickableObject {
    match plant_type {
        PlantType::Strawberry => clickable!(0.0, 0.0, "market_seeds_strawberry", sprite_renderer),
        PlantType::Flower => clickable!(0.0, 0.0, "market_seeds_flower", sprite_renderer),
        PlantType::Watermelon => clickable!(0.0, 0.0, "market_seeds_watermelon", sprite_renderer),
    }
}
