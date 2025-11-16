use iced::{Point, Rectangle, widget::canvas::Frame};

// Rotate a point around a center by a given angle in degrees 0 - 359
#[allow(dead_code)]
pub fn rotate_point(point: &Point, center: &Point, angle_degrees: &f32) -> Point {
    let angle_radians = angle_degrees.to_radians();
    let sin_angle = angle_radians.sin();
    let cos_angle = angle_radians.cos();

    let translated_x = point.x - center.x;
    let translated_y = point.y - center.y;

    let rotated_x = translated_x * cos_angle - translated_y * sin_angle;
    let rotated_y = translated_x * sin_angle + translated_y * cos_angle;

    Point::new(rotated_x + center.x, rotated_y + center.y)
}

/**
 * This function checks if the given point is on the line defined by line_start and line_end within the given tolerance.
 * Note that this only works for horizontal lines.
 */
pub fn is_point_on_horizontal_line(
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

/**
 * This function finds out if the given point is on a corner of the line. It returns the number of the corner
 * (0=upper left, 1=upper right, 2=lower right, 3=lower left) or None if the point is not on a corner.
 */
pub fn is_point_on_line_corner(
    pt: &Point,
    line_start: &Point,
    line_end: &Point,
    width: f32,
) -> Option<usize> {
    let w_half = width / 2.0;
    let corners = [
        Point::new(line_start.x, line_start.y - w_half), // upper left
        Point::new(line_end.x, line_end.y - w_half),     // upper right
        Point::new(line_end.x, line_end.y + w_half),     // lower right
        Point::new(line_start.x, line_start.y + w_half), // lower left
    ];

    for (i, corner) in corners.iter().enumerate() {
        if (pt.x - corner.x).abs() <= 0.01 && (pt.y - corner.y).abs() <= 0.01 {
            return Some(i);
        }
    }
    None
}

pub fn rel_to_abs_pt(frame: &Frame, rel_point: &Point) -> Point {
    Point::new(rel_point.x * frame.width(), rel_point.y * frame.height())
}

#[allow(dead_code)]
pub fn rel_to_abs_rct(frame: &Frame, rel_rect: &Rectangle) -> Rectangle {
    Rectangle {
        x: rel_rect.x * frame.width(),
        y: rel_rect.y * frame.height(),
        width: rel_rect.width * frame.width(),
        height: rel_rect.height * frame.height(),
    }
}

#[allow(dead_code)]
pub fn rotate_line(
    start: &Point,
    end: &Point,
    center: &Point,
    angle_degrees: f32,
) -> (Point, Point) {
    (
        rotate_point(start, center, &angle_degrees),
        rotate_point(end, center, &angle_degrees),
    )
}

#[allow(dead_code)]
pub fn rotate_rectangle_corners_at_center(
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

#[allow(dead_code)]
pub fn rotate_rectangle(rectangle: &Rectangle, center: &Point, angle_degrees: f32) -> [Point; 4] {
    let top_left = Point::new(rectangle.x, rectangle.y);
    let top_right = Point::new(rectangle.x + rectangle.width, rectangle.y);
    let bottom_right = Point::new(rectangle.x, rectangle.y + rectangle.height);
    let bottom_left = Point::new(
        rectangle.x + rectangle.width,
        rectangle.y + rectangle.height,
    );

    [
        rotate_point(&top_left, center, &angle_degrees),
        rotate_point(&top_right, center, &angle_degrees),
        rotate_point(&bottom_right, center, &angle_degrees),
        rotate_point(&bottom_left, center, &angle_degrees),
    ]
}
