use crate::game::clickableobject::ClickableObject;
use crate::game::constants::RESOLUTION_Y;
use crate::game::GameData;
use crate::game::{
    clickableobject::{ObjectAction, ObjectSprite},
    sprite_renderer::SpriteRenderer,
};
use crate::{clickable, clickable_nohover};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::Scene;

enum State {
    Idle,
    PlayAnim { play_anim_timer: f32, vel_y: f32 },
    Intro(f32),
}

pub struct TitleScene {
    play_btn: ClickableObject,
    sound_toggle: ClickableObject,
    state: State,
    bg_y: f32,
}

impl TitleScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let play_btn = clickable!(0.0, 0.0, "title_play", sprite_renderer);
        let sound_toggle = clickable_nohover!(0.0, -0.0, "title_sound_on", sprite_renderer);
        Self {
            play_btn,
            sound_toggle,
            state: State::Intro(-1.0),
            bg_y: 0.0,
        }
    }
}

impl Scene for TitleScene {
    fn refresh(&mut self, _data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {}
    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        let dt = context.dt as f32;

        if data.popr.distortion > 0.0 {
            data.popr.distortion -= data.popr.distortion * dt * 3.0;
            data.popr.time = 0.0
        }
        match &mut self.state {
            State::Idle => {
                self.play_btn.update(context, data);

                if self.play_btn.is_clicked {
                    self.state = State::PlayAnim {
                        play_anim_timer: 0.0,
                        vel_y: -300.0,
                    };
                }

                self.sound_toggle.update(context, data);

                if self.sound_toggle.is_clicked {
                    data.settings.sound_on = !data.settings.sound_on;
                }

                if data.settings.sound_on {
                    self.sound_toggle.texture = ObjectSprite::Frame("title_sound_on");
                    self.sound_toggle.hovered_texture = ObjectSprite::Frame("title_sound_on");
                } else {
                    self.sound_toggle.texture = ObjectSprite::Frame("title_sound_off");
                    self.sound_toggle.hovered_texture = ObjectSprite::Frame("title_sound_off");
                }
            }
            State::PlayAnim {
                play_anim_timer,
                vel_y,
            } => {
                if *play_anim_timer < 2.0 {
                    data.popr.darkness = *play_anim_timer / 2.0;
                    *vel_y -= dt * 600.0;
                } else if *play_anim_timer < 2.5 {
                    data.popr.darkness = 1.0;
                } else {
                    return Some(ObjectAction::Goto(super::ActiveScene::Garden));
                }
                *play_anim_timer += dt;

                self.bg_y += *vel_y * dt;
            }
            State::Intro(time) => {
                if data.popr.darkness > 0.0 {
                    data.popr.darkness -= dt * 3.0;
                }
                if *time < 0.0 {
                    self.bg_y = -(RESOLUTION_Y as f32);
                    if data.input.left_pressed {
                        *time = 0.0
                    }
                } else if *time < 3.0 {
                    *time += dt;
                    let t = 1.0 - (*time / 3.0);
                    self.bg_y = -t * t * RESOLUTION_Y as f32;
                } else {
                    self.state = State::Idle;
                }
            }
        }

        self.play_btn.position.y = self.bg_y - 20.0;
        self.sound_toggle.position.y = self.bg_y - 170.0;

        None
    }

    fn render(&mut self, _data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(
            D2Instance {
                position: Vec2::new(0.0, self.bg_y + RESOLUTION_Y as f32),
                ..Default::default()
            },
            "mainplant_hands",
            0,
        );

        sprite_renderer.render(
            D2Instance {
                position: Vec2::new(0.0, self.bg_y),
                ..Default::default()
            },
            "title_bg",
            0,
        );

        sprite_renderer.render(
            D2Instance {
                position: Vec2::new(-11.0, self.bg_y + 120.0),
                ..Default::default()
            },
            "title_title",
            0,
        );

        self.play_btn.render(sprite_renderer);
        self.sound_toggle.render(sprite_renderer);
    }
}
