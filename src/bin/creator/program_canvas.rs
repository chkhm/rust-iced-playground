//use crate::message::Message;
use crate::shape_rectangle::{RectangleShape, RectangleState};
use iced::widget::canvas::{Cache, Event, Geometry, Program};
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

impl<Message> Program<Message> for CanvasProgram {
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
            position
        } else {
            return (iced::widget::canvas::event::Status::Ignored, None);
        };

        // Step 1: first check if one of the shapes captures the event
        // t.b.d
        let is_handled =
            state
                .rectangle_shape
                .update(&mut state.rectangle_state, event.clone(), bounds, cursor);
        if is_handled {
            return (iced::widget::canvas::event::Status::Captured, None);
        }

        // Step 2: check if PanZoomState handles it
        let pan_zoom_handle_result = state
            .pan_zoom_state
            .handle_message(event.clone(), cursor_position);
        if pan_zoom_handle_result {
            return (iced::widget::canvas::event::Status::Captured, None);
        }
        (iced::widget::canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let g: Vec<Geometry> =
            state
                .rectangle_shape
                .draw(&state.rectangle_state, renderer, theme, bounds, cursor);

        g
    }
}

#[derive(Debug, Clone, Copy)]
struct PanZoomState {
    translation: Vector,
    _scale: f32,
    /// The point in screen coordinates where dragging started
    drag_start: Option<Point>,
}

impl Default for PanZoomState {
    fn default() -> Self {
        Self {
            translation: Vector::new(0.0, 0.0),
            _scale: 1.0,
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
                    _ => false,
                };
            }
            _ => {}
        }
        false
        /*
                mouse::Event::WheelScrolled { delta } => {
                    // Zooming
                    let scale_change = if delta.y > 0.0 { 1.1 } else { 0.9 }; // Simple zoom factor
                    let new_scale = (self.scale * scale_change).clamp(0.1, 10.0); // Constrain zoom levels

                    if self.scale != new_scale {
                        // Zoom relative to the cursor position (if available)
                        // This math ensures the point under the cursor stays under the cursor after scaling/translation
                        if let Some(cursor_position) = mouse_event.position() {
                            let zoom_factor = new_scale / self.scale;
                            // Adjust translation based on cursor position
                            self.translation = cursor_position.into()
                                + (self.translation - cursor_position.into()) * zoom_factor;
                        }
                        self.scale = new_scale;
                        return true;
                    }
                }
                _ => {}
            }
        }
        _ => {}
        */
    }
}
