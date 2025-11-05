use iced::{
    // Alignment,
    Color,
    Point,
    Rectangle,
    Renderer,
    Theme,
    futures::io::Cursor,
    mouse,
    widget::canvas::{Event, Frame, Geometry, Path, Program, Stroke, Style, gradient::Linear},
};

use iced::mouse::Event::ButtonPressed;
use iced::mouse::Event::ButtonReleased;

// use std::time::Duration;

use crate::util::rotate_line;

use super::util::rotate_point;

/**
 * This little program draws a circle and a line across the circle. The line starts and ends ath the default positions.
 * Originally the line is horizontal but it rotates around the center of the frame.
 *
 * We also draw the position where the user clicked on the canvas as a red dot. The point is rotated back to the original coordinate system.
 * If the user clicked on the line a message is printed to the console for debugging purposes.
 *
 *  Size values are percentage of frame size.
 *  the line center is in the middle of the frame the view will rotate it around this center point.
 *  the circle is centered in the middle of the frame. It won't rotate. Maybe later when we fill it with some color pattern.
 *
 * The draw function scales to the relative sizes to the absolute frame sizes.
 *
 */
#[derive(Debug, Default)]
pub struct CircleAndLineProgram {
    pub rotation_angle: f32, // degrees
}

/**
 * holds the state of the program.
 */
#[derive(Debug)]
pub struct CicleAndLineState {
    circle_radius: f32,
    line_start: Point, // percentage of frame size
    line_end: Point,   // percentage of frame size
    line_width: f32,   // percentage of min(frame.width, frame.height)
    cursor_pos: Point, // position of the cursor in relative frame coordinates when clicked
    is_resizing: bool, // user keeps the left mouse button pressed
}

/**
 * This function checks if the given point is on the line defined by line_start and line_end within the given tolerance.
 * Note that this only works for horizontal lines.
 */
fn is_point_on_horizontal_line(
    pt: &Point,
    line_start: &Point,
    line_end: &Point,
    tolerance: f32,
) -> bool {
    if (line_start.y - line_end.y).abs() > 0.001 {
        println!("is_point_on_line: only horizontal lines are supported currently.");
    }

    if pt.x < line_start.x.min(line_end.x) || pt.x > line_start.x.max(line_end.x) {
        return false;
    }
    if pt.y < line_start.y.min(line_end.y) - tolerance
        || pt.y > line_start.y.max(line_end.y) + tolerance
    {
        return false;
    }
    true
}

fn rel_to_abs_pt(frame: &Frame, rel_point: &Point) -> Point {
    Point::new(rel_point.x * frame.width(), rel_point.y * frame.height())
}

#[allow(dead_code)]
fn rel_to_abs_rct(frame: &Frame, rel_rect: &Rectangle) -> Rectangle {
    Rectangle {
        x: rel_rect.x * frame.width(),
        y: rel_rect.y * frame.height(),
        width: rel_rect.width * frame.width(),
        height: rel_rect.height * frame.height(),
    }
}

impl Default for CicleAndLineState {
    fn default() -> Self {
        Self {
            circle_radius: 0.25, // 25% frame size
            line_start: Point::new(0.10, 0.40),
            line_end: Point::new(0.90, 0.40),
            line_width: 0.20,
            // line_rotate: 0.0, // degrees
            cursor_pos: Point::new(0.0, 0.0),
            is_resizing: false,
        }
    }
}

impl<Message> Program<Message> for CircleAndLineProgram {
    type State = CicleAndLineState;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        // state.line_rotate = self.rotation_angle;
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (iced::widget::canvas::event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(ButtonPressed(mouse::Button::Left)) => {
                let center_point = Point::new(bounds.width / 2.0, bounds.height / 2.0);
                println!("bounds: {:?} and center {:?}", bounds, bounds.center());
                let rotated_cursor_position =
                    rotate_point(&cursor_position, &center_point, &-self.rotation_angle);
                let rel_cursor_position = Point::new(
                    rotated_cursor_position.x / bounds.width,
                    rotated_cursor_position.y / bounds.height,
                );

                state.cursor_pos = rel_cursor_position;

                if is_point_on_horizontal_line(
                    &rel_cursor_position,
                    &state.line_start,
                    &state.line_end,
                    state.line_width / 2.0,
                ) {
                    println!("Clicked on the line!");
                } else {
                    println!("Clicked outside the line!");
                }
                state.is_resizing = true;
                //state.line_width = 0.6;
                println!("Canvas clicked at position: {:?}", cursor_position);
            }
            Event::Mouse(ButtonReleased(mouse::Button::Left)) => {
                state.is_resizing = false;
                //state.line_width = 0.2;
                println!("Canvas release at position: {:?}", cursor_position);
            }
            _ => {}
        }

        // No interaction implemented yet
        (iced::widget::canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        state: &Self::State,
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
            &Path::circle(frame.center(), frame_min * state.circle_radius),
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
            frame.width() * state.line_start.x,
            frame.height() * state.line_start.y,
        );

        let end_point = Point::new(
            frame.width() * state.line_end.x,
            frame.height() * state.line_end.y,
        );

        frame.stroke(
            &Path::line(
                rotate_point(&start_point, &frame.center(), &self.rotation_angle),
                rotate_point(&end_point, &frame.center(), &self.rotation_angle),
                //start_point,
                //end_point,
            ),
            Stroke {
                //style: Color::WHITE.into(),
                style: Style::Gradient(gradient.into()),
                width: frame_min * state.line_width,
                ..Default::default()
            },
        );

        frame.stroke(
            &Path::line(start_point, end_point),
            Stroke {
                style: Color::BLACK.into(),
                width: frame_min * state.line_width,
                ..Default::default()
            },
        );

        frame.fill(
            &Path::circle(rel_to_abs_pt(&frame, &state.cursor_pos), 10.0),
            Color::from_rgb(1.0, 0.0, 0.0),
        );

        vec![frame.into_geometry()]
    }
}
