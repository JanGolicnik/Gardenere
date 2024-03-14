pub mod clickableobject;
mod constants;
mod main_plant;
mod plant;
mod player;
mod polygon;
pub mod post_processing;
mod scenes;
mod sounds;
pub mod sprite_renderer;

use std::collections::HashMap;

use jandering_engine::engine::EngineContext;
use jandering_engine::object::D2Instance;
use jandering_engine::types::{Vec2, Vec3};
use jandering_engine::{
    bind_group::camera::d2::D2CameraBindGroup, engine::Engine, renderer::BindGroupHandle,
};
use rand::Rng;
use wgpu::Color;
use winit::event::{ElementState, MouseButton, WindowEvent};

use jandering_engine::{engine::EngineDescriptor, renderer::Renderer};

use constants::{RESOLUTION_X, RESOLUTION_Y};

use self::constants::{STARTING_CASH, STARTING_POTS};
use self::main_plant::MainPlant;
use self::player::Player;
use self::post_processing::PostProcessing;
use self::scenes::{ActiveScene, Scenes};
use self::sounds::play_sound;
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
    rng: rand::rngs::ThreadRng,
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
    rng: &'a mut rand::rngs::ThreadRng,
}

impl Game {
    pub async fn new() -> Self {
        let rng = rand::thread_rng();

        let mut engine = Engine::new(EngineDescriptor {
            resolution: (RESOLUTION_X, RESOLUTION_Y),
            clear_color: Vec3::new(0.7, 0.6, 0.5),
            show_cursor: false,
        });

        let popr = PostProcessing::new(&mut engine.renderer).await;

        let camera_bg: BindGroupHandle<D2CameraBindGroup> = engine
            .renderer
            .add_bind_group(D2CameraBindGroup::new(&engine.renderer, false));

        let mut sprite_renderer = SpriteRenderer::new(&mut engine.renderer, camera_bg).await;

        let settings = GameSettings { sound_on: true };

        let (scenes, player, main_plant) = make_everything(&mut sprite_renderer);

        Self {
            engine,
            camera_bg,
            sprite_renderer,
            scenes,
            popr,
            player,
            main_plant,
            settings,
            rng,
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
            mut rng,
            ..
        } = self;

        let mut input = InputInfo {
            left_pressed: false,
            left_released: false,
            right_pressed: false,
            right_released: false,
            mouse_pos: None,
        };

        let mut refresh_scene = false;

        let mut next_note = 1.0;
        let mut next_noise = 0.5;

        engine.run(move |context, renderer: &mut Renderer| {
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
                rng: &mut rng,
            };

            if refresh_scene {
                scenes
                    .get_active_scene()
                    .refresh(&mut data, &mut sprite_renderer);
                refresh_scene = false;
            }

            if next_noise < 0.0 && data.settings.sound_on {
                play_sound(
                    "res/sounds/noise.mp3",
                    (data.popr.distortion as f64 + 1.0)
                        .log(100.0)
                        .clamp(0.0, 1.0),
                );
                next_noise = 0.5;
            }
            next_noise -= context.dt as f32;

            if next_note < 0.0 && data.settings.sound_on {
                if data.rng.gen::<f32>() * 3.0 > data.popr.distortion {
                    play_random_high_note(&mut data);
                } else {
                    play_random_low_note(&mut data);
                }
                next_note = (1.0 + data.rng.gen::<f32>()) * 6.0;
            }
            next_note -= context.dt as f32;

            let scene = scenes.get_active_scene();

            let action = scene.update(context, &mut sprite_renderer, &mut data);

            renderer.clear_texture(context.encoder, data.popr.target_texture, CLEAR_COLOR);
            renderer.set_render_target(data.popr.target_texture);

            scene.render(&mut data, &mut sprite_renderer);

            if data.player.cut_eye {
                sprite_renderer.render(D2Instance::default(), "noeye", 1000);
                data.popr.distortion = data.popr.distortion.max(3.0);
            }

            sprite_renderer.submit(context, renderer);

            data.popr.render_tonemap(renderer, context);

            if let Some(action) = action {
                match action {
                    clickableobject::ObjectAction::Goto(scene) => {
                        if data.settings.sound_on {
                            play_sound("res/sounds/leaf.mp3", 0.2 + data.rng.gen::<f64>() * 0.5);
                        }
                        if matches!(scene, ActiveScene::Title) {
                            let (new_scenes, new_player, new_main_plant) =
                                make_everything(&mut sprite_renderer);
                            scenes = new_scenes;
                            player = new_player;
                            main_plant = new_main_plant;
                        }
                        scenes.set_scene(scene);
                        refresh_scene = true;
                    }
                    clickableobject::ObjectAction::Exit => todo!(),
                    clickableobject::ObjectAction::NewDay => {
                        data.player.coins += 1;
                        data.player.total_coins += 1;
                        scenes.garden.new_day(&mut data, &mut sprite_renderer);
                        (0..1).for_each(|_| data.main_plant.new_day(data.player, data.popr));
                    }
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

fn make_everything(sprite_renderer: &mut SpriteRenderer) -> (Scenes, Player, MainPlant) {
    let scenes = Scenes::new(sprite_renderer);

    let player = Player {
        hp: 100.0,
        coins: STARTING_CASH,
        total_coins: STARTING_CASH,
        owned_seeds: HashMap::new(),
        owned_pots: STARTING_POTS,
        has_axe: false,
        cut_finger: false,
        used_finger: false,
        cut_eye: false,
        used_eye: false,
        has_page: false,
    };

    let main_plant = MainPlant::new(sprite_renderer);

    (scenes, player, main_plant)
}

fn play_random_high_note(data: &mut GameData) {
    let rand = data.rng.gen::<u32>() % 3;
    let sound = match rand {
        0 => "res/sounds/note1.mp3",
        1 => "res/sounds/note2.mp3",
        _ => "res/sounds/note3.mp3",
    };
    play_sound(sound, 0.3);
}

fn play_random_low_note(data: &mut GameData) {
    let rand = data.rng.gen::<u32>() % 3;
    let sound = match rand {
        0 => "res/sounds/note4.mp3",
        1 => "res/sounds/note5.mp3",
        _ => "res/sounds/note6.mp3",
    };
    play_sound(sound, 0.3);
}
