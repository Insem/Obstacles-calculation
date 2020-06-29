use super::digitRange::*;
use super::line::Line;
use super::point::Point;

pub fn startCoordinatesLine(l: &Line) -> Line {
    Line::new(
        Point::new(0.0, 0.0),
        Point::new(
            l.B_point.x + antiNum(l.A_point.x),
            l.B_point.y + antiNum(l.A_point.y),
        ),
        l.wise,
    )
}
pub fn startCoordinatesPoint(A: &Point, P: &Point) -> Point {
    Point::new(P.x + antiNum(A.x), P.y + antiNum(A.y))
}

pub fn detectPoint(line: &Line, startPoint: &Point) -> bool {
    let zeroLine: Line = startCoordinatesLine(&line);
    let point: Point = startCoordinatesPoint(&line.A_point, &startPoint);
    let planeAngle: f32;
    if !(zeroLine.B_point.x == 0.0_f32 && zeroLine.B_point.y != 0.0_f32) {
        let planeAtan: f32 = (zeroLine.B_point.y / zeroLine.B_point.x).atan();
        if planeAtan < 0.0_f32 {
            planeAngle = planeAtan + std::f32::consts::PI
        } else {
            planeAngle = planeAtan
        };
    } else {
        if zeroLine.B_point.y > 0.0_f32 {
            planeAngle = std::f32::consts::PI / 2_f32;
        } else {
            planeAngle = (std::f32::consts::PI * 3_f32) / 2_f32;
        }
    }
    let findAngle: f32;
    if !(point.x == 0.0_f32 && point.y != 0.0_f32) {
        let findAtan: f32 = (point.y / point.x).atan();
        if point.y >= 0.0_f32 {
            println!("findAtan: {:?}", findAtan);
            if findAtan < 0.0_f32 {
                findAngle = findAtan + std::f32::consts::PI
            } else {
                findAngle = findAtan
            }
        } else {
            if findAtan < 0.0_f32 {
                findAngle = findAtan + (std::f32::consts::PI * 2.0_f32)
            } else {
                println!("zero");
                findAngle = findAtan + std::f32::consts::PI
            }
        };
    } else {
        if point.y > 0.0_f32 {
            findAngle = std::f32::consts::PI / 2_f32;
        } else {
            findAngle = (std::f32::consts::PI * 3_f32) / 2_f32;
        }
    }
    /*println!(
        "{:?}, {:?}, {:?}, {:?}",
        (line.B_point.y / line.B_point.x),
        planeAtan,
        line.B_point.y,
        line.B_point.x
    );*/
    // пофиксить баг где x == 0

    //println!("point: {:?}, {:?}", point, zeroLine);
    /*println!(
        "Angles: {:?}, {:?}, {:?}, {:?}",
        toDegrees(planeAtan),
        toDegrees(planeAngle),
        toDegrees(findAtan),
        toDegrees(findAngle)
    );*/
    match line.wise {
        Wise::right => MathRange::containRanges(&Wise::getRight(planeAngle), findAngle),
        Wise::left => MathRange::containRanges(&Wise::getLeft(planeAngle), findAngle),
    }
}
pub fn antiNum(num: f32) -> f32 {
    num * -1.0
}
fn toDegrees(r: f32) -> f32 {
    (r * 180_f32) / std::f32::consts::PI
}
