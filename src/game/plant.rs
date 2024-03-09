use crate::{clickable_nohover, game::clickableobject::ObjectSprite};
use jandering_engine::types::Vec2;

use crate::clickable;

use super::{clickableobject::ClickableObject, sprite_renderer::SpriteRenderer};

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
pub enum PlantType {
    Strawberry,
    Flower,
}

#[derive(Clone)]
pub struct Plant {
    pub object: ClickableObject,
    pub plant_type: PlantType,
    pub growth: u32,
}

impl Plant {
    pub fn new(plant_type: PlantType, sprite_renderer: &mut SpriteRenderer) -> Self {
        let object = match plant_type {
            PlantType::Strawberry => {
                clickable_nohover!(0.0, 0.0, "plants_strawberry", sprite_renderer)
            }
            PlantType::Flower => clickable_nohover!(0.0, 0.0, "plants_flower", sprite_renderer),
        };
        Self {
            object,
            plant_type,
            growth: 0,
        }
    }

    pub fn grow(&mut self, sprite_renderer: &mut SpriteRenderer) {
        self.growth += 1;
        let mut hovered = None;
        let tex = match self.plant_type {
            PlantType::Strawberry => match self.growth {
                0 => "plants_strawberry",
                1 => "plants_strawberry1",
                2 => {
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
        };
        self.object.texture = ObjectSprite::Frame(tex);
        self.object.hovered_texture =
            ObjectSprite::Frame(if let Some(t) = hovered { t } else { tex });
        self.object.size = sprite_renderer
            .get_sprite(self.object.get_current_frame())
            .size;
    }

    pub fn value(&self) -> u32 {
        match self.plant_type {
            PlantType::Strawberry => 3,
            PlantType::Flower => 2,
        }
    }

    pub fn can_harvest(&self) -> bool {
        match self.plant_type {
            PlantType::Strawberry => self.growth == 2,
            PlantType::Flower => self.growth == 1,
        }
    }
}

pub fn seed_packet_from_plant(
    plant_type: PlantType,
    sprite_renderer: &mut SpriteRenderer,
) -> ClickableObject {
    match plant_type {
        PlantType::Strawberry => clickable!(0.0, 0.0, "market_seeds_plant1", sprite_renderer),
        PlantType::Flower => clickable!(0.0, 0.0, "market_seeds_plant2", sprite_renderer),
    }
}
