use crate::game::clickableobject::ObjectSprite;
use crate::game::constants::RESOLUTION_Y;
use crate::game::plant::PlantType;
use crate::game::player::Player;
use crate::game::InputInfo;
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

    pot: ClickableObject,
    packets: Vec<SeedPacket>,
    time: f32,
}

impl MarketScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let front = clickable!(-32.0, -256.0, "market_front", sprite_renderer);
        let packets = vec![
            SeedPacket {
                object: clickable!(-37.0, 164.0, "market_seeds_plant1", sprite_renderer),
                plant_type: PlantType::Strawberry,
                cost: 2,
                starting_y: 164.0,
            },
            SeedPacket {
                object: clickable!(170.0, 170.0, "market_seeds_plant2", sprite_renderer),
                plant_type: PlantType::Flower,
                cost: 3,
                starting_y: 170.0,
            },
        ];

        let pot = clickable!(-70.0, -130.0, "market_pot", sprite_renderer);

        Self {
            front,
            packets,
            pot,
            time: 0.0,
        }
    }
}

impl Scene for MarketScene {
    fn refresh(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {
        self.time = 0.0;
    }

    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        _sprite_renderer: &mut SpriteRenderer,
        player: &mut Player,
    ) -> Option<ObjectAction> {
        self.time += context.dt as f32;
        self.front.update(context, input);
        if self.front.is_clicked {
            return Some(ObjectAction::Goto(ActiveScene::Front));
        }
        self.pot.update(context, input);
        if self.pot.is_clicked {
            if player.coins >= 1 {
                player.coins -= 1;
                player.owned_pots += 1;
            } else {
                log::info!("not enough coins");
            }
            input.left_pressed = false;
        }

        self.packets.iter_mut().for_each(|packet| {
            packet.object.update(context, input);

            if packet.object.is_clicked {
                if player.coins >= packet.cost {
                    player.coins -= packet.cost;
                    if let Some(val) = player.owned_seeds.get_mut(&packet.plant_type) {
                        *val += 1;
                    } else {
                        player.owned_seeds.insert(packet.plant_type, 1);
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

    fn render(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(D2Instance::default(), "market_bg", 0);
        (0..player.coins).for_each(|i| {
            let t = (self.time - i as f32 * 0.2).clamp(0.0, 1.0);
            let starting_pos = RESOLUTION_Y as f32 * 0.5 + 100.0;
            let target_pos = -177.0 + i as f32 * 23.0;
            let new_pos = starting_pos + (target_pos - starting_pos) * t.powf(2.5);
            sprite_renderer.render(
                D2Instance {
                    position: Vec2::new(505.0, new_pos),
                    ..Default::default()
                },
                "market_coin",
                i + 2,
            )
        });

        for packet in self.packets.iter() {
            packet.object.render(sprite_renderer);
        }
        self.pot.render(sprite_renderer);
        self.front.render(sprite_renderer);
    }
}
