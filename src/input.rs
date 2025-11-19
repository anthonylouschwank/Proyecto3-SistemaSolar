use minifb::{Key, Window};

pub struct InputState {
    pub mover_adelante: bool,
    pub mover_atras: bool,
    pub mover_izquierda: bool,
    pub mover_derecha: bool,
    pub mover_arriba: bool,
    pub mover_abajo: bool,

    pub mirar_izquierda: bool,
    pub mirar_derecha: bool,
    pub mirar_arriba: bool,
    pub mirar_abajo: bool,

    pub warp_1: bool,
    pub warp_2: bool,
    pub warp_3: bool,
    pub warp_animated: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mover_adelante: false,
            mover_atras: false,
            mover_izquierda: false,
            mover_derecha: false,
            mover_arriba: false,
            mover_abajo: false,
            mirar_izquierda: false,
            mirar_derecha: false,
            mirar_arriba: false,
            mirar_abajo: false,
            warp_1: false,
            warp_2: false,
            warp_3: false,
            warp_animated: false,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.mover_adelante = window.is_key_down(Key::W);
        self.mover_atras = window.is_key_down(Key::S);
        self.mover_izquierda = window.is_key_down(Key::A);
        self.mover_derecha = window.is_key_down(Key::D);
        self.mover_arriba = window.is_key_down(Key::E);
        self.mover_abajo = window.is_key_down(Key::Q);

        self.mirar_izquierda = window.is_key_down(Key::Left);
        self.mirar_derecha = window.is_key_down(Key::Right);
        self.mirar_arriba = window.is_key_down(Key::Up);
        self.mirar_abajo = window.is_key_down(Key::Down);

        self.warp_1 = window.is_key_down(Key::Key1);
        self.warp_2 = window.is_key_down(Key::Key2);
        self.warp_3 = window.is_key_down(Key::Key3);

        self.warp_animated = window.is_key_down(Key::Space);
    }
}
