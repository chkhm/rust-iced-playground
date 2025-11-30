use iced::{
    Alignment, Length, Subscription,
    widget::{Canvas, button, column, text},
};
use sweeten::widget::mouse_area::MouseArea;

// ...existing code...

use std::time::Duration;

mod circle_and_line_program;
use circle_and_line_program::CircleAndLineProgram;

fn main() -> iced::Result {
    //iced::run("My App", MyApp::update, MyApp::view)
    iced::application("iced svg polygon", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    _AreaClicked,
    RotateStop,
    Tick,
}

#[derive(Debug, Default)]
struct MyApp {
    rotating: bool,
    rotation_angle: f32,
    mouse_state_text: String,
    circle_and_line_program: CircleAndLineProgram,
}

impl MyApp {
    fn subscription(&self) -> Subscription<Message> {
        if self.rotating {
            iced::time::every(Duration::from_millis(10)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, _message: Message) {
        match _message {
            Message::RotateStop => {
                self.rotating = !self.rotating;
            }
            Message::Tick => {
                self.rotation_angle = (self.rotation_angle + 0.25) % 360.0;
                self.circle_and_line_program.rotation_angle = self.rotation_angle;
            }
            Message::_AreaClicked => {} //todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            button(text(if !self.rotating { "Play" } else { "Pause" }))
                .on_press(Message::RotateStop),
            text(format!("Rotation Angle: {:.2}°", self.rotation_angle)),
            text(self.mouse_state_text.clone()),
            "A Canvas",
            MouseArea::new(
                Canvas::new(&self.circle_and_line_program)
                    // Canvas::new(CircleAndLineProgram::new(self.rotation_angle))
                    .width(Length::Fill)
                    .height(Length::Fill)
            ) //.on_press_with(on_press)
              //.on_pressed_with(),
              //.on_press(Message::AreaClicked),
        ]
        .align_x(Alignment::Center)
        .into()
    }
}
