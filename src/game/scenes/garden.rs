use std::{cmp::Ordering, f32::consts::PI};

use crate::game::{
    clickableobject::ObjectSprite,
    plant::{seed_packet_from_plant, Plant, PlantType},
    polygon::Polygon,
    GameData, InputInfo,
};
use jandering_engine::{
    engine::EngineContext,
    object::D2Instance,
    types::{Vec2, DEG_TO_RAD},
};

use crate::{
    clickable,
    game::{
        clickableobject::{ClickableObject, ObjectAction},
        constants::{RESOLUTION_X, RESOLUTION_Y},
        sprite_renderer::SpriteRenderer,
    },
};

use super::{ActiveScene, Scene};

const CARD_STARTING_Y: f32 = -(RESOLUTION_Y as f32 * 0.5) - 200.0;

const POT_START: Vec2 = Vec2::new(
    -(RESOLUTION_X as f32 * 0.5) - 200.0,
    -(RESOLUTION_Y as f32 * 0.5),
);
const POT_END: Vec2 = Vec2::new(
    -(RESOLUTION_X as f32 * 0.5) + 100.0,
    -(RESOLUTION_Y as f32 * 0.5) + 75.0,
);

#[derive(Clone)]
struct Pot {
    object: ClickableObject,
    plant: Option<Plant>,
}

struct Pots {
    placeable_area: Polygon,
    pots: Vec<Pot>,
    base_pot: Pot,
    held_pot: Option<(usize, Vec2)>,
}

struct Card {
    object: ClickableObject,
    pub plant_type: PlantType,
}

struct Cards {
    cards: Vec<Card>,
    held_card: Option<usize>,
}

pub struct GardenScene {
    front: ClickableObject,
    placeable_pot: ClickableObject,

    cards: Cards,
    pots: Pots,
}

impl GardenScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let front = clickable!(0.0, 214.0, "garden_front", sprite_renderer);
        let placeable_pot = clickable!(POT_START.x, POT_START.y, "garden_pot", sprite_renderer);

        let base_pot = Pot {
            object: clickable!(0.0, 0.0, "garden_pot", sprite_renderer),
            plant: None,
        };

        let placeable_area = Polygon {
            points: vec![
                Vec2 {
                    x: -470.0,
                    y: -336.0,
                },
                Vec2 { x: -410.0, y: 0.0 },
                Vec2 {
                    x: -245.0,
                    y: 126.0,
                },
                Vec2 { x: 245.0, y: 126.0 },
                Vec2 { x: 410.0, y: 0.0 },
                Vec2 {
                    x: 470.0,
                    y: -336.0,
                },
            ],
        };

        let cards = Cards {
            cards: Vec::new(),
            held_card: None,
        };

        let pots = Pots {
            placeable_area,
            pots: Vec::new(),
            base_pot,
            held_pot: None,
        };

        Self {
            front,
            placeable_pot,
            pots,
            cards,
        }
    }

    pub fn new_day(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        self.pots
            .pots
            .iter_mut()
            .for_each(|pot| pot.grow(sprite_renderer));
    }
}

