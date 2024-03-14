use self::{
    cutting::CuttingScene, dying::DyingScene, flower_minigame::FlowerMinigameScene,
    front::FrontScene, garden::GardenScene, house::HouseScene, market::MarketScene,
    strawberry_minigame::StrawberryMinigameScene, table::TableScene, title::TitleScene,
    watermelon_minigame::WatermelonMinigameScene,
};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::{
    clickableobject::ObjectAction, constants::SKIP_INTRO, sprite_renderer::SpriteRenderer, GameData,
};

pub mod cutting;
pub mod dying;
pub mod flower_minigame;
pub mod front;
pub mod garden;
pub mod house;
pub mod market;
pub mod strawberry_minigame;
pub mod table;
pub mod title;
pub mod watermelon_minigame;

#[derive(Copy, Clone)]
pub enum ActiveScene {
    House,
    Front,
    Garden,
    Market,
    Title,
    Table,
    Cutting,
    FlowerMinigame,
    StrawberryMinigame,
    WatermelonMinigame,
    Dying,
}

pub struct Scenes {
    pub front: FrontScene,
    pub house: HouseScene,
    pub garden: GardenScene,
    pub market: MarketScene,
    pub title: TitleScene,
    pub table: TableScene,
    pub cutting: CuttingScene,
    pub flower_minigame: FlowerMinigameScene,
    pub strawberry_minigame: StrawberryMinigameScene,
    pub watermelon_minigame: WatermelonMinigameScene,
    pub dying: DyingScene,

    pub active_scene: ActiveScene,
}

impl Scenes {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let house = HouseScene::new(sprite_renderer);
        let front = FrontScene::new(sprite_renderer);
        let garden = GardenScene::new(sprite_renderer);
        let market = MarketScene::new(sprite_renderer);
        let title = TitleScene::new(sprite_renderer);
        let table = TableScene::new(sprite_renderer);
        let cutting = CuttingScene::new(sprite_renderer);
        let flower_minigame = FlowerMinigameScene::new(sprite_renderer);
        let strawberry_minigame = StrawberryMinigameScene::new(sprite_renderer);
        let watermelon_minigame = WatermelonMinigameScene::new(sprite_renderer);
        let dying = DyingScene::new(sprite_renderer);

        Scenes {
            house,
            front,
            garden,
            market,
            title,
            table,
            cutting,
            flower_minigame,
            strawberry_minigame,
            watermelon_minigame,
            dying,
            active_scene: if SKIP_INTRO {
                ActiveScene::Garden
            } else {
                ActiveScene::Title
            },
        }
    }

    pub fn get_active_scene(&mut self) -> &mut dyn Scene {
        match self.active_scene {
            ActiveScene::House => &mut self.house as &mut dyn Scene,
            ActiveScene::Front => &mut self.front as &mut dyn Scene,
            ActiveScene::Garden => &mut self.garden as &mut dyn Scene,
            ActiveScene::Market => &mut self.market as &mut dyn Scene,
            ActiveScene::Title => &mut self.title as &mut dyn Scene,
            ActiveScene::Table => &mut self.table as &mut dyn Scene,
            ActiveScene::Cutting => &mut self.cutting as &mut dyn Scene,
            ActiveScene::FlowerMinigame => &mut self.flower_minigame as &mut dyn Scene,
            ActiveScene::StrawberryMinigame => &mut self.strawberry_minigame as &mut dyn Scene,
            ActiveScene::WatermelonMinigame => &mut self.watermelon_minigame as &mut dyn Scene,
            ActiveScene::Dying => &mut self.dying as &mut dyn Scene,
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

pub struct MinigameFingers {
    pub pos: Vec2,
    pub vel_x: f32,
}

impl MinigameFingers {
    pub fn update(&mut self, data: &mut GameData, context: &EngineContext) {
        self.pos.x += (data.input.mouse_pos.unwrap_or(Vec2::ZERO).x - self.pos.x)
            * context.dt as f32
            * if data.player.cut_finger { 0.75 } else { 1.0 };
    }

    pub fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                position: self.pos,
                ..Default::default()
            },
            if data.player.cut_finger {
                "garden_fingersmissing"
            } else {
                "garden_fingers"
            },
            10,
        )
    }

    pub fn collides(&self, data: &mut GameData, pos: Vec2) -> bool {
        self.pos.distance(pos) < if data.player.cut_finger { 40.0 } else { 75.0 }
    }
}
