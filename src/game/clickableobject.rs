use jandering_engine::{engine::EngineContext, object::D2Instance, types::Vec2};

use super::{
    constants::FRAME_LENGTH, scenes::ActiveScene, sprite_renderer::SpriteRenderer, InputInfo,
};

#[derive(Copy, Clone)]
pub enum ObjectAction {
    Goto(ActiveScene),
    Sleep,
    Pressed,
    Exit,
}

#[derive(Clone)]
pub struct ObjectFrame {
    pub tex: &'static str,
    pub frames: u32,
}

#[derive(Clone)]
pub enum ObjectSprite {
    Frame(&'static str),
    Frames(Vec<ObjectFrame>),
}

#[derive(Clone)]
pub struct ClickableObject {
    pub position: Vec2,
    pub size: Vec2,
    pub is_clicked: bool,
    pub is_hovered: bool,
    pub is_held: bool,
    pub z_index: u32,
    pub scale: f32,
    pub rotation: f32,
    pub texture: ObjectSprite,
    pub hovered_texture: ObjectSprite,
    time: f32,
}

impl ClickableObject {
    pub fn new(
        position: Vec2,
        texture: ObjectSprite,
        hovered_texture: ObjectSprite,
        sprite_renderer: &mut SpriteRenderer,
    ) -> Self {
        let first_tex = match &texture {
            ObjectSprite::Frame(tex) => tex,
            ObjectSprite::Frames(vec) => vec[0].tex,
        };
        let size = sprite_renderer.get_sprite(first_tex).size;
        Self {
            texture,
            size,
            hovered_texture,
            position,
            is_clicked: false,
            is_hovered: false,
            is_held: false,
            z_index: 1,
            time: 0.0,
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, context: &EngineContext, input: &mut InputInfo) {
        if let Some(mouse_pos) = input.mouse_pos {
            self.is_hovered = self.is_hovered(mouse_pos);
        }
        self.is_clicked = false;
        self.time += context.dt as f32;
        if input.left_pressed && self.is_hovered {
            self.is_clicked = true;
            self.is_held = true;
        }
        if input.left_released {
            self.is_held = false;
        }

        self.update_sprite();
    }

    pub fn is_hovered(&self, mouse_pos: Vec2) -> bool {
        let half_size = self.size * self.scale * 0.5;

        mouse_pos.x > self.position.x - half_size.x
            && mouse_pos.x < self.position.x + half_size.x
            && mouse_pos.y > self.position.y - half_size.y
            && mouse_pos.y < self.position.y + half_size.y
    }

    pub fn render(&self, sprite_renderer: &mut SpriteRenderer) {
        let texture_handle = self.get_current_frame();

        sprite_renderer.render_with_scale(
            D2Instance {
                position: self.position,
                rotation: self.rotation,
                ..Default::default()
            },
            texture_handle,
            1,
            self.scale,
        )
    }

    pub fn render_at(&self, sprite_renderer: &mut SpriteRenderer, position: Vec2) {
        let texture_handle = self.get_current_frame();

        sprite_renderer.render_with_scale(
            D2Instance {
                position,
                rotation: self.rotation,
                ..Default::default()
            },
            texture_handle,
            1,
            self.scale,
        )
    }

    fn get_current_tex(&self) -> &ObjectSprite {
        if self.is_hovered {
            &self.hovered_texture
        } else {
            &self.texture
        }
    }

    pub fn get_current_frame(&self) -> &str {
        match self.get_current_tex() {
            ObjectSprite::Frame(tex) => tex,
            ObjectSprite::Frames(vec) => {
                let mut acc = 0.0;
                for frame in vec.iter() {
                    acc += frame.frames as f32 * FRAME_LENGTH;
                    if acc > self.time {
                        return frame.tex;
                    }
                }
                vec.last().unwrap().tex
            }
        }
    }

    fn update_sprite(&mut self) {
        if let ObjectSprite::Frames(vec) = self.get_current_tex() {
            let length = vec
                .iter()
                .fold(0.0, |acc, e| acc + e.frames as f32 * FRAME_LENGTH);
            self.time %= length;
        }
    }

    pub fn size(&self, sprite_renderer: &mut SpriteRenderer) -> Vec2 {
        sprite_renderer.get_sprite(self.get_current_frame()).size * self.scale
    }
}

#[macro_export]
macro_rules! clickable {
    ($x: expr, $y: expr, $tex: expr, $sprite_renderer: expr) => {
        ClickableObject::new(
            Vec2::new($x, $y),
            ObjectSprite::Frame($tex),
            ObjectSprite::Frame(concat!($tex, "_hovered")),
            $sprite_renderer,
        )
    };
}

#[macro_export]
macro_rules! clickable_nohover {
    ($x: expr, $y: expr, $tex: expr, $sprite_renderer: expr) => {
        ClickableObject::new(
            Vec2::new($x, $y),
            ObjectSprite::Frame($tex),
            ObjectSprite::Frame($tex),
            $sprite_renderer,
        )
    };
}

#[macro_export]
macro_rules! clickable_idleanim {
    ($x: expr, $y: expr, $sprite_renderer: expr $(, $frame: expr)+ ) => {
        ClickableObject::new(
            Vec2::new($x, $y),
            ObjectSprite::Frames(0.0, vec![$($frame)+]),
            "test",
            $sprite_renderer,
        )
    };
}

#[macro_export]
macro_rules! clickable_fullanim {
    ($x: expr, $y: expr, $sprite_renderer: expr, $first_frame: expr, $(, $frame: expr)+ ) => {
        ClickableObject::new(
            Vec2::new($x, $y),
            ObjectSprite::Frames(0.0, vec![$first_frame, $($frame)+]),
            "test",
            $sprite_renderer,
        )
    };
}

#[macro_export]
macro_rules! frame {
    ($tex: expr, $len: expr) => {
        ObjectFrame {
            tex: $tex,
            frames: $len,
        }
    };
}
