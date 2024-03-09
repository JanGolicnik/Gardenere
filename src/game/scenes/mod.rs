use self::{
    front::FrontScene, garden::GardenScene, house::HouseScene, market::MarketScene,
    title::TitleScene,
};
use jandering_engine::engine::EngineContext;

use super::{clickableobject::ObjectAction, sprite_renderer::SpriteRenderer, GameData};

pub mod front;
pub mod garden;
pub mod house;
pub mod market;
pub mod title;

#[derive(Copy, Clone)]
pub enum ActiveScene {
    House,
    Front,
    Garden,
    Market,
    Title,
}

pub struct Scenes {
    pub front: FrontScene,
    pub house: HouseScene,
    pub garden: GardenScene,
    pub market: MarketScene,
    pub title: TitleScene,

    pub active_scene: ActiveScene,
}

impl Scenes {
    pub async fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let house = HouseScene::new(sprite_renderer);
        let front = FrontScene::new(sprite_renderer);
        let garden = GardenScene::new(sprite_renderer);
        let market = MarketScene::new(sprite_renderer);
        let title = TitleScene::new(sprite_renderer);

        Scenes {
            house,
            front,
            garden,
            market,
            title,
            active_scene: ActiveScene::Title,
        }
    }

    pub fn get_active_scene(&mut self) -> &mut dyn Scene {
        match self.active_scene {
            ActiveScene::House => &mut self.house as &mut dyn Scene,
            ActiveScene::Front => &mut self.front as &mut dyn Scene,
            ActiveScene::Garden => &mut self.garden as &mut dyn Scene,
            ActiveScene::Market => &mut self.market as &mut dyn Scene,
            ActiveScene::Title => &mut self.title as &mut dyn Scene,
        }
    }

    pub fn set_scene(&mut self, scene: ActiveScene) {
        self.active_scene = scene;
    }
}

pub trait Scene {
    fn refresh(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer);
    fn update(
        &mut self,
        context: &mut EngineContext,
        sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction>;

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer);
}
