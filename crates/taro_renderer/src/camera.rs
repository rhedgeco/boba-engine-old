use std::cell::BorrowError;

use boba_3d::glam::{Mat4, Quat, Vec3};
use boba_3d::pearls::BobaTransform;
use boba_core::{Pearl, PearlRegister};
use log::error;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, TextureView};

use crate::{RenderPearls, RenderPhaseStorage, RenderResources};

#[derive(Clone)]
pub struct TaroCameraSettings {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct TaroCamera {
    transform: Pearl<BobaTransform>,
    pub phases: RenderPhaseStorage,
    pub settings: TaroCameraSettings,
    buffer: Buffer,
    bind_group: BindGroup,
}

impl PearlRegister for TaroCamera {
    fn register(_: Pearl<Self>, _: &mut boba_core::storage::StageRunners) {
        // do nothing for now
    }
}

impl TaroCamera {
    pub fn new(
        transform: Pearl<BobaTransform>,
        settings: TaroCameraSettings,
        resources: &RenderResources,
    ) -> Result<Self, BorrowError> {
        let tdata = transform.data()?;

        let uniform = Self::build_matrix(tdata.position(), tdata.rotation(), &settings);
        let buffer = Self::build_buffer(uniform, resources);
        let layout = Self::create_bind_group_layout(resources);
        let bind_group = Self::build_bind_group(&buffer, &layout, resources);

        drop(tdata);
        Ok(Self {
            transform,
            phases: Default::default(),
            settings,
            buffer,
            bind_group,
        })
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn rebuild_matrix(&mut self, resources: &RenderResources) {
        let Ok(tdata) = self.transform.data() else {
            error!("Could not rebuild matrix. Transform is borrowed as mutable");
            return;
        };

        let uniform = Self::build_matrix(tdata.position(), tdata.rotation(), &self.settings);
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

    fn build_matrix(
        position: Vec3,
        rotation: Quat,
        settings: &TaroCameraSettings,
    ) -> CameraUniform {
        let target = rotation * Vec3::Z;
        let view = Mat4::look_at_rh(position, target, Vec3::Y);
        let proj = Mat4::perspective_rh(
            settings.fovy,
            settings.aspect,
            settings.znear,
            settings.zfar,
        );

        CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
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
                layout,
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
