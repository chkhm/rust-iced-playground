use iced::{Point, Rectangle};

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
