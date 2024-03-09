use jandering_engine::{
    bind_group::texture::TextureBindGroup,
    engine::EngineContext,
    object::{primitives, D2Instance, Object, VertexRaw},
    renderer::{BindGroupHandle, Renderer, TextureHandle, UntypedBindGroupHandle},
    shader::Shader,
    texture::Texture,
    types::UVec2,
};

use self::bind_groups::FactorBindGroup;

mod bind_groups;

pub struct PostProcessing {
    quad: Object<D2Instance>,
    fade_shader: Shader,
    factor_bg: BindGroupHandle<FactorBindGroup>,
    bind_groups: [UntypedBindGroupHandle; 2],
    pub target_texture: TextureHandle,
    pub darkness: f32,
}

impl PostProcessing {
    pub fn new(renderer: &mut Renderer) -> Self {
        let quad = primitives::quad(renderer, vec![D2Instance::default()]);

        let target_texture = renderer.add_texture(Texture::new_color(
            renderer,
            UVec2::new(renderer.config.width, renderer.config.height),
            wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            Some(wgpu::TextureFormat::Rgba16Float),
        ));

        let factor_bg = renderer.add_bind_group(FactorBindGroup::new(renderer));

        let target_texture_bind_group = TextureBindGroup::new(renderer, target_texture);
        let target_texture_bg = renderer.add_bind_group(target_texture_bind_group);
        let bind_groups = [factor_bg.into(), target_texture_bg.into()];

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
        }
    }

    fn set_factor(&mut self, renderer: &mut Renderer, val: f32) {
        let factor = renderer.get_bind_group_t_mut(self.factor_bg).unwrap();
        factor.uniform.factor = val;
    }

    pub fn render_tonemap(&mut self, renderer: &mut Renderer, context: &mut EngineContext) {
        self.set_factor(renderer, 1.0 - self.darkness);
        renderer.set_target_surface();
        renderer.render(&[&self.quad], context, &self.fade_shader, &self.bind_groups);
    }
}
