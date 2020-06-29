use super::digitRange::Wise;
use super::point::Point;

#[derive(Debug)]
pub struct Line {
    pub A_point: Point,
    pub B_point: Point,
    pub wise: Wise,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.A_point == other.A_point && self.B_point == other.B_point
    }
}
impl Line {
    pub fn new(firstPoint: Point, secondPoint: Point, wise: Wise) -> Line {
        Line {
            A_point: firstPoint,
            B_point: secondPoint,
            wise: wise,
        }
    }
}
