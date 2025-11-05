use iced::{
    // Alignment,
    Color,
    // Length,
    Point,
    Rectangle,
    Renderer,
    // Subscription,
    Theme,
    // Vector,
    mouse,
    widget::{
        // Canvas, MouseArea, button,
        canvas::{Event, Frame, Geometry, Path, Program, Stroke, Style, gradient::Linear},
        // column, text,
    },
};

use iced::mouse::Event::ButtonPressed;
use iced::mouse::Event::ButtonReleased;

// use std::time::Duration;

use super::util::rotate_point;

/**
 *  size values are percentage of frame size.
 *  the rectangle center is in the middle of the frame the view will rotate it around this center point.
 *  the circle is centered in the middle of the frame. It won't rotate. Maybe later when we fill it with some color pattern.
 */
#[derive(Debug)]
pub struct CircleAndLineProgram {
    circle_radius: f32,
    line_start: Point, // percentage of frame size
    line_end: Point,   // percentage of frame size
    line_width: f32,   // percentage of min(frame.width, frame.height)
    line_rotate: f32,  // degrees
    is_resizing: bool,
}

impl Default for CircleAndLineProgram {
    fn default() -> Self {
        Self {
            circle_radius: 0.25, // 25% frame size
            line_start: Point::new(0.10, 0.40),
            line_end: Point::new(0.90, 0.40),
            line_width: 0.20,
            line_rotate: 0.0, // degrees
            is_resizing: false,
        }
    }
}

impl CircleAndLineProgram {
    pub fn new(rotation: f32) -> Self {
        Self {
            circle_radius: 0.25,
            line_start: Point::new(0.10, 0.40),
            line_end: Point::new(0.90, 0.40),
            line_width: 0.20, // percentage of the min(frame.width, frame.height)
            line_rotate: rotation, // degrees 0 - 359
            is_resizing: false, // true if the rectangle is being resized
        }
    }
}

impl<Message> Program<Message> for CircleAndLineProgram {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (iced::widget::canvas::event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(ButtonPressed(mouse::Button::Left)) => {
                println!("Canvas clicked at position: {:?}", cursor_position);
            }
            Event::Mouse(ButtonReleased(mouse::Button::Left)) => {
                println!("Canvas release at position: {:?}", cursor_position);
            }
            _ => {}
        }

        // No interaction implemented yet
        (iced::widget::canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // border rectangle
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.0, 0.2, 0.4));

        let frame_min = frame.width().min(frame.height());

        // the circle int the center
        frame.fill(
            &Path::circle(frame.center(), frame_min * self.circle_radius),
            Color::from_rgb(0.6, 0.8, 1.0),
        );

        // gradient for the rotating rectangle
        // we have a diagonal rainbow gradient from red via green to blue
        // green is a very small segment because the color is so dominant
        // Note that the rainbow will not rotate with the rectangle. That would be an interesting new challenge.
        let gradient = Linear::new(
            Point::new(0.0, 0.0),
            Point::new(bounds.width, bounds.height),
        )
        .add_stop(0.0, Color::from_rgb(1.0, 0.0, 0.0))
        .add_stop(0.3, Color::from_rgb(0.9, 0.05, 0.0))
        .add_stop(0.47, Color::from_rgb(0.75, 0.75, 0.0))
        .add_stop(0.5, Color::from_rgb(0.0, 1.0, 0.0))
        .add_stop(0.53, Color::from_rgb(0.0, 0.75, 0.75))
        .add_stop(0.7, Color::from_rgb(0.0, 0.05, 0.75))
        .add_stop(1.0, Color::from_rgb(0.0, 0.0, 1.0));

        // the rotating rectangle

        //let angle = (i as f64 * 2.0 * PI) / self.edges as f64;
        //let x = center_x + radius * angle.cos();
        //let y = center_y + radius * angle.sin();

        //let height_vector = Vector::new(
        //    (frame_min * self.rect_height / 2.0) * self.rect_rotate.to_radians().cos(),
        //    (frame_min * self.rect_height / 2.0) * self.rect_rotate.to_radians().sin(),
        //);

        let start_point = Point::new(
            frame.width() * self.line_start.x,
            frame.height() * self.line_start.y,
        );

        let end_point = Point::new(
            frame.width() * self.line_end.x,
            frame.height() * self.line_end.y,
        );

        frame.stroke(
            &Path::line(
                rotate_point(&start_point, &frame.center(), &self.line_rotate),
                rotate_point(&end_point, &frame.center(), &self.line_rotate),
                //start_point,
                //end_point,
            ),
            Stroke {
                //style: Color::WHITE.into(),
                style: Style::Gradient(gradient.into()),
                width: frame_min * self.line_width,
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
