mod euclid;
use euclid::digitRange::Wise;
use euclid::figure::Figure;
use euclid::line::Line;
use euclid::point::Point;
use euclid::scale::*;

fn main() {
    let A: Point = Point::new(-4_f32, 3_f32);
    let B: Point = Point::new(6_f32, 5_f32);
    let C: Point = Point::new(-4_f32, -2_f32);
    let D: Point = Point::new(6_f32, -4_f32);
    //______________________________________________
    let Q: Point = Point::new(3_f32, -1_f32); //_____
                                              //______________________________________________
    let AB: Line = Line::new(A, B, Wise::left);
    let BD: Line = Line::new(B, D, Wise::left);
    let DC: Line = Line::new(D, C, Wise::left);
    let CA: Line = Line::new(C, A, Wise::left);

    let lineArr: std::vec::Vec<Line> = vec![AB, BD, DC, CA];
    let trap: Figure = Figure::new(lineArr);
    println!("Попала ли точка {:?}", trap.checkPoint(Q));
}
