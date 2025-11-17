use minifb::{Key, Window};

pub struct InputState {
    pub move_forward: bool,
    pub move_back: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,

    pub look_left: bool,
    pub look_right: bool,
    pub look_up: bool,
    pub look_down: bool,

    pub warp_1: bool,
    pub warp_2: bool,
    pub warp_3: bool,
    pub warp_animated: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            move_forward: false,
            move_back: false,
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
            look_left: false,
            look_right: false,
            look_up: false,
            look_down: false,
            warp_1: false,
            warp_2: false,
            warp_3: false,
            warp_animated: false,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.move_forward = window.is_key_down(Key::W);
        self.move_back = window.is_key_down(Key::S);
        self.move_left = window.is_key_down(Key::A);
        self.move_right = window.is_key_down(Key::D);
        self.move_up = window.is_key_down(Key::E);
        self.move_down = window.is_key_down(Key::Q);

        self.look_left = window.is_key_down(Key::Left);
        self.look_right = window.is_key_down(Key::Right);
        self.look_up = window.is_key_down(Key::Up);
        self.look_down = window.is_key_down(Key::Down);

        self.warp_1 = window.is_key_down(Key::Key1);
        self.warp_2 = window.is_key_down(Key::Key2);
        self.warp_3 = window.is_key_down(Key::Key3);

        self.warp_animated = window.is_key_down(Key::Space);
    }
}
