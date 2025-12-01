use crate::message::Message;
use crate::shape_rectangle::{RectangleShape, RectangleState};
use iced::widget::canvas::path::{Builder, Path};
use iced::widget::canvas::{self, Cache, Event, Frame, Geometry, Program, stroke};
use iced::{Point, Rectangle, Renderer, Theme, Vector, mouse};

pub struct CanvasProgram {
    _canvas_cache: Cache,
    // canvas_programs: Vec<Box<dyn Program<Message, State = ()>>>,
}

impl Default for CanvasProgram {
    fn default() -> Self {
        Self {
            _canvas_cache: Cache::new(),
            // canvas_programs: Vec::new(),
        }
    }
}

impl CanvasProgram {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CanvasState {
    pan_zoom_state: PanZoomState,
    rectangle_shape: RectangleShape,
    rectangle_state: RectangleState,
}

impl Program<Message> for CanvasProgram {
    type State = CanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: iced::widget::canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        // Get the absolute cursor position - return if not available
        let cursor_position = if let Some(position) = cursor.position_in(bounds) {
            position - state.pan_zoom_state.translation // * state.pan_zoom_state.scale
        } else {
            return (iced::widget::canvas::event::Status::Ignored, None);
        };

        let cursor_position = Point::new(
            cursor_position.x / state.pan_zoom_state.scale,
            cursor_position.y / state.pan_zoom_state.scale,
        );

        // Step 1: first check if one of the shapes captures the event
        // t.b.d
        let is_handled = state.rectangle_shape.update(
            &mut state.rectangle_state,
            event.clone(),
            cursor_position,
        );
        if is_handled {
            println!("rect handled");
            let message = Message::CanvasMouseMoved(cursor_position);
            return (iced::widget::canvas::event::Status::Captured, Some(message));
        }

        // Step 2: check if PanZoomState handles it
        let pan_zoom_handle_result = state
            .pan_zoom_state
            .handle_message(event.clone(), cursor_position);
        if pan_zoom_handle_result {
            let message = Message::CanvasMouseMoved(cursor_position);
            return (iced::widget::canvas::event::Status::Captured, Some(message));
        }
        let message = Message::CanvasMouseMoved(cursor_position);
        (iced::widget::canvas::event::Status::Ignored, Some(message))
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.translate(state.pan_zoom_state.translation);
        frame.scale(state.pan_zoom_state.scale);

        draw_grid(&mut frame, bounds);

        state.rectangle_shape.draw(
            // &mut self.canvas_cache,
            &state.rectangle_state,
            &mut frame,
            // renderer,
            theme,
            // bounds,
            cursor,
        );
        vec![frame.into_geometry()]
    }
}

// Helper function to draw a simple grid
fn draw_grid(frame: &mut Frame, bounds: Rectangle) {
    let min_x = (bounds.x / 50.0).floor() * 50.0;
    let max_x = (bounds.x + bounds.width / 50.0).ceil() * 50.0;
    let min_y = (bounds.y / 50.0).floor() * 50.0;
    let max_y = (bounds.y + bounds.height / 50.0).ceil() * 50.0;

    let grid = Path::new(|path_builder: &mut Builder| {
        for x in (min_x as i32..max_x as i32).step_by(50) {
            path_builder.move_to(Point::new(x as f32, bounds.y));
            path_builder.line_to(Point::new(x as f32, bounds.y + bounds.height));
        }
        for y in (min_y as i32..max_y as i32).step_by(50) {
            path_builder.move_to(Point::new(bounds.x, y as f32));
            path_builder.line_to(Point::new(bounds.x + bounds.width, y as f32));
        }
    });

    frame.stroke(&grid, stroke::Stroke::default());
}

#[derive(Debug, Clone, Copy)]
struct PanZoomState {
    translation: Vector,
    scale: f32,
    /// The point in screen coordinates where dragging started
    drag_start: Option<Point>,
}

impl Default for PanZoomState {
    fn default() -> Self {
        Self {
            translation: Vector::new(0.0, 0.0),
            scale: 1.0,
            drag_start: None,
        }
    }
}

// --- PanZoom State Logic (Event Handling) ---
impl PanZoomState {
    /// Handles a canvas event and returns true if the canvas view was modified
    fn handle_message(
        &mut self,
        event: iced::widget::canvas::Event,
        cursor_position: Point,
    ) -> bool {
        // we might add more patterns in the future so kill clippy
        #[allow(clippy::single_match)]
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        // Start dragging if cursor is in bounds
                        println!("CanvasMousePressed at {:?}", cursor_position);
                        self.drag_start = Some(cursor_position);
                        true // Mark as handled to capture mouse
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        // Stop dragging
                        println!("CanvaseMouseReleased");
                        self.drag_start = None;
                        true // Mark as handled to capture mouse
                    }
                    mouse::Event::CursorMoved { .. } => {
                        // Continue dragging
                        if let Some(start_pos) = self.drag_start {
                            let delta = cursor_position - start_pos;
                            self.translation = self.translation + delta;
                            self.drag_start = Some(cursor_position); // Update start position for smooth dragging
                            println!(
                                "CanvasMouseMoved to {:?}, translation now {:?}",
                                cursor_position, self.translation
                            );
                            return true; // Mark as handled to capture mouse
                        }
                        false // Mark as not handled
                    }
                    mouse::Event::WheelScrolled { delta } => {
                        match delta {
                            mouse::ScrollDelta::Lines { x, y } => {
                                println!("Lines Scrolled - x:{} y:{}", x, y);
                            }
                            mouse::ScrollDelta::Pixels { x, y } => {
                                let scale_change = if y > 0.0 { 1.1 } else { 0.9 }; // zom by 10% in or out
                                let new_scale = (self.scale * scale_change).clamp(0.1, 10.0); // Constrain zoom levels
                                if self.scale != new_scale {
                                    // Zoom relative to the cursor position (if available)
                                    // This math ensures the point under the cursor stays under the cursor after scaling/translation
                                    let zoom_factor = new_scale / self.scale;
                                    // Adjust translation based on cursor position
                                    let pos_as_vec =
                                        Vector::new(cursor_position.x, cursor_position.y);
                                    self.translation =
                                        pos_as_vec + (self.translation - pos_as_vec) * zoom_factor;
                                    self.scale = new_scale;
                                    println!(
                                        "Pixels Scrolled - x:{} y:{} scale: {}",
                                        x, y, self.scale
                                    );
                                    return true;
                                }
                            }
                        }
                        false
                    }
                    _ => false,
                };
            }
            _ => {}
        }
        false
    }
}
