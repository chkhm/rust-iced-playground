// use std::os::macos::raw::stat;

use iced::{
    // Alignment,
    Color,
    Point,
    Rectangle,
    Renderer,
    Theme,
    mouse,
    widget::canvas::{Event, Frame, Geometry, Path, Program, Stroke, Style, gradient::Linear},
};

use iced::mouse::Event::ButtonPressed;
use iced::mouse::Event::ButtonReleased;

// use std::time::Duration;

use iced_hello_world::is_point_on_horizontal_line;
use iced_hello_world::is_point_on_line_corner;
use iced_hello_world::rel_to_abs_pt;
use iced_hello_world::rotate_point;

/**
 * This little program draws a circle and a line across the circle. The line starts and ends at the default positions.
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

impl CircleAndLineProgram {
    fn rel_cursor_pos(&self, bounds: &Rectangle, cursor_position: &Point) -> Point {
        let center_point = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let rotated_cursor_position =
            rotate_point(cursor_position, &center_point, &-self.rotation_angle);
        Point::new(
            rotated_cursor_position.x / bounds.width,
            rotated_cursor_position.y / bounds.height,
        )
    }
}

/**
 * Dragging mode enum
 * - None: not dragging
 * - Line: dragging the line
 * - Corner: dragging a corner (number represents the corner number (0 - 3))
 */
#[derive(Debug)]
enum DraggingMode {
    NoDragging,
    Line,
    Corner(usize), // corner index
}

/**
 * holds the state of the program.
 */
#[derive(Debug)]
pub struct CicleAndLineState {
    circle_radius: f32,
    line_start: Point,         // percentage of frame size
    line_end: Point,           // percentage of frame size
    line_width: f32,           // percentage of min(frame.width, frame.height)
    cursor_pos: Point,         // position of the cursor in relative frame coordinates when clicked
    is_dragging: DraggingMode, // user keeps the left mouse button pressed
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
            is_dragging: DraggingMode::NoDragging,
        }
    }
}

impl<Message> Program<Message> for CircleAndLineProgram {
    type State = CicleAndLineState;

