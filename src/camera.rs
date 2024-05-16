use winit::{event::{ElementState, KeyEvent, WindowEvent}, keyboard::{Key, KeyCode, NamedKey, PhysicalKey}};

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // 旋转世界坐标到到摄像机所观察的位置，本质是摄像机变换的逆矩阵
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        // 变换场景空间，产生景深效果
        let proj = glam::Mat4::perspective_rh(self.fovy.to_radians(),self.aspect,self.znear,self.zfar);

        proj * view
    }

    pub fn new(aspect:f32) -> Self {
        Camera {
            eye: (0.0,1.0,2.0).into(),
            // 看向原点
            target: (0.0,0.0,0.0).into(),
            up: glam::Vec3::Y,
            aspect: aspect,
            fovy: 45.0,
            zfar: 100.0,
            znear:0.1
        }
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool
}

impl CameraController {
    pub fn new(speed: f32)-> Self{
        Self {
            speed,
            is_backward_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false
        }
    }

    pub fn process_events(&mut self,event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {  event: 
                KeyEvent{
                    state,
                    physical_key,
                    ..
                }, 
                .. 
            } => {
                let is_pressed = *state == ElementState::Pressed;
               
                match physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }

            _ => false
        }
    }


    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // 防止摄像机离场景中心太近时出现问题
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // 在按下前进或后退键时重做半径计算
        let forward = camera.target - camera.eye;
        let forward_mag = forward.length();

        if self.is_right_pressed {
            // 重新调整目标和眼睛之间的距离，以便其不发生变化。
            // 因此，眼睛仍然位于目标和眼睛形成的圆圈上。
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

         
    }
}