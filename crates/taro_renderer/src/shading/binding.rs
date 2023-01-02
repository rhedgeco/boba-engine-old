use std::{any::type_name, marker::PhantomData};

use bytemuck::Pod;
use wgpu::{util::DeviceExt, BindGroup};

use crate::TaroHardware;

pub struct TaroBinding<T> {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    _data_type: PhantomData<T>,
}

impl<T> TaroBinding<T>
where
    T: Pod + Default,
{
    pub fn new(item: T, layout: &wgpu::BindGroupLayout, hardware: &TaroHardware) -> Self {
        let buffer = hardware
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Buffer", type_name::<T>())),
                contents: bytemuck::cast_slice(&[item]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some(&format!("{} BindGroup", type_name::<T>())),
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
            .queue()
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[item]))
    }
}