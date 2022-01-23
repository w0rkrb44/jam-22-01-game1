#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

#[rustfmt::skip]
const SMILEY2: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100000,
    0b00100000,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

pub struct Game {
    frame_count: u32,
    starting: bool,
}

//TODO: find out howto initialize vars... These are not assignments does not work.
impl Game {
    pub fn new() -> Self {
        Self {
            starting: true,
            frame_count: 400,
        }
    }

    //TODO: figure out how background color is set and also how update gets called when I have not
    //stated the existence of class Game to whatever calls update function...
    #[no_mangle]
    fn update(&mut self) {
        if self.starting {
            self.starting = false;
            self.frame_count = 400;
        }

        let gamepad = unsafe { *GAMEPAD1 };
        unsafe { *DRAW_COLORS = 2 }
        text("Hello from Rust!", 10, 10);
        blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);

        if gamepad & BUTTON_1 != 0 {
            self.frame_count = 0;
        }

        if self.frame_count > 0 && self.frame_count < 10 {
            unsafe { *DRAW_COLORS = 3 }
        }
        if self.frame_count > 10 && self.frame_count < 20 {
            unsafe { *DRAW_COLORS = 2 }
        }
        if self.frame_count > 20 && self.frame_count < 30 {
            unsafe { *DRAW_COLORS = 3 }
        }
        if self.frame_count > 30 && self.frame_count < 40 {
            unsafe { *DRAW_COLORS = 2 }
        }
        if self.frame_count > 40 && self.frame_count < 50 {
            unsafe { *DRAW_COLORS = 3 }
        }

        if self.frame_count > 75 && self.frame_count < 90 {
            blit(&SMILEY2, 76, 76, 8, 8, BLIT_1BPP);
        }

        let text_str = format!("Press X to blink");
        text(text_str, 10, 90);
        self.frame_count += 1;
    }
}
