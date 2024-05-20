pub struct Instance {
    pub position: glam::Vec3,
    pub rotation: glam::Quat,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (glam::Mat4::from_translation(self.position) * glam::Mat4::from_quat(self.rotation)).to_cols_array_2d()
        }
    }
}

#[repr(C)]
#[derive(Debug,Clone,Copy,bytemuck::Pod,bytemuck::Zeroable)]
// 四元数的矩阵形式
pub struct InstanceRaw{
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout { 
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Instance, 
            attributes: &[
                // mat4 从技术的角度来看是由 4 个 vec4 构成，占用 4 个插槽。
                // 我们需要为每个 vec4 定义一个插槽，然后在着色器中重新组装出 mat4。
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ] 
        }
    }
}


