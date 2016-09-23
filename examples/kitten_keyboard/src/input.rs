pub struct Input {
    pub left_is_pressed: bool,
    pub right_is_pressed: bool,
    pub up_is_pressed: bool,
    pub down_is_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            left_is_pressed: false,
            right_is_pressed: false,
            up_is_pressed: false,
            down_is_pressed: false,
        }
    }
}
