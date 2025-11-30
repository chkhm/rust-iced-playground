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
    Color, Point, Rectangle, Size, Theme, mouse,
    widget::canvas::{Event, Frame},
};

#[derive(Clone, Debug, Default)]
pub struct RectangleShape;

impl RectangleShape {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug)]
pub struct RectangleState {
    pub rect: Rectangle,
    // pub fill: Fill,
    pub drag_start: Option<Point>,
    pub drag_offset: Option<Point>,
}

impl Default for RectangleState {
    fn default() -> Self {
        Self {
            rect: Rectangle {
                x: 100.0,
                y: 50.0,
                width: 100.0,
                height: 50.0,
            },
            // fill: Color::from_rgb(0.0, 0.5, 0.5),
            drag_start: None,
            drag_offset: None,
        }
    }
}

impl RectangleState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
    #[allow(dead_code)]
    pub fn with_dimensions(
        position: Point,
        width: f32,
        height: f32, /* fill: impl Into<Fill> */
    ) -> Self {
        Self {
            rect: Rectangle {
                x: position.x,
                y: position.y,
                width,
                height,
            },
            // fill,
            drag_start: None,
            drag_offset: None,
        }
    }
}

impl RectangleShape {
    pub fn update(
        &self,
        state: &mut RectangleState,
        event: Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> bool {
        // Get the absolute cursor position - return if not available
        let cursor_position = if let Some(position) = cursor.position_in(_bounds) {
            position
        } else {
            return false;
        };

        // we might add more patterns in the future so kill clippy
        #[allow(clippy::single_match)]
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        // Check if the cursor is inside the rectangle
                        if state.rect.contains(cursor_position) {
                            println!("Rectangle clicked at {:?}", cursor_position);
                            state.drag_start = Some(cursor_position);
                            state.drag_offset = Some(Point {
                                x: cursor_position.x - state.rect.x,
                                y: cursor_position.y - state.rect.y,
                            });
                            return true;
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        // Handle button release if needed
                        if state.drag_start.is_some() {
                            println!("Rectangle released at {:?}", cursor_position);
                            state.rect.x = cursor_position.x - state.drag_offset.unwrap().x;
                            state.rect.y = cursor_position.y - state.drag_offset.unwrap().y;
                            state.drag_start = None;
                            state.drag_offset = None;
                            return true;
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        // Handle cursor movement for dragging
                        if let Some(_start) = state.drag_start {
                            println!("Rectangle dragged to {:?}", cursor_position);
                            state.rect.x = cursor_position.x - state.drag_offset.unwrap().x;
                            state.rect.y = cursor_position.y - state.drag_offset.unwrap().y;
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        // No events were handled.
        false
    }

    pub fn draw(
        &self,
        // canvas_cache: &mut Cache,
        state: &RectangleState,
        frame: &mut Frame,
        // _renderer: &Renderer,
        _theme: &Theme,
        // _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) //-> Vec<Geometry>
    {
        // let mut frame = Frame::new(renderer, bounds.size());

        // Draw a rectangle
        let rect = iced::widget::canvas::Path::rectangle(
            Point::new(state.rect.x, state.rect.y),
            Size::new(state.rect.width, state.rect.height),
        );
        frame.fill(&rect, Color::from_rgb(0.0, 0.5, 0.5));

        // vec![frame.into_geometry()]
    }
}
