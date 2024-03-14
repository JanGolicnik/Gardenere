use crate::game::clickableobject::ClickableObject;
use crate::game::GameData;
use crate::game::{
    clickableobject::{ObjectAction, ObjectSprite},
    sprite_renderer::SpriteRenderer,
};
use crate::{clickable, clickable_nohover};
use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::Scene;

pub struct TableScene {
    book: ClickableObject,
    knife: ClickableObject,
    spoon: ClickableObject,
    plate: ClickableObject,
    home: ClickableObject,

    book_opened: bool,
    mainplant_hungry: bool,

    timer: f32,
    pre_timer_distortion: f32,
}

impl TableScene {
    pub fn new(sprite_renderer: &mut SpriteRenderer) -> Self {
        let book = clickable!(-340.0, -50.0, "table_closedbook", sprite_renderer);
        let plate = clickable_nohover!(70.0, -110.0, "table_plate", sprite_renderer);
        let knife = clickable!(284.0, -118.0, "table_knife", sprite_renderer);
        let spoon = clickable!(353.0, -120.0, "table_spoon", sprite_renderer);
        let home = clickable!(-160.0, -260.0, "table_home", sprite_renderer);
        Self {
            book,
            knife,
            spoon,
            plate,
            home,
            book_opened: false,
            mainplant_hungry: false,
            timer: -1.0,
            pre_timer_distortion: 1.0,
        }
    }
}

impl Scene for TableScene {
    fn refresh(&mut self, data: &mut GameData, _sprite_renderer: &mut SpriteRenderer) {
        self.mainplant_hungry = data.main_plant.requires_blood;
    }
    fn update(
        &mut self,
        context: &mut EngineContext,
        _sprite_renderer: &mut SpriteRenderer,
        data: &mut GameData,
    ) -> Option<ObjectAction> {
        if self.timer > 0.0 {
            let over_half = self.timer > 1.5;
            self.timer -= context.dt as f32;
            if self.timer < 0.0 {
                data.popr.distortion = self.pre_timer_distortion;
            } else if over_half && self.timer < 1.5 {
                if self.knife.is_clicked {
                    self.knife.texture = ObjectSprite::Frame("table_knife_blood");
                    data.player.cut_finger = true;
                }
                if self.spoon.is_clicked {
                    self.spoon.texture = ObjectSprite::Frame("table_spoon_blood");
                    data.player.cut_eye = true;
                }
            } else if self.spoon.is_clicked {
                if self.timer > 1.75 {
                    data.popr.distortion += context.dt as f32 * 7.0;
                }
                if (self.timer - 1.5).abs() < 0.25 {
                    data.popr.darkness = 1.0;
                    data.popr.distortion = self.pre_timer_distortion;
                } else {
                    data.popr.darkness = 0.0;
                }
            } else {
                data.popr.distortion += context.dt as f32 * 3.0;
            }

            return None;
        }

        self.knife.scale = 1.0;
        self.spoon.scale = 1.0;

        if self.book_opened {
            if data.input.left_pressed {
                self.book_opened = false;
                data.input.left_pressed = false;
            }
        } else {
            self.book.update(context, data);
            self.home.update(context, data);
            self.plate.update(context, data);
            if self.book.is_clicked {
                self.book_opened = true;
            }
            if self.home.is_clicked {
                return Some(ObjectAction::Goto(super::ActiveScene::House));
            }

            if self.mainplant_hungry {
                if !data.player.cut_finger {
                    self.knife.update(context, data);
                }
                if !data.player.cut_eye {
                    self.spoon.update(context, data);
                }

                if data.player.cut_finger {
                    self.knife.is_hovered = false;
                } else if self.knife.is_clicked {
                    self.timer = 3.0;
                    self.pre_timer_distortion = data.popr.distortion;
                    self.knife.scale = 0.0;
                }

                if data.player.cut_eye {
                    self.spoon.is_hovered = false;
                } else if self.spoon.is_clicked {
                    self.timer = 3.0;
                    self.pre_timer_distortion = data.popr.distortion;
                    self.spoon.scale = 0.0;
                }
            } else {
                self.knife.is_hovered = false;
                self.spoon.is_hovered = false;
            }
        }

        None
    }

    fn render(&mut self, data: &mut GameData, sprite_renderer: &mut SpriteRenderer) {
        sprite_renderer.render(D2Instance::default(), "table_bg", 0);

        if self.book_opened {
            if data.player.has_page {
                sprite_renderer.render(D2Instance::default(), "table_book_fixed", 1);
            } else {
                sprite_renderer.render(D2Instance::default(), "table_book", 1);
            }
        } else {
            self.book.render(sprite_renderer);
            self.home.render(sprite_renderer);
            self.knife.render(sprite_renderer);
            self.spoon.render(sprite_renderer);
            self.plate.render(sprite_renderer);
        }
    }
}
