use sdl2::rect::{Point, Rect};

/// Checks if a rectangle contains another rectangle
pub fn rect_contains_rect(parent: Rect, child: Rect) -> bool {
    let x_min = child.x();
    let x_max = x_min + child.width() as i32;
    let y_min = child.y();
    let y_max = y_min + child.height() as i32;

    let check_xmin = x_min >= parent.x() && x_min <= parent.x() + parent.width() as i32;
    let check_xmax = x_max >= parent.x() && x_max <= parent.x() + parent.width() as i32;
    let check_ymin = y_min >= parent.y() && y_min <= parent.y() + parent.height() as i32;
    let check_ymax = y_max >= parent.y() && y_max <= parent.y() + parent.height() as i32;

    check_xmin && check_xmax && check_ymin && check_ymax
}

/// Checks if a rectangle overlaps another rectangle
pub fn rect_overlaps_rect(first: Rect, second: Rect) -> bool {
    let check_x_first = first.x() < second.x() + second.width() as i32;
    let check_x_second = second.x() < first.x() + first.width() as i32;
    let check_y_first = first.y() < second.y() + second.height() as i32;
    let check_y_second = second.y() < first.y() + first.height() as i32;

    check_x_first && check_x_second && check_y_first && check_y_second
}

/// Checks if a rectangle contains a point
pub fn rect_contains_point(parent: Rect, child: Point) -> bool {
    false
}
