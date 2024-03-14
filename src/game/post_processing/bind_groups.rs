use jandering_engine::bind_group::{BindGroup, BindGroupRenderData, BindGroupWriteData};
use jandering_engine::renderer::Renderer;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PoprBindGroupUniform {
    pub factor: f32,
    pub time: f32,
    pub distortion: f32,
    pub vignette: f32,
}

pub struct PoprBindGroup {
    pub uniform: PoprBindGroupUniform,
    render_data: BindGroupRenderData,
}

impl BindGroup for PoprBindGroup {
    fn get_bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout> {
        Some(&self.render_data.bind_group_layout)
    }

    fn get_bind_group(&self) -> Option<&wgpu::BindGroup> {
        Some(&self.render_data.bind_group)
    }

    fn write(&mut self, data: &BindGroupWriteData) {
        data.queue.write_buffer(
            &self.render_data.buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );
    }
}

impl PoprBindGroup {
    pub fn new(renderer: &Renderer) -> Self {
        let uniform = PoprBindGroupUniform {
            factor: 0.0,
            time: 0.0,
            distortion: 1.0,
            vignette: 1.0,
        };

        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("PoprBindGroup Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("PoprBindGroup_bind_group_layout"),
                });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("PoprBindGroup_bind_group"),
            });

        Self {
            uniform,
            render_data: BindGroupRenderData {
                buffer,
                bind_group_layout,
                bind_group,
            },
        }
    }
}
