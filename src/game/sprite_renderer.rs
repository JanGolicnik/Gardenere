use std::{cmp::Ordering, collections::HashMap};

use jandering_engine::{
    bind_group::{camera::d2::D2CameraBindGroup, texture::TextureBindGroup},
    engine::EngineContext,
    object::{primitives, D2Instance, Object, VertexRaw},
    renderer::{BindGroupHandle, Renderer},
    shader::{create_shader, Shader, ShaderDescriptor},
    texture::{load_texture, Texture, TextureDescriptor},
    types::{UVec2, Vec2},
    utils::{load_text, FilePath},
};

struct QueuedSprite {
    instance: D2Instance,
    texture_handle: TextureHandle,
    z_index: u32,
}

const MAX_NUM_SPRITES: usize = 256;

type TextureHandle = usize;

type Quad = Object<D2Instance>;

pub struct Sprite {
    pub texture: TextureHandle,
    pub size: Vec2,
}

pub struct SpriteRenderer {
    quads: [Quad; MAX_NUM_SPRITES],
    queued: Vec<QueuedSprite>,
    camera_bg: BindGroupHandle<D2CameraBindGroup>,
    shader: Shader,

    sprites: HashMap<String, Sprite>,
    textures: Vec<BindGroupHandle<TextureBindGroup>>,
}

impl SpriteRenderer {
    pub async fn new(
        renderer: &mut Renderer,
        camera_bg: BindGroupHandle<D2CameraBindGroup>,
    ) -> Self {
        let quads: Vec<Quad> = (0..MAX_NUM_SPRITES)
            .map(|_| primitives::quad(renderer, vec![D2Instance::default()]))
            .collect();

        let err_texture = renderer.add_texture(Texture::new_color(
            renderer,
            UVec2::new(1, 1),
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            None,
        ));

        let err_texture_bind_group = TextureBindGroup::new(renderer, err_texture);
        let err_texture_bg = renderer.add_bind_group(err_texture_bind_group);

        let bind_groups = [camera_bg.into(), err_texture_bg.into()];

        let shader = create_shader(
            renderer,
            ShaderDescriptor {
                descriptors: &[VertexRaw::desc(), D2Instance::desc()],
                bind_groups: &bind_groups,
                targets: Some(&[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })]),
                ..ShaderDescriptor::default_flat()
            },
        );

        let mut textures = vec![err_texture_bg];
        let mut sprites = HashMap::new();

        let text = load_text(FilePath::FileName("textures.txt")).await.unwrap();
        for line in text.lines() {
            let texture = load_texture(line, renderer, TextureDescriptor::default())
                .await
                .unwrap_or_else(|_| panic!("failed loading texture {}", line));
            let size = Vec2::new(texture.width() as f32, texture.height() as f32);
            let texture_handle = renderer.add_texture(texture);
            let texture_bing_group = TextureBindGroup::new(renderer, texture_handle);
            let texture_bg = renderer.add_bind_group(texture_bing_group);

            textures.push(texture_bg);
            let texture = textures.len() - 1;
            let handle = line.split('.').collect::<Vec<&str>>()[0].replace('\\', "_");
            sprites.insert(handle.to_string(), Sprite { texture, size });
        }

        Self {
            quads: match quads.try_into() {
                Ok(arr) => arr,
                Err(..) => {
                    panic!()
                }
            },
            queued: Vec::new(),
            camera_bg,
            shader,

            sprites,
            textures,
        }
    }

    pub fn get_sprite(&mut self, handle: &str) -> &Sprite {
        match self.sprites.get(handle) {
            Some(sprite) => sprite,
            None => panic!("missing sprite {}", handle),
        }
    }

    pub fn render_handle(
        &mut self,
        instance: D2Instance,
        texture_handle: TextureHandle,
        z_index: u32,
    ) {
        self.queued.push(QueuedSprite {
            instance,
            texture_handle,
            z_index,
        });
    }

    pub fn render(&mut self, mut instance: D2Instance, sprite_name: &str, z_index: u32) {
        let sprite = if let Some(sprite) = self.sprites.get(sprite_name) {
            sprite
        } else {
            log::error!("sprite {} not found, using err tex", sprite_name);
            &self.sprites["err"]
        };
        instance.scale = sprite.size;
        self.queued.push(QueuedSprite {
            instance,
            texture_handle: sprite.texture,
            z_index,
        });
    }

    pub fn render_with_scale(
        &mut self,
        mut instance: D2Instance,
        sprite_name: &str,
        z_index: u32,
        scale: f32,
    ) {
        let sprite = if let Some(sprite) = self.sprites.get(sprite_name) {
            sprite
        } else {
            log::error!("sprite {} not found, using err tex", sprite_name);
            &self.sprites["err"]
        };
        instance.scale = sprite.size * scale;
        self.queued.push(QueuedSprite {
            instance,
            texture_handle: sprite.texture,
            z_index,
        });
    }

    pub fn submit(&mut self, context: &mut EngineContext, renderer: &mut Renderer) {
        self.queued.sort_unstable_by(|a, b| {
            if a.z_index == b.z_index {
                //for some reason this doesnt work:
                //a.instance.position.y.cmp(&b.instance.position.y)
                if a.instance.position.y < b.instance.position.y {
                    Ordering::Greater
                } else if a.instance.position.y == b.instance.position.y {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                a.z_index.cmp(&b.z_index)
            }
        });

        for (i, queued) in self.queued.iter().take(MAX_NUM_SPRITES).enumerate() {
            let quad = &mut self.quads[i];
            quad.instances[0] = queued.instance;
            quad.update(context, renderer);
            renderer.render(
                &[quad],
                context,
                &self.shader,
                &[
                    self.camera_bg.into(),
                    self.textures[queued.texture_handle].into(),
                ],
            )
        }

        self.queued.clear();
    }
}