    /**
     * Handles mouse events on the canvas.
     *
     * The method first calculates center point and the rotated relative cursor position.
     *
     * It then matches the event:
     * - on left button press it checks if the click was on a corner or on the line and sets is_dragging accordingly
     * - on left button release it resets is_dragging
     * - on cursor move it updates the line position if is_dragging is true
     *
     * Note: Function returns "captured" if the event was handled (mouse click, release, move only if in dragging
     *       mode), "ignored" otherwise.
     */
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        // Get the absolute cursor position - return if not available
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position
        } else {
            return (iced::widget::canvas::event::Status::Ignored, None);
        };

        // --- process events
        match event {
            // --- left button pressed
            Event::Mouse(ButtonPressed(mouse::Button::Left)) => {
                let rel_cursor_position: Point = self.rel_cursor_pos(&bounds, &cursor_position);
                state.cursor_pos = rel_cursor_position;

                // check if the click was on a corner
                if let Some(corner_index) = is_point_on_line_corner(
                    &rel_cursor_position,
                    &state.line_start,
                    &state.line_end,
                    state.line_width,
                ) {
                    println!("Clicked on corner {}", corner_index);
                    state.is_dragging = DraggingMode::Corner(corner_index);
                    return (iced::widget::canvas::event::Status::Captured, None);
                }

                // check if the click was on the line (i.e. inside the line width)
                if is_point_on_horizontal_line(
                    &rel_cursor_position,
                    &state.line_start,
                    &state.line_end,
                    state.line_width / 2.0,
                ) {
                    println!("Clicked on the line!");
                    state.is_dragging = DraggingMode::Line;
                    return (iced::widget::canvas::event::Status::Captured, None);
                } else {
                    println!("Clicked outside the line!");
                }
                println!("Canvas clicked at position: {:?}", cursor_position);
                return (iced::widget::canvas::event::Status::Captured, None);
            }

            // --- left button released
            Event::Mouse(ButtonReleased(mouse::Button::Left)) => {
                state.is_dragging = DraggingMode::NoDragging;
                println!("Canvas release at position: {:?}", cursor_position);
                return (iced::widget::canvas::event::Status::Captured, None);
            }

            // --- cursor moved
            //Event::Mouse(mouse::Event::CursorMoved { position }) => match state.is_dragging {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => match state.is_dragging {
                DraggingMode::Corner(corner_index) => {
                    let rel_cursor_position: Point = self.rel_cursor_pos(&bounds, &cursor_position);
                    let delta = rel_cursor_position - state.cursor_pos;
                    state.cursor_pos = rel_cursor_position;
                    match corner_index {
                        0 => {
                            // upper left
                            state.line_start = Point::new(
                                state.line_start.x + delta.x,
                                state.line_start.y + delta.y / 2.0,
                            );
                            state.line_end = Point::new(state.line_end.x, state.line_start.y); // our rects are all horizontal
                            state.line_width -= delta.y; // this is correct. Center moves by half but width goes in both directions so we go full delta
                        }
                        1 => {
                            // upper right
                            state.line_end = Point::new(
                                state.line_end.x + delta.x,
                                state.line_end.y + delta.y / 2.0,
                            );
                            state.line_start = Point::new(state.line_start.x, state.line_end.y); // our rects are all horizontal
                            state.line_width -= delta.y;
                        }
                        2 => {
                            // lower right
                            state.line_end = Point::new(
                                state.line_end.x + delta.x,
                                state.line_end.y + delta.y / 2.0,
                            );
                            state.line_start = Point::new(state.line_start.x, state.line_end.y); // our rects are all horizontal
                            state.line_width += delta.y;
                        }
                        3 => {
                            // lower left
                            state.line_start = Point::new(
                                state.line_start.x + delta.x,
                                state.line_start.y + delta.y / 2.0,
                            );
                            state.line_end = Point::new(state.line_end.x, state.line_start.y); // our rects are all horizontal
                            state.line_width += delta.y;
                        }
                        _ => {}
                    }
                    return (iced::widget::canvas::event::Status::Captured, None);
                }
                DraggingMode::Line => {
                    let rel_cursor_position: Point = self.rel_cursor_pos(&bounds, &cursor_position);
                    let delta = rel_cursor_position - state.cursor_pos;
                    state.line_start =
                        Point::new(state.line_start.x + delta.x, state.line_start.y + delta.y);
                    state.line_end =
                        Point::new(state.line_end.x + delta.x, state.line_end.y + delta.y);
                    state.cursor_pos = rel_cursor_position;
                    return (iced::widget::canvas::event::Status::Captured, None);
                }
                _ => {}
            },
            _ => {}
        }

        // No events were handled.
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

        // gradient for the rotating rectangle:
        // - we have a diagonal rainbow gradient from red via green to blue
        // - green is a very small segment because the color is so dominant
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

        // calculate line start and end points in absolute coordinates and draw it
        let start_point = Point::new(
            frame.width() * state.line_start.x,
            frame.height() * state.line_start.y,
        );

        let end_point = Point::new(
            frame.width() * state.line_end.x,
            frame.height() * state.line_end.y,
        );

        // draw the rotated line with gradient fill
        frame.stroke(
            &Path::line(
                rotate_point(&start_point, &frame.center(), &self.rotation_angle),
                rotate_point(&end_point, &frame.center(), &self.rotation_angle),
            ),
            Stroke {
                //style: Color::WHITE.into(),
                style: Style::Gradient(gradient.into()),
                width: frame_min * state.line_width,
                ..Default::default()
            },
        );

        // draw the line in non-rotated position for easier debugging
        frame.stroke(
            &Path::line(start_point, end_point),
            Stroke {
                style: Color::BLACK.into(),
                width: frame_min * state.line_width,
                ..Default::default()
            },
        );

        // draw the clicked position as a red dot
        frame.fill(
            &Path::circle(rel_to_abs_pt(&frame, &state.cursor_pos), 10.0),
            Color::from_rgb(1.0, 0.0, 0.0),
        );

        vec![frame.into_geometry()]
    }
}
