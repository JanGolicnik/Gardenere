pub mod clickableobject;
mod constants;
mod plant;
mod player;
mod polygon;
mod post_processing;
mod scenes;
mod sprite_renderer;

use std::collections::HashMap;

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
}

pub struct InputInfo {
    left_pressed: bool,
    left_released: bool,
    right_pressed: bool,
    right_released: bool,
    mouse_pos: Option<Vec2>,
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
            coins: 3,
            owned_seeds: HashMap::new(),
            owned_pots: 3,
        };

        Self {
            engine,
            camera_bg,
            sprite_renderer,
            scenes,
            popr,
            player,
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

            let mut update_scene = true;

            if sleep_timer > 0.0 {
                sleep_timer -= dt;
                if sleep_timer <= 0.0 {
                    scenes.garden.new_day(&mut sprite_renderer);
                }
                update_scene = false;
                popr.set_factor(renderer, (0.5 - sleep_timer / SLEEP_LENGTH).abs() * 2.0);
            } else {
                popr.set_factor(renderer, 1.0);
            }

            let scene = scenes.get_active_scene();

            let mut action = None;
            if update_scene {
                action = scene.update(context, &mut input, &mut sprite_renderer, &mut player);
            }

            renderer.clear_texture(context.encoder, popr.target_texture, CLEAR_COLOR);
            renderer.set_render_target(popr.target_texture);

            scene.render(&mut player, &mut sprite_renderer);
            sprite_renderer.submit(context, renderer);

            popr.render_tonemap(renderer, context);

            if let Some(action) = action {
                match action {
                    clickableobject::ObjectAction::Goto(scene) => {
                        scenes.set_scene(scene);
                        scenes
                            .get_active_scene()
                            .refresh(&mut player, &mut sprite_renderer);
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
