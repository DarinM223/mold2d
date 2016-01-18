use sdl2::rect::Rect;

pub trait Shape<A, B> {
    fn contains(child: A, parent: B) -> bool;

    fn overlaps(first: A, second: B) -> bool;
}

pub trait Rectangle {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

impl Rectangle for Rect {
    fn x(&self) -> i32 {
        self.x()
    }

    fn y(&self) -> i32 {
        self.y()
    }

    fn width(&self) -> u32 {
        self.width()
    }

    fn height(&self) -> u32 {
        self.height()
    }
}

pub struct GeoUtils;

impl<A: Rectangle, B: Rectangle> Shape<A, B> for GeoUtils {
    /// Checks if a rectangle can be contained inside a parent rectangle
    fn contains(parent: A, child: B) -> bool {
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

    /// Checks if a rectangle overlaps with another rectangle
    fn overlaps(first: A, second: B) -> bool {
        let check_x_first = first.x() < second.x() + second.width() as i32;
        let check_x_second = second.x() < first.x() + first.width() as i32;
        let check_y_first = first.y() < second.y() + second.height() as i32;
        let check_y_second = second.y() < first.y() + first.height() as i32;

        check_x_first && check_x_second && check_y_first && check_y_second
    }
}
