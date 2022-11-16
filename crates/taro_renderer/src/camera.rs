use boba_core::Pearl;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, TextureView};

use crate::{RenderPearls, RenderPhaseStorage, RenderResources};

#[derive(Clone)]
pub struct TaroCameraSettings {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct TaroCamera {
    pub phases: RenderPhaseStorage,
    pub settings: TaroCameraSettings,
    buffer: Buffer,
    bind_group: BindGroup,
}

impl TaroCamera {
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub fn new(settings: TaroCameraSettings, resources: &RenderResources) -> Self {
        let uniform = Self::build_matrix(&settings);
        let buffer = Self::build_buffer(uniform, resources);
        let layout = Self::create_bind_group_layout(resources);
        let bind_group = Self::build_bind_group(&buffer, &layout, resources);

        Self {
            phases: Default::default(),
            settings,
            buffer,
            bind_group,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn rebuild_matrix(&mut self, resources: &RenderResources) {
        let uniform = Self::build_matrix(&self.settings);
        resources
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn create_bind_group_layout(resources: &RenderResources) -> BindGroupLayout {
        resources
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
                label: Some("camera_bind_group_layout"),
            })
    }

    pub fn execute_render_phases(
        &mut self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        pearls: &RenderPearls,
    ) {
        self.phases
            .execute_phases(view, &self.bind_group, encoder, pearls);
    }

    fn build_matrix(settings: &TaroCameraSettings) -> CameraUniform {
        let view = cgmath::Matrix4::look_at_rh(settings.eye, settings.target, settings.up);
        let proj = cgmath::perspective(
            cgmath::Deg(settings.fovy),
            settings.aspect,
            settings.znear,
            settings.zfar,
        );

        CameraUniform {
            view_proj: (Self::OPENGL_TO_WGPU_MATRIX * proj * view).into(),
        }
    }

    fn build_buffer(uniform: CameraUniform, resources: &RenderResources) -> Buffer {
        resources
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }

    fn build_bind_group(
        buffer: &Buffer,
        layout: &BindGroupLayout,
        resources: &RenderResources,
    ) -> BindGroup {
        resources
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            })
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[derive(Default)]
pub struct CameraStorage {
    pub main_camera: Option<Pearl<TaroCamera>>,
}
