use crate::{clickable_nohover, game::clickableobject::ObjectSprite};
use jandering_engine::{object::D2Instance, types::Vec2};

use crate::clickable;

use super::{clickableobject::ClickableObject, sprite_renderer::SpriteRenderer};

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
pub enum PlantType {
    Strawberry,
    Flower,
    Watermelon,
}

#[derive(Eq, PartialEq, std::hash::Hash, Clone, Copy)]
pub enum PlantState {
    Growing,
    Harvestable,
    Dead,
}

#[derive(Clone)]
pub struct Plant {
    pub object: ClickableObject,
    pub plant_type: PlantType,
    pub growth: u32,
    pub state: PlantState,
    update_sprite: bool,
    pub watered: bool,
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
            state: PlantState::Growing,
            update_sprite: false,
            watered: false,
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
        if self.watered {
            // match self.plant_type {
            //     PlantType::Strawberry => {
            //         self.set_growth(4);
            //     }
            //     PlantType::Flower => {
            //         self.set_growth(1);
            //     }
            //     PlantType::Watermelon => {
            //         self.set_growth(5);
            //     }
            // }
            self.set_growth(self.growth + 1);
            self.watered = false;
        } else {
            self.die();
        }
    }

    fn set_growth(&mut self, val: u32) {
        self.growth = val;
        let mut hovered = None;
        let tex = match self.plant_type {
            PlantType::Strawberry => match self.growth {
                0 => "plants_strawberry",
                1..=3 => "plants_strawberry1",
                4 => {
                    self.state = PlantState::Harvestable;
                    hovered = Some("plants_strawberry2_hovered");
                    "plants_strawberry2"
                }
                _ => {
                    self.state = PlantState::Dead;
                    "plants_strawberry3"
                }
            },
            PlantType::Flower => match self.growth {
                0 => "plants_flower",
                1 => {
                    self.state = PlantState::Harvestable;
                    hovered = Some("plants_flower1_hovered");
                    "plants_flower1"
                }
                _ => {
                    self.state = PlantState::Dead;
                    "plants_flower2"
                }
            },
            PlantType::Watermelon => match self.growth {
                0 => "plants_watermelon1",
                1..=2 => "plants_watermelon2",
                3..=4 => "plants_watermelon3",
                5 => {
                    self.state = PlantState::Harvestable;
                    hovered = Some("plants_watermelon4_hovered");
                    "plants_watermelon4"
                }
                _ => {
                    self.state = PlantState::Dead;
                    "plants_watermelon5"
                }
            },
        };
        self.object.texture = ObjectSprite::Frame(tex);

        self.object.hovered_texture =
            ObjectSprite::Frame(if let Some(t) = hovered { t } else { tex });
        self.update_sprite = true;
    }

    pub fn harvest(&mut self) -> bool {
        match self.plant_type {
            PlantType::Strawberry => {
                self.state = PlantState::Growing;
                self.set_growth(1);
                false
            }
            PlantType::Flower => {
                if self.growth >= 2 {
                    return true;
                }
                true
            }
            PlantType::Watermelon => {
                self.state = PlantState::Growing;
                self.set_growth(2);
                false
            }
        }
    }

    pub fn die(&mut self) {
        match self.plant_type {
            PlantType::Strawberry => self.set_growth(5),
            PlantType::Flower => self.set_growth(2),
            PlantType::Watermelon => self.set_growth(6),
        }
    }

    pub fn render(&mut self, sprite_renderer: &mut SpriteRenderer) {
        self.object.render(sprite_renderer);
        if let Some(sprite) = match self.state {
            PlantState::Growing => {
                if !self.watered {
                    Some("garden_water")
                } else {
                    None
                }
            }
            PlantState::Harvestable => Some("plants_coins"),
            PlantState::Dead => None,
        } {
            let position = self.object.position + Vec2::new(-30.0, 60.0);
            sprite_renderer.render(
                D2Instance {
                    position,
                    ..Default::default()
                },
                sprite,
                4,
            );
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
