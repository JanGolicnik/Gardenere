use std::{cmp::Ordering, collections::HashMap, hash::Hash};

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

        // let mut textures = vec![err_texture_bg];
        // let mut sprites = HashMap::new();

        // let text = load_text(FilePath::FileName("textures.txt")).await.unwrap();
        // for line in text.lines() {
        //     let texture = load_texture(line, renderer, TextureDescriptor::default())
        //         .await
        //         .unwrap_or_else(|_| panic!("failed loading texture {}", line));
        //     let size = Vec2::new(texture.width() as f32, texture.height() as f32);
        //     let texture_handle = renderer.add_texture(texture);
        //     let texture_bing_group = TextureBindGroup::new(renderer, texture_handle);
        //     let texture_bg = renderer.add_bind_group(texture_bing_group);

        //     textures.push(texture_bg);
        //     let texture = textures.len() - 1;
        //     let handle = line.split('.').collect::<Vec<&str>>()[0].replace('\\', "_");
        //     sprites.insert(handle.to_string(), Sprite { texture, size });
        // }

        let bytes = [
            ("empty", &include_bytes!("../../res/empty.png")[..]),
            ("err", &include_bytes!("../../res/err.png")[..]),
            ("noeye", &include_bytes!("../../res/noeye.png")[..]),
            ("front_bg", &include_bytes!("../../res/front/bg.png")[..]),
            (
                "front_garden",
                &include_bytes!("../../res/front/garden.png")[..],
            ),
            (
                "front_gardenfucked",
                &include_bytes!("../../res/front/gardenfucked.png")[..],
            ),
            (
                "front_gardenfucked_hovered",
                &include_bytes!("../../res/front/gardenfucked_hovered.png")[..],
            ),
            (
                "front_garden_hovered",
                &include_bytes!("../../res/front/garden_hovered.png")[..],
            ),
            (
                "front_house",
                &include_bytes!("../../res/front/house.png")[..],
            ),
            (
                "front_house_hovered",
                &include_bytes!("../../res/front/house_hovered.png")[..],
            ),
            (
                "front_market",
                &include_bytes!("../../res/front/market.png")[..],
            ),
            (
                "front_market_hovered",
                &include_bytes!("../../res/front/market_hovered.png")[..],
            ),
            (
                "front_page",
                &include_bytes!("../../res/front/page.png")[..],
            ),
            (
                "front_page_hovered",
                &include_bytes!("../../res/front/page_hovered.png")[..],
            ),
            (
                "garden_axe",
                &include_bytes!("../../res/garden/axe.png")[..],
            ),
            ("garden_bg", &include_bytes!("../../res/garden/bg.png")[..]),
            (
                "garden_coin",
                &include_bytes!("../../res/garden/coin.png")[..],
            ),
            (
                "garden_eye",
                &include_bytes!("../../res/garden/eye.png")[..],
            ),
            (
                "garden_falling_flower",
                &include_bytes!("../../res/garden/falling_flower.png")[..],
            ),
            (
                "garden_falling_strawberry",
                &include_bytes!("../../res/garden/falling_strawberry.png")[..],
            ),
            (
                "garden_falling_watermelon",
                &include_bytes!("../../res/garden/falling_watermelon.png")[..],
            ),
            (
                "garden_finger",
                &include_bytes!("../../res/garden/finger.png")[..],
            ),
            (
                "garden_fingers",
                &include_bytes!("../../res/garden/fingers.png")[..],
            ),
            (
                "garden_fingersmissing",
                &include_bytes!("../../res/garden/fingersmissing.png")[..],
            ),
            (
                "garden_front",
                &include_bytes!("../../res/garden/front.png")[..],
            ),
            (
                "garden_front_hovered",
                &include_bytes!("../../res/garden/front_hovered.png")[..],
            ),
            (
                "garden_growth6",
                &include_bytes!("../../res/garden/growth6.png")[..],
            ),
            (
                "garden_growth6floor",
                &include_bytes!("../../res/garden/growth6floor.png")[..],
            ),
            (
                "garden_mainpot",
                &include_bytes!("../../res/garden/mainpot.png")[..],
            ),
            (
                "garden_mainpot_hovered",
                &include_bytes!("../../res/garden/mainpot_hovered.png")[..],
            ),
            (
                "garden_pot",
                &include_bytes!("../../res/garden/pot.png")[..],
            ),
            (
                "garden_pot_hovered",
                &include_bytes!("../../res/garden/pot_hovered.png")[..],
            ),
            (
                "garden_water",
                &include_bytes!("../../res/garden/water.png")[..],
            ),
            (
                "garden_wateringcan",
                &include_bytes!("../../res/garden/wateringcan.png")[..],
            ),
            (
                "garden_watermelon_piece",
                &include_bytes!("../../res/garden/watermelon_piece.png")[..],
            ),
            ("house_bed", &include_bytes!("../../res/house/bed.png")[..]),
            (
                "house_bed_hovered",
                &include_bytes!("../../res/house/bed_hovered.png")[..],
            ),
            ("house_bg", &include_bytes!("../../res/house/bg.png")[..]),
            (
                "house_door",
                &include_bytes!("../../res/house/door.png")[..],
            ),
            (
                "house_door_hovered",
                &include_bytes!("../../res/house/door_hovered.png")[..],
            ),
            ("house_guy", &include_bytes!("../../res/house/guy.png")[..]),
            (
                "house_guy_1",
                &include_bytes!("../../res/house/guy_1.png")[..],
            ),
            (
                "house_guy_2",
                &include_bytes!("../../res/house/guy_2.png")[..],
            ),
            (
                "house_table",
                &include_bytes!("../../res/house/table.png")[..],
            ),
            (
                "house_table_hovered",
                &include_bytes!("../../res/house/table_hovered.png")[..],
            ),
            (
                "mainplant_blood",
                &include_bytes!("../../res/mainplant/blood.png")[..],
            ),
            (
                "mainplant_cutting1",
                &include_bytes!("../../res/mainplant/cutting1.png")[..],
            ),
            (
                "mainplant_cutting2",
                &include_bytes!("../../res/mainplant/cutting2.png")[..],
            ),
            (
                "mainplant_cutting3",
                &include_bytes!("../../res/mainplant/cutting3.png")[..],
            ),
            (
                "mainplant_cutting4",
                &include_bytes!("../../res/mainplant/cutting4.png")[..],
            ),
            (
                "mainplant_growth0",
                &include_bytes!("../../res/mainplant/growth0.png")[..],
            ),
            (
                "mainplant_growth1",
                &include_bytes!("../../res/mainplant/growth1.png")[..],
            ),
            (
                "mainplant_growth2",
                &include_bytes!("../../res/mainplant/growth2.png")[..],
            ),
            (
                "mainplant_growth3",
                &include_bytes!("../../res/mainplant/growth3.png")[..],
            ),
            (
                "mainplant_growth4",
                &include_bytes!("../../res/mainplant/growth4.png")[..],
            ),
            (
                "mainplant_growth5",
                &include_bytes!("../../res/mainplant/growth5.png")[..],
            ),
            (
                "mainplant_growth6",
                &include_bytes!("../../res/mainplant/growth6.png")[..],
            ),
            (
                "mainplant_hands",
                &include_bytes!("../../res/mainplant/hands.png")[..],
            ),
            (
                "mainplant_killed_bg",
                &include_bytes!("../../res/mainplant/killed/bg.png")[..],
            ),
            (
                "mainplant_killed_mainplant",
                &include_bytes!("../../res/mainplant/killed/mainplant.png")[..],
            ),
            (
                "mainplant_killed_plants1",
                &include_bytes!("../../res/mainplant/killed/plants1.png")[..],
            ),
            (
                "mainplant_killed_plants2",
                &include_bytes!("../../res/mainplant/killed/plants2.png")[..],
            ),
            (
                "mainplant_killed_vines2",
                &include_bytes!("../../res/mainplant/killed/vines2.png")[..],
            ),
            (
                "mainplant_killed_vines3",
                &include_bytes!("../../res/mainplant/killed/vines3.png")[..],
            ),
            ("market_bg", &include_bytes!("../../res/market/bg.png")[..]),
            (
                "market_coin",
                &include_bytes!("../../res/market/coin.png")[..],
            ),
            (
                "market_front",
                &include_bytes!("../../res/market/front.png")[..],
            ),
            (
                "market_front_hovered",
                &include_bytes!("../../res/market/front_hovered.png")[..],
            ),
            (
                "market_holyaxe",
                &include_bytes!("../../res/market/holyaxe.png")[..],
            ),
            (
                "market_holyaxe_hovered",
                &include_bytes!("../../res/market/holyaxe_hovered.png")[..],
            ),
            (
                "market_pot",
                &include_bytes!("../../res/market/pot.png")[..],
            ),
            (
                "market_pot_hovered",
                &include_bytes!("../../res/market/pot_hovered.png")[..],
            ),
            (
                "market_shopkeep",
                &include_bytes!("../../res/market/shopkeep.png")[..],
            ),
            (
                "market_seeds_flower",
                &include_bytes!("../../res/market/seeds/flower.png")[..],
            ),
            (
                "market_seeds_flower_hovered",
                &include_bytes!("../../res/market/seeds/flower_hovered.png")[..],
            ),
            (
                "market_seeds_strawberry",
                &include_bytes!("../../res/market/seeds/strawberry.png")[..],
            ),
            (
                "market_seeds_strawberry_hovered",
                &include_bytes!("../../res/market/seeds/strawberry_hovered.png")[..],
            ),
            (
                "market_seeds_watermelon",
                &include_bytes!("../../res/market/seeds/watermelon.png")[..],
            ),
            (
                "market_seeds_watermelon_hovered",
                &include_bytes!("../../res/market/seeds/watermelon_hovered.png")[..],
            ),
            (
                "plants_coins",
                &include_bytes!("../../res/plants/coins.png")[..],
            ),
            (
                "plants_flower",
                &include_bytes!("../../res/plants/flower.png")[..],
            ),
            (
                "plants_flower1",
                &include_bytes!("../../res/plants/flower1.png")[..],
            ),
            (
                "plants_flower1_hovered",
                &include_bytes!("../../res/plants/flower1_hovered.png")[..],
            ),
            (
                "plants_flower2",
                &include_bytes!("../../res/plants/flower2.png")[..],
            ),
            (
                "plants_plant1",
                &include_bytes!("../../res/plants/plant1.png")[..],
            ),
            (
                "plants_plant2",
                &include_bytes!("../../res/plants/plant2.png")[..],
            ),
            (
                "plants_strawberry",
                &include_bytes!("../../res/plants/strawberry.png")[..],
            ),
            (
                "plants_strawberry1",
                &include_bytes!("../../res/plants/strawberry1.png")[..],
            ),
            (
                "plants_strawberry2",
                &include_bytes!("../../res/plants/strawberry2.png")[..],
            ),
            (
                "plants_strawberry2_hovered",
                &include_bytes!("../../res/plants/strawberry2_hovered.png")[..],
            ),
            (
                "plants_strawberry3",
                &include_bytes!("../../res/plants/strawberry3.png")[..],
            ),
            (
                "plants_watermelon1",
                &include_bytes!("../../res/plants/watermelon1.png")[..],
            ),
            (
                "plants_watermelon2",
                &include_bytes!("../../res/plants/watermelon2.png")[..],
            ),
            (
                "plants_watermelon3",
                &include_bytes!("../../res/plants/watermelon3.png")[..],
            ),
            (
                "plants_watermelon4",
                &include_bytes!("../../res/plants/watermelon4.png")[..],
            ),
            (
                "plants_watermelon4_hovered",
                &include_bytes!("../../res/plants/watermelon4_hovered.png")[..],
            ),
            (
                "plants_watermelon5",
                &include_bytes!("../../res/plants/watermelon5.png")[..],
            ),
            ("table_bg", &include_bytes!("../../res/table/bg.png")[..]),
            (
                "table_book",
                &include_bytes!("../../res/table/book.png")[..],
            ),
            (
                "table_book_fixed",
                &include_bytes!("../../res/table/book_fixed.png")[..],
            ),
            (
                "table_book_hovered",
                &include_bytes!("../../res/table/book_hovered.png")[..],
            ),
            (
                "table_closedbook",
                &include_bytes!("../../res/table/closedbook.png")[..],
            ),
            (
                "table_closedbook_hovered",
                &include_bytes!("../../res/table/closedbook_hovered.png")[..],
            ),
            (
                "table_home",
                &include_bytes!("../../res/table/home.png")[..],
            ),
            (
                "table_home_hovered",
                &include_bytes!("../../res/table/home_hovered.png")[..],
            ),
            (
                "table_knife",
                &include_bytes!("../../res/table/knife.png")[..],
            ),
            (
                "table_knife_blood",
                &include_bytes!("../../res/table/knife_blood.png")[..],
            ),
            (
                "table_knife_hovered",
                &include_bytes!("../../res/table/knife_hovered.png")[..],
            ),
            (
                "table_plate",
                &include_bytes!("../../res/table/plate.png")[..],
            ),
            (
                "table_spoon",
                &include_bytes!("../../res/table/spoon.png")[..],
            ),
            (
                "table_spoon_blood",
                &include_bytes!("../../res/table/spoon_blood.png")[..],
            ),
            (
                "table_spoon_hovered",
                &include_bytes!("../../res/table/spoon_hovered.png")[..],
            ),
            ("title_bg", &include_bytes!("../../res/title/bg.png")[..]),
            (
                "title_play",
                &include_bytes!("../../res/title/play.png")[..],
            ),
            (
                "title_play_hovered",
                &include_bytes!("../../res/title/play_hovered.png")[..],
            ),
            (
                "title_sound_off",
                &include_bytes!("../../res/title/sound_off.png")[..],
            ),
            (
                "title_sound_on",
                &include_bytes!("../../res/title/sound_on.png")[..],
            ),
            (
                "title_title",
                &include_bytes!("../../res/title/title.png")[..],
            ),
        ];

        let mut textures = vec![err_texture_bg];
        let mut sprites = HashMap::new();

        for (handle, bytes) in bytes {
            let texture = Texture::from_bytes(renderer, bytes, TextureDescriptor::default())
                .unwrap_or_else(|_| panic!("failed loading texture {}", handle));

            let size = Vec2::new(texture.width() as f32, texture.height() as f32);
            let texture_handle = renderer.add_texture(texture);
            let texture_bing_group = TextureBindGroup::new(renderer, texture_handle);
            let texture_bg = renderer.add_bind_group(texture_bing_group);

            textures.push(texture_bg);
            let texture = textures.len() - 1;
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
