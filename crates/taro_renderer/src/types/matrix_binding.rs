use wgpu::BindGroupLayout;

use crate::RenderHardware;

pub fn create_matrix_bind_layout(hardware: &RenderHardware) -> BindGroupLayout {
    hardware
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Matrix4x4 Bind Group Layout"),
        })
}
