use fltk::{
    button::Button,
    enums::{Color, Event, FrameType},
    prelude::*,
};
use std::ops::{Deref, DerefMut};



pub struct MyButton {
    btn: Button,
}

impl MyButton {
    #[must_use]
    pub fn new() -> MyButton {
        let mut b = MyButton {
            btn: Button::new(0, 0, 200, 0, ""),
        };
        b.set_label_size(100);
        b.set_frame(FrameType::GleamUpBox);
        b.set_label_color(Color::White);
        b.set_selection_color(Color::from_hex(0x001b_1b1b));
        b.set_color(Color::from_hex(0x42_4242));
        b.handle(move |b, ev| match ev {
            Event::Enter => {
                b.set_color(Color::from_hex(0x2b_2b2b));
                b.redraw();
                true
            }
            Event::Leave => {
                b.set_color(Color::from_hex(0x42_4242));
                b.redraw();
                true
            }
            _ => false,
        });
        b
    }
}

impl Default for MyButton {
    fn default() -> Self {
        MyButton::new()
    }
}

impl Deref for MyButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}
