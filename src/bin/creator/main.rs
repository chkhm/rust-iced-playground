/*!

Main entry point for the creator binary.

This binrary implements a shared whiteboard application where multiple users can draw shapes.
It uses the iced GUI library for rendering and user interaction.

It is a work in progress and currently only supports basic shapes and local drawing.

Longterm concept:
- Users can draw shapes (circles, rectangles, lines) on a shared canvas (they are all realized as iced canvas programs).
- Each shape is represented as a separate canvas program that handles its own rendering and interaction.
- The main application manages the collection of shapes and their layout on the canvas.
- The shared canvas is theoretically of infinite size, with panning and zooming capabilities.
- The point (0,0) is at the center of the canvas with
  - negative x and negative y is on the upper left quadrant,
  - positive x and negative y is on the upper right quadrant,
  - negative x and positive y is on the lower left quadrant,
  - positive x and positive y is on the lower right quadrant.

- The canvas grows as users draw near the edges.

- Future plans include networking support for real-time collaboration, shape manipulation (move, resize, rotate), and more complex shapes.
- The architecture is designed to be modular and extensible, allowing for easy addition of new shape types and features.

*/

use std::sync::Arc;

use iced::widget::{Canvas, column, text};
use iced::{Alignment, Length, Point};

mod message;
use message::Message;
mod program_canvas;
mod shape_rectangle;
use program_canvas::CanvasProgram;

// use sweeten::widget::mouse_area;

fn main() -> iced::Result {
    // CreatorApp::run(Settings::default())
    iced::application("Creator", CreatorApp::update, CreatorApp::view)
        //    // .subscription(MyApp::subscription)
        .run()
}

#[derive(Debug, Default)]
struct CreatorApp {
    // canvas_cache: Cache,
    // canvas_programs: Vec<Box<dyn Program<Message, State = ()>>>,
    cursor_pos: Point,
}

impl CreatorApp {
    fn update(&mut self, _message: Message) {
        match _message {
            Message::CanvasMouseMoved(cursor_pos) => {
                self.cursor_pos = cursor_pos;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let c = Canvas::new(CanvasProgram::new())
            .width(Length::Fill)
            .height(Length::Fill);

        //let ma = mouse_area(c).on_press_with(|cursor_pos| Message::CanvasMousePressed(cursor_pos));
        //let ma = mouse_area(c)
        //    .on_press_with(Message::CanvasMousePressed)
        //    .on_release(Message::CanvaseMouseReleased)
        //    .on_move(Message::CanvasMouseMoved);

        column![
            "Creator Canvas",
            text(format!("({},{})", self.cursor_pos.x, self.cursor_pos.y)),
            c,
        ]
        .align_x(Alignment::Center)
        .into()
    }
}
