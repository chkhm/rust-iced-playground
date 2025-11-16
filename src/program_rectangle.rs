//
// This file defines a rectangle program for the Iced GUI framework.
//
// Concept:
//  - We work with a canvas of theoretically infinite size.
//  - The coordinate (0,0) is in the center of the canvas.
//  - Positive X is to the right, positive Y is downwards.
//  - The visible area is defined by the bounds parameter in the draw() function.
//  - the rectangle is defined by its top-left corner position, width, height, and fill color.
//

use iced::{
    Color, Point, Size,
    widget::canvas::{self, Canvas, Cursor, Frame, Geometry, Program},
};

pub struct RectangleProgram;

#[derive(Clone, Debug)]
pub struct RectangleState {
    pub position: Point, // Top-left corner
    pub width: f32,
    pub height: f32,
    pub fill: impl Into<Fill>,
}

impl Default for RectangleState {
    fn default() -> Self {
        Self {
            position: Point::new(-50.0, -25.0),
            width: 100.0,
            height: 50.0,
            fill: Color::from_rgb(0.0, 0.5, 0.5),
        }
    }
}

impl Program<Message> for RectangleProgram {
    type State = RectangleState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw a rectangle
        let rect = iced::widget::canvas::Path::rectangle(
            _state.position,
            Size::new(_state.width, _state.height),
        );
        frame.fill(&rect, Color::from_rgb(0.0, 0.5, 0.5));

        vec![frame.into_geometry()]
    }
}
