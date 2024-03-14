use crate::game::clickableobject::ObjectSprite;
use crate::game::constants::{
    AXE_PRICE, FLOWER_PRICE, POT_PRICE, RESOLUTION_Y, STRAWBERRY_PRICE, WATERMELON_PRICE,
};
use crate::game::main_plant::MainPlantStage;
use crate::game::plant::PlantType;
use crate::game::GameData;
use crate::{
    clickable,
    game::{
        clickableobject::{ClickableObject, ObjectAction},
        sprite_renderer::SpriteRenderer,
    },
};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::{ActiveScene, Scene};

#[derive(Clone)]
pub struct SeedPacket {
    object: ClickableObject,
    plant_type: PlantType,
    cost: u32,
    starting_y: f32,
}

pub struct MarketScene {
    front: ClickableObject,

    holy_axe: ClickableObject,
    pot: ClickableObject,
    packets: Vec<SeedPacket>,
    time: f32,
}

impl MarketScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let front = clickable!(-32.0, -256.0, "market_front", sprite_renderer);
        let packets = vec![
            SeedPacket {
                object: clickable!(-100.0, 164.0, "market_seeds_flower", sprite_renderer),
                plant_type: PlantType::Flower,
                cost: FLOWER_PRICE,
                starting_y: 164.0,
            },
            SeedPacket {
                object: clickable!(107.0, 170.0, "market_seeds_strawberry", sprite_renderer),
                plant_type: PlantType::Strawberry,
                cost: STRAWBERRY_PRICE,
                starting_y: 170.0,
            },
            SeedPacket {
                object: clickable!(314.0, 160.0, "market_seeds_watermelon", sprite_renderer),
                plant_type: PlantType::Watermelon,
                cost: WATERMELON_PRICE,
                starting_y: 160.0,
            },
        ];

        let pot = clickable!(-120.0, -130.0, "market_pot", sprite_renderer);
        let holy_axe = clickable!(180.0, -163.0, "market_holyaxe", sprite_renderer);

        Self {
            front,
            packets,
            pot,
            holy_axe,
            time: 0.0,
        }
    }
}

impl Scene for MarketScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {
        self.time = 0.0;
    }

    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        self.time += context.dt as f32;
        self.front.update(context, data);
        if self.front.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Front));
        }
        self.pot.update(context, data);
        if self.pot.is_clicked {
            if data.player.coins >= POT_PRICE {
                data.player.coins -= POT_PRICE;
                data.player.owned_pots += 1;
            }
            data.input.left_pressed = false;
        }

        if matches!(
            data.main_plant.stage,
            MainPlantStage::Scary
                | MainPlantStage::Overgrown
                | MainPlantStage::Gone
                | MainPlantStage::Final
        ) {
            self.holy_axe.update(context, data);
            if self.holy_axe.is_clicked {
                if data.player.coins >= AXE_PRICE {
                    data.player.coins -= AXE_PRICE;
                    data.player.has_axe = true;
                }
                data.input.left_pressed = false;
            }
        }

        self.packets.iter_mut().for_each(|packet| {
            packet.object.update(context, data);

            if packet.object.is_clicked {
                if data.player.coins >= packet.cost {
                    data.player.coins -= packet.cost;
                    if let Some(val) = data.player.owned_seeds.get_mut(&packet.plant_type) {
                        *val += 1;
                    } else {
                        data.player.owned_seeds.insert(packet.plant_type, 1);
                    }
                } else {
                    log::info!("not enough coins");
                }
            }

            let target_height = if packet.object.is_hovered {
                packet.starting_y + 15.0
            } else {
                packet.starting_y
            };
            packet.object.position.y +=
                (target_height - packet.object.position.y) * context.dt as f32 * 10.0;
        });

        None
    }

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(D2Instance::default(), "market_bg", 0);

        (0..data.player.coins).for_each(|i| {
            let index_in_stack = i % 25;
            let stack = i / 25;

            let (x, y) = match stack {
                0 => (505.0, -207.0),
                1 => (525.0, -177.0),
                2.. => (480.0, -157.0),
            };

            let t = (self.time - index_in_stack as f32 * 0.1).clamp(0.0, 1.0);
            let starting_pos = RESOLUTION_Y as f32 * 0.5 + 100.0;
            let target_pos = y + index_in_stack as f32 * 20.0;
            let new_pos = starting_pos + (target_pos - starting_pos) * t.powf(2.5);
            sprite_renderer.render(
                D2Instance {
                    position: Vec2::new(x, new_pos),
                    ..Default::default()
                },
                "market_coin",
                (2 - stack) * 25 + i + 2,
            )
        });

        for packet in self.packets.iter() {
            packet.object.render(sprite_renderer);
        }

        if matches!(
            data.main_plant.stage,
            MainPlantStage::Scary
                | MainPlantStage::Overgrown
                | MainPlantStage::Gone
                | MainPlantStage::Final
        ) {
            if !data.player.has_axe {
                self.holy_axe.render(sprite_renderer);
            }
        } else {
            sprite_renderer.render(
                D2Instance {
                    position: Vec2::new(-393.0, 84.0),
                    ..Default::default()
                },
                "market_shopkeep",
                1,
            );
        }

        self.pot.render(sprite_renderer);
        self.front.render(sprite_renderer);
    }
}
