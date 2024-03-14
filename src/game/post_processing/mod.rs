use jandering_engine::{
    bind_group::texture::TextureBindGroup,
    engine::EngineContext,
    object::{primitives, D2Instance, Object, VertexRaw},
    renderer::{BindGroupHandle, Renderer, TextureHandle, UntypedBindGroupHandle},
    shader::Shader,
    texture::{load_texture, Texture, TextureDescriptor},
    types::UVec2,
};

use self::bind_groups::PoprBindGroup;

mod bind_groups;

pub struct PostProcessing {
    quad: Object<D2Instance>,
    fade_shader: Shader,
    factor_bg: BindGroupHandle<PoprBindGroup>,
    bind_groups: [UntypedBindGroupHandle; 3],
    pub target_texture: TextureHandle,
    pub darkness: f32,
    pub time: f32,
    pub distortion: f32,
    pub vignette: f32,
}

impl PostProcessing {
    pub async fn new(renderer: &mut Renderer) -> Self {
        let quad = primitives::quad(renderer, vec![D2Instance::default()]);

        let target_texture = renderer.add_texture(Texture::new_color(
            renderer,
            UVec2::new(renderer.config.width, renderer.config.height),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            Some(wgpu::TextureFormat::Rgba16Float),
        ));

        let factor_bg = renderer.add_bind_group(PoprBindGroup::new(renderer));

        let target_texture_bind_group = TextureBindGroup::new(renderer, target_texture);
        let target_texture_bg = renderer.add_bind_group(target_texture_bind_group);

        let paper_texture = Texture::from_bytes(
            renderer,
            include_bytes!("../../../res/paper.jpg"),
            TextureDescriptor {
                is_normal_map: true,
                ..Default::default()
            },
        )
        .unwrap();
        let paper_texture = renderer.add_texture(paper_texture);
        let paper_texture_bg = TextureBindGroup::new(renderer, paper_texture);
        let paper_texture_bg = renderer.add_bind_group(paper_texture_bg);

        let bind_groups = [
            factor_bg.into(),
            target_texture_bg.into(),
            paper_texture_bg.into(),
        ];

        let fade_shader = jandering_engine::shader::create_shader(
            renderer,
            jandering_engine::shader::ShaderDescriptor {
                code: include_str!("fade_shader.wgsl"),
                descriptors: &[VertexRaw::desc(), D2Instance::desc()],
                bind_groups: &bind_groups,
                ..Default::default()
            },
        );

        Self {
            quad,
            fade_shader,
            bind_groups,
            target_texture,
            factor_bg,
            darkness: 0.0,
            time: 0.0,
            distortion: 0.7,
            vignette: 1.0,
        }
    }

    pub fn render_tonemap(&mut self, renderer: &mut Renderer, context: &mut EngineContext) {
        self.time += self.distortion * context.dt as f32;
        let factor = renderer.get_bind_group_t_mut(self.factor_bg).unwrap();
        factor.uniform.factor = 1.0 - self.darkness;
        factor.uniform.time = self.time;
        factor.uniform.distortion = self.distortion;
        factor.uniform.vignette = self.vignette;

        renderer.set_target_surface();
        renderer.render(&[&self.quad], context, &self.fade_shader, &self.bind_groups);
    }
}
