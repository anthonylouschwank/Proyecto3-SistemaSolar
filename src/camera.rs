use std::f32::consts::PI;

use crate::input::InputState;
use crate::math::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 30.0, 80.0),  
            yaw: 0.0,  
            pitch: -0.3,  
            fov_y: 60.0_f32.to_radians(),
        }
    }

    pub fn forward(&self) -> Vec3 {
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();

        Vec3::new(sy * cp, sp, -cy * cp).normalized()
    }

    pub fn update(&mut self, dt: f32, input: &InputState) {
        let move_speed = 50.0;
        let rot_speed = 1.5;

        if input.mirar_izquierda {
            self.yaw += rot_speed * dt;
        }
        if input.mirar_derecha {
            self.yaw -= rot_speed * dt;
        }
        if input.mirar_arriba {
            self.pitch += rot_speed * dt;
        }
        if input.mirar_abajo {
            self.pitch -= rot_speed * dt;
        }

        let max_pitch = 1.3;
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        }
        if self.pitch < -max_pitch {
            self.pitch = -max_pitch;
        }

        let forward = self.forward();
        let right = forward.cross(Vec3::up()).normalized();
        let mut velocity = Vec3::zero();

        if input.mover_adelante {
            velocity = velocity + forward;
        }
        if input.mover_atras {
            velocity = velocity - forward;
        }
        if input.mover_derecha {
            velocity = velocity + right;
        }
        if input.mover_izquierda {
            velocity = velocity - right;
        }
        if input.mover_arriba {
            velocity.y += 1.0;
        }
        if input.mover_abajo {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * move_speed * dt;
            self.position = self.position + velocity;
        }
    }
}