impl Scene for GardenScene {
    fn refresh(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        self.cards.cards.clear();
        for (plant_type, num) in data.player.owned_seeds.iter() {
            let plant_type = *plant_type;
            let mut object = seed_packet_from_plant(plant_type, sprite_renderer);
            object.position.y = CARD_STARTING_Y;
            self.cards.cards.push(Card { object, plant_type });
            self.placeable_pot.position = POT_START;
        }
    }
    fn update(
        &mut self,
        context: &mut EngineContext,
        sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        if data.popr.darkness > 0.0 {
            data.popr.darkness -= context.dt as f32 * 3.0;
            return None;
        }

        data.main_plant.update(context, data.input, sprite_renderer);
        self.front.update(context, data.input);
        if self.front.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Front));
        }

        let was_held = self.placeable_pot.is_held;
        self.placeable_pot.update(context, data.input);
        if self.placeable_pot.is_held {
            self.placeable_pot.position = data.input.mouse_pos.unwrap_or(Vec2::ZERO);
        } else {
            self.placeable_pot.position +=
                (POT_END - self.placeable_pot.position) * context.dt as f32 * 4.0;
        }
        self.placeable_pot.scale = Pots::perspective_factor(self.placeable_pot.position.y);

        self.cards.update(context, data.input, sprite_renderer);
        self.pots.update(
            context,
            data.input,
            self.cards.held_card.is_some(),
            sprite_renderer,
        );

        if let Some(card_index) = self.cards.held_card {
            if data.input.left_pressed {
                let plant_type = self.cards.cards[card_index].plant_type;
                if self.pots.place_plant(plant_type, sprite_renderer) {
                    let n_seeds = data.player.owned_seeds.get_mut(&plant_type).unwrap();
                    if *n_seeds == 1 {
                        data.player.owned_seeds.remove(&plant_type);
                        self.cards.cards.remove(card_index);
                    } else {
                        self.cards.cards[card_index].object.position.y = CARD_STARTING_Y;
                        *n_seeds -= 1;
                    }
                } else if data.input.left_pressed
                    && !self
                        .cards
                        .cards
                        .iter()
                        .enumerate()
                        .any(|(i, e)| i != card_index && e.object.is_clicked)
                {
                    data.input.left_pressed = false;
                }
                self.cards.held_card = None;
            }
        } else {
            self.pots.harvest_plants(data);
            if was_held && data.input.left_released && self.pots.try_placing_pot(data) {
                self.placeable_pot.position = POT_START;
            }
        }

        None
    }

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                scale: Vec2::new(RESOLUTION_X as f32, RESOLUTION_Y as f32),
                ..Default::default()
            },
            "garden_bg",
            0,
        );

        data.main_plant.render(sprite_renderer);
        if data.player.owned_pots > 0 {
            self.placeable_pot.render(sprite_renderer);
        }
        self.pots.render(sprite_renderer);
        self.front.render(sprite_renderer);
        self.cards.render(sprite_renderer);
    }
}

impl Pots {
    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        is_holding_card: bool,
        sprite_renderer: &mut SpriteRenderer,
    ) {
        let mouse_pos = input.mouse_pos.unwrap_or(Vec2::ZERO);

        let mut is_pot_hovered = false;
        self.pots.iter_mut().enumerate().for_each(|(i, pot)| {
            pot.object.update(context, input);

            if pot.object.is_held && self.held_pot.is_none() {
                self.held_pot = Some((i, pot.object.position - mouse_pos));
            }

            if self.held_pot.is_some_and(|(val, _)| val == i) {
                return;
            }
            is_pot_hovered |= pot.object.is_hovered;
        });

        let mouse_in_placeable_area = self.placeable_area.point_inside(mouse_pos);
        if (self.held_pot.is_some()
            && mouse_in_placeable_area
            && !is_pot_hovered
            && input.left_released)
            || is_holding_card
        {
            self.held_pot = None;
        }

        if let Some((i, mouse_offset)) = self.held_pot {
            self.pots.iter_mut().for_each(|pot| {
                pot.object.is_held = false;
                pot.object.is_hovered = false;
            });
            self.pots[i].object.position = mouse_pos + mouse_offset;
            self.pots[i].object.is_held = true;
            self.pots[i].object.is_hovered = true;
        }

        for pot in self.pots.iter_mut() {
            let center = pot.center();
            if let Some(plant) = &mut pot.plant {
                plant.update(sprite_renderer);
                plant.object.scale = pot.object.scale;
                plant.object.position = center;
                plant.object.position.y += plant.object.size.y * 0.5;
                plant.object.update(context, input);
                plant.object.position = center;
                plant.object.position.y += plant.object.size(sprite_renderer).y * 0.5;
            }
        }
    }

    fn render(&mut self, sprite_renderer: &mut SpriteRenderer) {
        self.pots.iter_mut().for_each(|pot| {
            pot.object.scale = Self::perspective_factor(pot.object.position.y);
            pot.object.render(sprite_renderer);

            if let Some(plant) = &mut pot.plant {
                plant.object.render(sprite_renderer);
            }
        });
    }

    fn place_plant(&mut self, plant_type: PlantType, sprite_renderer: &mut SpriteRenderer) -> bool {
        if let Some(pot) = self
            .pots
            .iter_mut()
            .filter(|e| e.plant.is_none() && e.object.is_hovered)
            .min_by(|a, b| {
                if a.object.position.y < b.object.position.y {
                    Ordering::Greater
                } else if a.object.position.y == b.object.position.y {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            })
        {
            pot.plant = Some(Plant::new(plant_type, sprite_renderer));
            true
        } else {
            false
        }
    }

    fn harvest_plants(&mut self, data: &mut GameData) {
        if !data.input.left_pressed {
            return;
        }
        for pot in self.pots.iter_mut() {
            if let Some(plant) = &mut pot.plant {
                if plant.object.is_clicked && plant.can_harvest() {
                    data.input.left_pressed = false;
                    if plant.harvest(data) {
                        pot.plant = None;
                    }
                }
            }
        }
    }

    fn try_placing_pot(&mut self, data: &mut GameData) -> bool {
        if data.player.owned_pots == 0 {
            return false;
        }
        let mouse_pos = data.input.mouse_pos.unwrap_or(Vec2::ZERO);
        let mouse_in_placeable_area = self.placeable_area.point_inside(mouse_pos);

        if mouse_in_placeable_area
            && !self.pots.iter().any(|pot| {
                pot.object.is_hovered || pot.plant.as_ref().is_some_and(|val| val.object.is_hovered)
            })
        {
            let mut pot = self.base_pot.clone();
            pot.object.position = mouse_pos;
            self.pots.push(pot);
            data.player.owned_pots -= 1;
            true
        } else {
            false
        }
    }

    fn perspective_factor(y: f32) -> f32 {
        let distance_factor = (y.clamp(-360.0, 100.0) + 460.0) / 460.0;
        0.7 + (1.0 - distance_factor) * 0.75
    }
}

