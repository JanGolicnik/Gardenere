pub mod clickableobject;
mod constants;
mod main_plant;
mod plant;
mod player;
mod polygon;
mod post_processing;
mod scenes;
mod sprite_renderer;

use std::collections::HashMap;
use std::io::stdout;

use jandering_engine::engine::EngineContext;
use jandering_engine::types::{Vec2, Vec3};
use jandering_engine::{
    bind_group::camera::d2::D2CameraBindGroup, engine::Engine, renderer::BindGroupHandle,
};
use wgpu::Color;
use winit::event::{ElementState, MouseButton, WindowEvent};

use jandering_engine::{engine::EngineDescriptor, renderer::Renderer};

use constants::{RESOLUTION_X, RESOLUTION_Y};

use crate::game::constants::SLEEP_LENGTH;

use self::constants::{STARTING_CASH, STARTING_POTS};
use self::main_plant::MainPlant;
use self::player::Player;
use self::post_processing::PostProcessing;
use self::scenes::Scenes;
use self::sprite_renderer::SpriteRenderer;

const CLEAR_COLOR: Color = wgpu::Color {
    r: 0.7,
    g: 0.6,
    b: 0.5,
    a: 1.0,
};

pub struct Game {
    engine: Engine,
    camera_bg: BindGroupHandle<D2CameraBindGroup>,
    sprite_renderer: SpriteRenderer,
    scenes: Scenes,
    popr: PostProcessing,
    player: Player,
    main_plant: MainPlant,
    settings: GameSettings,
}

pub struct InputInfo {
    left_pressed: bool,
    left_released: bool,
    right_pressed: bool,
    right_released: bool,
    mouse_pos: Option<Vec2>,
}

pub struct GameSettings {
    sound_on: bool,
}

pub struct GameData<'a> {
    player: &'a mut Player,
    main_plant: &'a mut MainPlant,
    input: &'a mut InputInfo,
    settings: &'a mut GameSettings,
    popr: &'a mut PostProcessing,
}

impl Game {
    pub async fn new() -> Self {
        let mut engine = Engine::new(EngineDescriptor {
            resolution: (RESOLUTION_X, RESOLUTION_Y),
            clear_color: Vec3::new(0.7, 0.6, 0.5),
            show_cursor: false,
        });

        let camera_bg: BindGroupHandle<D2CameraBindGroup> = engine
            .renderer
            .add_bind_group(D2CameraBindGroup::new(&engine.renderer, false));

        let mut sprite_renderer = SpriteRenderer::new(&mut engine.renderer, camera_bg).await;

        let scenes = Scenes::new(&mut sprite_renderer).await;

        let popr = PostProcessing::new(&mut engine.renderer);

        let player = Player {
            hp: 100.0,
            coins: STARTING_CASH,
            total_coins: 0,
            owned_seeds: HashMap::new(),
            owned_pots: STARTING_POTS,
            has_axe: false,
        };

        let main_plant = MainPlant::new(&mut sprite_renderer);

        let settings = GameSettings { sound_on: true };

        Self {
            engine,
            camera_bg,
            sprite_renderer,
            scenes,
            popr,
            player,
            main_plant,
            settings,
        }
    }

    pub fn run(self) {
        let Self {
            engine,
            camera_bg,
            mut sprite_renderer,
            mut scenes,
            mut popr,
            mut player,
            mut main_plant,
            mut settings,
            ..
        } = self;

        let mut sleep_timer = 0.0;

        let mut input = InputInfo {
            left_pressed: false,
            left_released: false,
            right_pressed: false,
            right_released: false,
            mouse_pos: None,
        };

        engine.run(move |context, renderer: &mut Renderer| {
            let dt = context.dt as f32;

            update_input(context, &mut input);

            for event in context.events {
                if let WindowEvent::CursorMoved { position, .. } = event {
                    let camera_bind_group = renderer.get_bind_group_t(camera_bg).unwrap();
                    input.mouse_pos = Some(
                        camera_bind_group
                            .mouse_to_world(Vec2::new(position.x as f32, position.y as f32)),
                    );
                }
            }

            let mut data = GameData {
                player: &mut player,
                main_plant: &mut main_plant,
                input: &mut input,
                settings: &mut settings,
                popr: &mut popr,
            };

            let mut update_scene = true;

            if sleep_timer > 0.0 {
                sleep_timer -= dt;
                if sleep_timer <= 0.0 {
                    scenes.garden.new_day(&mut data, &mut sprite_renderer);
                    data.main_plant.new_day(data.player);
                    data.popr.darkness = 1.0;
                }
                update_scene = false;
                data.popr.darkness = 1.0 - (0.5 - sleep_timer / SLEEP_LENGTH).abs() * 2.0;
            }

            let scene = scenes.get_active_scene();

            let mut action = None;
            if update_scene {
                action = scene.update(context, &mut sprite_renderer, &mut data);
            }

            renderer.clear_texture(context.encoder, data.popr.target_texture, CLEAR_COLOR);
            renderer.set_render_target(data.popr.target_texture);

            scene.render(&mut data, &mut sprite_renderer);
            sprite_renderer.submit(context, renderer);

            data.popr.render_tonemap(renderer, context);

            if let Some(action) = action {
                match action {
                    clickableobject::ObjectAction::Goto(scene) => {
                        scenes.set_scene(scene);
                        scenes
                            .get_active_scene()
                            .refresh(&mut data, &mut sprite_renderer);
                    }
                    clickableobject::ObjectAction::Exit => todo!(),
                    clickableobject::ObjectAction::Sleep => sleep_timer = SLEEP_LENGTH,
                    _ => {}
                }
            }
        });
    }
}

fn update_input(context: &EngineContext, input: &mut InputInfo) {
    input.left_pressed = false;
    input.left_released = false;
    input.right_pressed = false;
    input.right_released = false;

    context.events.iter().for_each(|e| {
        if let WindowEvent::MouseInput { state, button, .. } = e {
            if matches!(state, ElementState::Pressed) {
                if matches!(button, MouseButton::Left) {
                    input.left_pressed = true;
                } else if matches!(button, MouseButton::Right) {
                    input.right_pressed = true;
                }
            } else if matches!(button, MouseButton::Left) {
                input.left_released = true;
            } else if matches!(button, MouseButton::Right) {
                input.right_released = true;
            }
        }
    });
}
