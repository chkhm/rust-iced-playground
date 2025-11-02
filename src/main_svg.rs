use iced::widget::{button, center, column, container, horizontal_space, row, slider, svg, text};
use iced::{Element, Fill, Length, Subscription};
use std::f64::consts::PI;
use std::time::Duration;

pub fn main() -> iced::Result {
    iced::application("iced svg polygon", State::update, State::view)
        .subscription(State::subscription)
        .run()
}

#[derive(Debug)]
struct State {
    edges: u32,
    hue: f32,
    saturation: f32,
    brightness: f32,
    rotation: f32,
    playing: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            edges: 5,
            hue: 0.0,
            saturation: 100.0,
            brightness: 100.0,
            rotation: 0.0,
            playing: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EdgesChanged(u32),
    Hue(f32),
    Saturation(f32),
    Brightness(f32),
    Tick,
    PlayPause,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::PlayPause => {
                self.playing = !self.playing;
            }
            Message::EdgesChanged(edges) => {
                self.edges = edges.max(3); // Minimum 3 edges for a polygon
            }
            Message::Hue(hue) => {
                self.hue = hue;
            }
            Message::Saturation(saturation) => {
                self.saturation = saturation;
            }
            Message::Brightness(brightness) => {
                self.brightness = brightness;
            }
            Message::Tick => {
                self.rotation = (self.rotation + 0.5) % 360.0;
            }
        }
    }

    fn generate_polygon_svg(&self) -> String {
        let radius = 100.0;
        let center_x = 150.0;
        let center_y = 150.0;
        let (fill_color, stroke_color) = self.get_colors();

        // Generate points for the polygon
        let points: Vec<(f64, f64)> = (0..self.edges)
            .map(|i| {
                let angle = (i as f64 * 2.0 * PI) / self.edges as f64;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                (x, y)
            })
            .collect();

        // Create SVG path
        let path_data = points
            .iter()
            .enumerate()
            .map(|(i, (x, y))| {
                if i == 0 {
                    format!("M {:.1} {:.1}", x, y)
                } else {
                    format!("L {:.1} {:.1}", x, y)
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Complete SVG with viewBox, path, and rotation transform
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <svg viewBox="0 0 300 300" xmlns="http://www.w3.org/2000/svg">
                <g transform="rotate({} 150 150)">
                    <path d="{} Z" 
                          fill="{}" 
                          stroke="{}" 
                          stroke-width="2"/>
                </g>
            </svg>"#,
            self.rotation, path_data, fill_color, stroke_color
        )
    }

    fn get_colors(&self) -> (String, String) {
        let (r, g, b) = hsb_to_rgb(self.hue, self.saturation, self.brightness);
        let (stroke_r, stroke_g, stroke_b) = hsb_to_rgb(
            self.hue,
            self.saturation,
            self.brightness * 0.7, // Darker stroke
        );

        let fill = format!(
            "rgba({},{},{},0.8)", // 80% opacity for fill
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8
        );

        let stroke = format!(
            "rgb({},{},{})",
            (stroke_r * 255.0) as u8,
            (stroke_g * 255.0) as u8,
            (stroke_b * 255.0) as u8
        );

        (fill, stroke)
    }

    fn view(&self) -> Element<Message> {
        // Generate SVG data
        let svg_data = self.generate_polygon_svg();

        // Create handle from memory
        let handle = svg::Handle::from_memory(svg_data.into_bytes());

        let svg = container(
            svg(handle)
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(300.0)),
        )
        .style(container::rounded_box)
        .center(Fill);

        let edges = slider(3..=12, self.edges, Message::EdgesChanged);

        let edge_controls = container(
            column![
                row![
                    text("Number of edges:").size(20),
                    horizontal_space(),
                    button(text(if !self.playing { "Play" } else { "Pause" }))
                        .on_press(Message::PlayPause),
                ]
                .spacing(5),
                edges,
            ]
            .spacing(10),
        )
        .center_x(Fill);

        let hsb_controls = container(
            column![
                row![text("Color:").size(20),],
                row![
                    text(format!("Hue: {:.1}", self.hue)).size(15).width(Fill),
                    text(format!("Saturation: {:.1}", self.saturation))
                        .size(15)
                        .width(Fill),
                    text(format!("Brightness: {:.1}", self.brightness))
                        .size(15)
                        .width(Fill)
                ]
                .spacing(5),
                row![
                    slider(0.0..=360.0, self.hue, Message::Hue),
                    slider(0.0..=100.0, self.saturation, Message::Saturation),
                    slider(0.0..=100.0, self.brightness, Message::Brightness)
                ]
                .spacing(5)
            ]
            .spacing(10),
        );

        center(
            column![edge_controls, svg, hsb_controls]
                .spacing(20)
                .height(Fill),
        )
        .padding(20)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.playing {
            iced::time::every(Duration::from_millis(10)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

fn hsb_to_rgb(h: f32, s: f32, b: f32) -> (f32, f32, f32) {
    let h = h / 360.0;
    let s = s / 100.0;
    let b = b / 100.0;

    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = b * (1.0 - s);
    let q = b * (1.0 - f * s);
    let t = b * (1.0 - (1.0 - f) * s);

    match (i % 6.0) as i32 {
        0 => (b, t, p),
        1 => (q, b, p),
        2 => (p, b, t),
        3 => (p, q, b),
        4 => (t, p, b),
        _ => (b, p, q),
    }
}
