use super::point::Point;
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub enum Wise {
    right,
    left,
}

pub struct MathRange {
    start: f32,
    end: f32,
}
impl Wise {
    pub fn getLeft(angle: f32) -> [MathRange; 2] {
        let mut arr: [MathRange; 2] = [
            MathRange::new(0.0_f32, angle),
            MathRange::new(angle + std::f32::consts::PI, 2.0_f32 * std::f32::consts::PI),
        ];
        arr
    }
    pub fn getRight(angle: f32) -> [MathRange; 1] {
        let mut arr: [MathRange; 1] = [MathRange::new(angle, angle + std::f32::consts::PI)];
        arr
    }
}
impl MathRange {
    pub fn new(start: f32, end: f32) -> MathRange {
        MathRange {
            start: start,
            end: end,
        }
    }
    pub fn contain(&self, num: f32) -> bool {
        println!(
            "Start, end, angle: {:?}, {:?}, {:?}, {:?}",
            toDegrees(self.start),
            toDegrees(self.end),
            toDegrees(num),
            (&self.start <= &num) && (&num <= &self.end)
        );
        if (&self.start <= &num) && (&num <= &self.end) {
            true
        } else {
            false
        }
    }
    pub fn containRanges(rangeArr: &[MathRange], num: f32) -> bool {
        let mut check: bool = false;
        for elem in rangeArr.iter() {
            check = check || elem.contain(num)
        }
        check
    }
}

fn toDegrees(r: f32) -> f32 {
    (r * 180_f32) / std::f32::consts::PI
}