impl Pot {
    fn center(&self) -> Vec2 {
        self.object.position + Vec2::new(0.0, self.object.size.y * 0.5 * 0.6 * self.object.scale)
    }

    fn grow(&mut self, sprite_renderer: &mut SpriteRenderer) {
        if let Some(plant) = &mut self.plant {
            plant.grow();
        }
    }
}

impl Cards {
    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        sprite_renderer: &mut SpriteRenderer,
    ) {
        let n_cards = self.cards.len();
        let angle = (n_cards as f32 - 1.0) * 90.0;
        let starting_pos = Vec2::new(0.0, RESOLUTION_Y as f32 / -2.0 + 50.0);
        let offset_angle = 90.0 - angle * 0.5;
        for (i, card) in self.cards.iter_mut().enumerate() {
            // cards start at (0.0, -res.y/0.0 - 100)
            // then fan out depending on how many you have
            let ratio = if n_cards == 1 {
                0.0
            } else {
                i as f32 / (n_cards - 1) as f32
            };
            let angle = ratio * angle;
            let angle_rad = (offset_angle + angle) * DEG_TO_RAD;
            let target_pos = starting_pos + Vec2::from_angle(angle_rad) * 60.0;
            card.object.position += (target_pos - card.object.position) * context.dt as f32 * 3.0;
            card.object.rotation =
                (Vec2::new(0.0, CARD_STARTING_Y) - card.object.position).to_angle() + PI * 0.5;
            card.object.update(context, input);

            if card.object.is_clicked && self.held_card.is_none() {
                input.left_pressed = false;
                self.held_card = Some(i);
            }
        }
        if let Some(card_index) = self.held_card {
            let card = &mut self.cards[card_index];
            card.object.position = input.mouse_pos.unwrap_or(Vec2::ZERO);
            card.object.rotation = 0.0;
        }
    }

    fn render(&mut self, sprite_renderer: &mut SpriteRenderer) {
        for card in self.cards.iter() {
            card.object.render(sprite_renderer);
        }
    }
}
