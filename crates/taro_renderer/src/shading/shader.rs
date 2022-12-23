use std::{marker::PhantomData, sync::Arc};

use boba_3d::glam::Mat4;
use bytemuck::Pod;
use once_cell::sync::OnceCell;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, RenderPass};

use crate::{types::CompiledTaroMesh, TaroHardware};

pub struct TaroBinding<T>
where
    T: Pod + Default,
{
    buffer: Buffer,
    bind_group: BindGroup,
    _data_type: PhantomData<T>,
}

impl<T> TaroBinding<T>
where
    T: Pod + Default,
{
    pub fn build(item: T, layout: &BindGroupLayout, hardware: &TaroHardware) -> Self {
        let buffer = hardware
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[item]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group = hardware
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        Self {
            buffer,
            bind_group,
            _data_type: Default::default(),
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn write(&self, item: T, hardware: &TaroHardware) {
        hardware
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[item]));
    }
}

pub struct TaroShader<T> {
    shader: Arc<OnceCell<T>>,
}

impl<T> Default for TaroShader<T>
where
    T: TaroCoreShader,
{
    fn default() -> Self {
        Self {
            shader: Default::default(),
        }
    }
}

impl<T> TaroShader<T>
where
    T: TaroCoreShader,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn compile(&self, hardware: &TaroHardware) -> &T {
        self.shader.get_or_init(|| T::build_instance(hardware))
    }
}

pub trait TaroCoreShader {
    fn build_instance(hardware: &TaroHardware) -> Self;
}

pub trait TaroMeshShader: TaroCoreShader {
    fn render<'pass>(
        &'pass self,
        pass: &mut RenderPass<'pass>,
        mesh: &'pass CompiledTaroMesh,
        camera_matrix: &Mat4,
        model_matrix: &Mat4,
        hardware: &TaroHardware,
    );
}
