use std::thread::AccessError;

use self::{front::FrontScene, garden::GardenScene, house::HouseScene, market::MarketScene};
use jandering_engine::{engine::EngineContext, types::Vec2};

use super::{
    clickableobject::ObjectAction, player::Player, sprite_renderer::SpriteRenderer, InputInfo,
};

pub mod front;
pub mod garden;
pub mod house;
pub mod market;

#[derive(Copy, Clone)]
pub enum ActiveScene {
    House,
    Front,
    Garden,
    Market,
}

pub struct Scenes {
    pub front: FrontScene,
    pub house: HouseScene,
    pub garden: GardenScene,
    pub market: MarketScene,

    pub active_scene: ActiveScene,
}

impl Scenes {
    pub async fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let house = HouseScene::new(sprite_renderer);
        let front = FrontScene::new(sprite_renderer);
        let garden = GardenScene::new(sprite_renderer);
        let market = MarketScene::new(sprite_renderer);

        Scenes {
            house,
            front,
            garden,
            market,
            active_scene: ActiveScene::House,
        }
    }

    pub fn get_active_scene(&mut self) -> &mut dyn Scene {
        match self.active_scene {
            ActiveScene::House => &mut self.house as &mut dyn Scene,
            ActiveScene::Front => &mut self.front as &mut dyn Scene,
            ActiveScene::Garden => &mut self.garden as &mut dyn Scene,
            ActiveScene::Market => &mut self.market as &mut dyn Scene,
        }
    }

    pub fn set_scene(&mut self, scene: ActiveScene) {
        self.active_scene = scene;
    }
}

pub trait Scene {
    fn refresh(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer);
    fn update(
        &mut self,
        context: &mut EngineContext,
        input: &mut InputInfo,
        sprite_renderer: &mut SpriteRenderer,
        player: &mut Player,
    ) -> Option<ObjectAction>;

    fn render(&mut self, player: &mut Player, sprite_renderer: &mut SpriteRenderer);
}
