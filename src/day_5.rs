pub fn begin(part: u8, input: String) -> u32{

    let lines = generate_lines(input);

    match part {
        1 => return part_1::begin(&lines),
        //2 => return part_2::begin(&lines),
        _ => panic!("Invalid part entered.")
    }
}

fn generate_lines(input: String) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut points = line.split("->")
                .map(|point| {
                    let mut nums = point.split(",");
                    Point::new(nums.next().unwrap().trim().parse::<u32>().unwrap(), nums.next().unwrap().trim().parse::<u32>().unwrap())
                });
            Line::new(points.next().unwrap(), points.next().unwrap())
        })
        .collect::<Vec<Line>>()
}

pub struct Line {
    p1: Point,
    p2: Point,
    crosses: Vec<Point>
}
impl Line{
    fn new(p1: Point, p2: Point) -> Line {
        println!("Calculating points in line from {} -> {}", p1.print_point(), p2.print_point());
        let mut crosses = Vec::new();
        if p1.x == p2.x && p1.y == p2.y {
            crosses.push(Point::new(p1.x, p1.y));
        }else if p1.x == p2.x { 
            let iter;
            if p1.y < p2.y {
                iter = (p1.y)..(p2.y + 1);
            }
            else {
                iter = (p2.y)..(p1.y + 1);
            }
            for i in iter {
                let new_pt = Point::new(p1.x, i);
                crosses.push(new_pt);
            }
        }else if (p1.y == p2.y) {
            let iter;
            if p1.x < p2.x {
                iter = (p1.x)..(p2.x + 1);
            }
            else {
                iter = (p2.x)..(p1.x + 1);
            }
            for i in iter {
                let new_pt = Point::new(i, p1.y);
                crosses.push(new_pt);
            }
        }else{
            crosses = vec![];
        }
        Line { p1, p2, crosses }
    }
    fn find_intersection_pts(&self, other: &Line) -> Vec<&Point>{
        self.crosses
            .iter()
            .filter(|x| other.crosses.contains(x))
            .collect()
    }
}

struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {x, y}
    }
    fn print_point(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

mod part_1 { 

    use std::collections::HashMap;
    use super::Line as Line;
    use super::Point as Point;

    pub fn begin(lines: &Vec<Line>) -> u32 {

        /* WAY TOO EXPENSIVE
        let mut points_found = 0;
        for i in 0..999 {
            for j in 0..999 {
                let point = Point::new(i, j);
                println!("Intersects: {}, Checking Point: {}", points_found, point.print_point());
                let mut count = 0;
                for line in lines {
                    if line.crosses.contains(&point) {
                        count += 1;
                        if count == 2 {
                            points_found += 1;
                            break;
                        }
                    }
                }
            }
        }

        points_found
        */

        let mut result = HashMap::new();
        let mut greatest = 0;
        for (index_1, line) in lines.iter().enumerate() {
            for (index_2, other_line) in lines.iter().enumerate() {
                if index_1 * index_2 / 2500 > greatest { greatest = index_1 * index_2 / 2500 }
                println!("{} Intersects Found, {}% Done", result.len(), greatest);
                if index_1 != index_2 {
                    let intersecs = line.find_intersection_pts(other_line);
                    for i in intersecs {
                        let entry = result.entry(i.print_point()).or_insert(0);
                    }
                }

            }
        }
        result.len() as u32
    }
}

mod part_2 {
    pub fn begin(input: String) -> u32 {
        0
    }
}

//some tests i used for TDD (test-driven development)
#[cfg(test)]
mod test {
    use super::Line as Line;
    use super::Point as Point;
    
    #[test]
    fn same_y() {
        let pt1 = Point::new(48, 2);
        let pt2 = Point::new(1, 2);
        let ln1 = Line::new(&pt1, &pt2);
        assert_eq!(ln1.crosses.len(), 48);
    }
    #[test]
    fn same_x() {
        let pt1 = Point::new(1, 7);
        let pt2 = Point::new(1, 347);
        let ln1 = Line::new(&pt1, &pt2);
        assert_eq!(ln1.crosses.len(), 341);
    }
    #[test]
    fn same_point() {
        let pt1 = Point::new(2, 2);
        let pt2 = Point::new(2, 2);
        let ln1 = Line::new(&pt1, &pt2);
        assert_eq!(ln1.crosses.len(), 1);
    }
    #[test]
    fn find_3_overlaps() {
        let pt1 = Point::new(0, 9);
        let pt2 = Point::new(5, 9);
        let pt3 = Point::new(0, 9);
        let pt4 = Point::new(2, 9);
        let ln1 = Line::new(&pt1, &pt2);
        let ln2 = Line::new(&pt3, &pt4);
        assert_eq!(ln1.find_intersection_pts(&ln2).len(), 3);
    }

}
