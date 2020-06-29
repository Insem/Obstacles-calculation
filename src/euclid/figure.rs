use super::line::Line;
use super::point::Point;
use super::scale::detectPoint;

pub struct Figure {
    lines: std::vec::Vec<Line>,
}

impl Figure {
    pub fn new(lineArr: std::vec::Vec<Line>) -> Figure {
        if checkLines(&lineArr) {
            Figure { lines: lineArr }
        } else {
            println!("Error");
            //obrabotat oshibku
            Figure { lines: vec![] }
        }
    }

    pub fn checkPoint(&self, point: Point) -> bool {
        let i: i8 = 0;
        for line in &self.lines {
            if !detectPoint(line, &point) {
                println!("Точка не подходит для линии {:?}, {:?}", point, line);
                return false;
            }
        }
        true
    }
}

fn checkLines(lineArr: &std::vec::Vec<Line>) -> bool {
    let i: usize = 0;
    while i < lineArr.len() {
        if i == 0 {
            break;
        }
        if lineArr[i].A_point != lineArr[i - 1].B_point {
            println!(
                "Линии не соприкасаются {:?}, {:?} , {:?}",
                lineArr[i].A_point,
                lineArr[i - 1].B_point,
                i
            );
            return false;
        }
    }
    true
}
