use iced::{
    Alignment, Color, Length, Point, Rectangle, Renderer, Subscription, Theme, Vector, mouse, widget::{
        Canvas, MouseArea, button,
        canvas::{Event, Frame, Geometry, Path, Program, Stroke, Style, gradient::Linear},
        column, text,
    }
};

use iced::mouse::Event::ButtonPressed;

use std::time::Duration;

fn main() -> iced::Result {
    //iced::run("My App", MyApp::update, MyApp::view)
    iced::application("iced svg polygon", MyApp::update, MyApp::view)
        .subscription(MyApp::subscription)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    AreaClicked,
    RotateStop,
    Tick,
}

#[derive(Debug, Default)]
struct MyApp {
    rotating: bool,
    rotation_angle: f32,
    mouse_state_text: String,
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
                self.rotation_angle = (self.rotation_angle + 0.5) % 360.0;
            }
            Message::AreaClicked => {} //todo!(),
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
                Canvas::new(CircleAndRectangleProgram::new(self.rotation_angle))
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .on_press(Message::AreaClicked),
        ]
        .align_x(Alignment::Center)
        .into()
    }
}

/**
 *  size values are percentage of frame size.
 *  the rectangle center is in the middle of the frame the view will rotate it around this center point.
 *  the circle is centered in the middle of the frame. It won't rotate. Maybe later when we fill it with some color pattern.
 */
#[derive(Debug)]
struct CircleAndRectangleProgram {
    circle_radius: f32,
    rect_width: f32,
    rect_height: f32,
    rect_rotate: f32,
    is_resizing : bool,
}

impl Default for CircleAndRectangleProgram {
    fn default() -> Self {
        Self {
            circle_radius: 0.25, // 25% frame size
            rect_width: 0.20,
            rect_height: 0.80,
            rect_rotate: 0.0, // degrees
            is_resizing : false,
        }
    }
}

impl CircleAndRectangleProgram {
    fn new(rotation: f32) -> Self {
        Self {
            circle_radius: 0.25,
            rect_width: 0.20, // percentage of the min(frame.width, frame.height)
            rect_height: 0.80, // percentage of the min(frame.width, frame.height)
            rect_rotate: rotation, // degrees 0 - 359
            is_resizing: false, // true if the rectangle is being resized
        }
    }

    fn set_rectangle_rotation(&mut self, angle: f32) {
        self.rect_rotate = angle;
    }    
}

// Rotate a point around a center by a given angle in degrees 0 - 359
fn rotate_point(point: &Point, center: &Point, angle_degrees: &f32) -> Point {
    let angle_radians = angle_degrees.to_radians();
    let sin_angle = angle_radians.sin();
    let cos_angle = angle_radians.cos();

    let translated_x = point.x - center.x;
    let translated_y = point.y - center.y;

    let rotated_x = translated_x * cos_angle - translated_y * sin_angle;
    let rotated_y = translated_x * sin_angle + translated_y * cos_angle;

    Point::new(rotated_x + center.x, rotated_y + center.y)
}

fn rotate_rectangle_corners_at_center(
    center: &Point,
    width: f32,
    height: f32,
    angle_degrees: f32,
) -> [Point; 4] {
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let top_left = Point::new(center.x - half_width, center.y - half_height);
    let top_right = Point::new(center.x + half_width, center.y - half_height);
    let bottom_right = Point::new(center.x + half_width, center.y + half_height);
    let bottom_left = Point::new(center.x - half_width, center.y + half_height);

    [
        rotate_point(&top_left, center, &angle_degrees),
        rotate_point(&top_right, center, &angle_degrees),
        rotate_point(&bottom_right, center, &angle_degrees),
        rotate_point(&bottom_left, center, &angle_degrees),
    ]
}

fn rotate_rectangle(
    rectangle : &Rectangle,    
    center: &Point,
    angle_degrees: f32,
) -> [Point; 4] {
    let top_left = Point::new(rectangle.x, rectangle.y);
    let top_right = Point::new(rectangle.x+rectangle.width, rectangle.y);
    let bottom_right = Point::new(rectangle.x, rectangle.y+rectangle.height);
    let bottom_left = Point::new(rectangle.x+rectangle.width, rectangle.y+rectangle.height);

    [
        rotate_point(&top_left, center, &angle_degrees),
        rotate_point(&top_right, center, &angle_degrees),
        rotate_point(&bottom_right, center, &angle_degrees),
        rotate_point(&bottom_left, center, &angle_degrees),
    ]
}


impl<Message> Program<Message> for CircleAndRectangleProgram {
    type State = ();

    fn update(
            &self,
            _state: &mut Self::State,
            event: Event,
            bounds: Rectangle,
            cursor: mouse::Cursor,
        ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        
        let cursor_position = 
            if let Some(position) = cursor.position_in(bounds) {
                position
            } else {
                return (iced::widget::canvas::event::Status::Ignored, None);
            };

        match event {
            Event::Mouse(ButtonPressed(mouse::Button::Left)) => {
            //mouse::Event::ButtonPressed(mouse::Button::Left) => {
                // Example interaction: print cursor position on left click
                println!("Canvas clicked at position: {:?}", cursor_position);
                if 
            }
            //mouse::Event::ButtonReleased(mouse::Button::Left) => {
            //    // Handle button release if needed
            //    println!("Canvas release at position: {:?}", cursor_position);
            //}
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

        let height_vector = Vector::new(
            (frame_min * self.rect_height / 2.0) * self.rect_rotate.to_radians().cos(),
            (frame_min * self.rect_height / 2.0) * self.rect_rotate.to_radians().sin(),
        );

        frame.stroke(
            &Path::line(
                // frame.center() + Vector::new(-250.0, 100.0),
                // frame.center() + Vector::new(250.0, -100.0),
                frame.center() - height_vector,
                // frame.center() + Vector::new(250.0, -100.0),
                frame.center() + height_vector,
            ),
            Stroke {
                //style: Color::WHITE.into(),
                style: Style::Gradient(gradient.into()),
                width: frame_min * self.rect_width,
                ..Default::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
