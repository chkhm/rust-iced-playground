use iced::Element;
use iced::mouse;
use iced::widget::canvas;
use iced::{Color, Rectangle, Renderer, Theme};

pub fn main() -> iced::Result {
    iced::application("iced canvas polygon", State::update, State::view)
        //.subscription(State::subscription)
        .run()
}

#[derive(Debug)]
struct State {
    edges: u32,
}

impl Default for State {
    fn default() -> Self {
        Self { edges: 5 }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EdgesChanged(u32),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::EdgesChanged(edges) => {
                self.edges = edges.max(3); // Minimum 3 edges for a polygon
            }
        }
    }

    fn view<'a>(&'a self) -> Element<'a, Message> {
        //canvas(Circle { radius: 50.0 }).into()
        view(self)
    }
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

// Finally, we simply use our `Circle` to create the `Canvas`!
fn view<'a, Message: 'a>(_state: &'a State) -> Element<'a, Message> {
    canvas(Circle { radius: 50.0 }).into()
}
