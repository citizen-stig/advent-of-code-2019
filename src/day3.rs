#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y, z:0 }
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

}

#[derive(Debug, PartialEq, Eq)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn new(start: &Point, end: &Point) -> Segment {
        Segment {
            start: Point::new(start.x, start.y),
            end: Point::new(end.x, end.y),
        }
    }
    fn new_from_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Segment {
        Segment {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }

    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
    pub fn get_intersection(&self, other: &Segment) -> Option<Point> {

        let (x1, y1, x2, y2) = (self.start.x, self.start.y, self.end.x, self.end.y);
        let (x3, y3, x4, y4) = (other.start.x, other.start.y, other.end.x, other.end.y);

        let determinant = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        match determinant {
//            0 => None,
//            _ => {
//                let t = ( (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4) ) / determinant;
//                let u = ( (x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3) ) / determinant;
//                if t >= 0 && t <=1 && u >= 0 && u <= 1 {
//                    Some(
//                        Point::new(
//                            x3 + t * (x2 - x1),
//                            y1 + t * (y2 - y1)
//                        )
//                    )
//                } else {
//                    None
//                }
//            }
//        }
    }
}

enum Direction {
    R,
    D,
    L,
    U,
}

struct Path {
    direction: Direction,
    distance: i32,
}


fn parse_wire(input: &str) -> Vec<Segment> {
    let mut start_point = Point::new(0, 0);
    let lines: Vec<Segment> = input
        .split(',')
        .map(|raw_path| -> Path {
            let direction = match raw_path.get(0..1) {
                Some("R") => Direction::R,
                Some("D") => Direction::D,
                Some("L") => Direction::L,
                Some("U") => Direction::U,
                _ => panic!("Unknown direction at the path {}", raw_path)
            };
            let distance = raw_path.get(1..).unwrap().parse::<i32>().unwrap();
            Path { direction, distance }
        })
        .map(|path: Path| -> Segment {
            let end_point = match path.direction {
                Direction::R => {
                    Point::new(start_point.x + path.distance, start_point.y)
                }
                Direction::D => {
                    Point::new(start_point.x, start_point.y - path.distance)
                }
                Direction::L => {
                    Point::new(start_point.x - path.distance, start_point.y)
                }
                Direction::U => {
                    Point::new(start_point.x, start_point.y + path.distance)
                }
            };
            let line = Segment::new(&start_point, &end_point);
            start_point = end_point;
            line
        })
        .collect();
    lines
}

fn find_distance_to_nearest_intersection(wire_1: Vec<Segment>, wire_2: Vec<Segment>) -> i32 {
    let central_port = Point::new(0, 0);
    let mut min_distance: Option<i32> = None;

    for segment_1 in &wire_1 {
        for segment_2 in &wire_2 {
            match segment_1.get_intersection(segment_2) {
                Some(point) => {
                    let distance = central_port.manhattan_distance(&point);
                    if distance > 0 {
                        let actual_min_distance = match min_distance {
                            Some(actual_min_distance) => actual_min_distance,
                            None => distance
                        };
                        if distance <= actual_min_distance {
                            min_distance = Some(distance);
                        }
                    }
                },
                None => ()
            }
        }
    }

    min_distance.unwrap()
}

pub fn solve() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_point {
        use super::*;

        #[test]
        fn test_self() {
            let a = Point::new(1, 1);
            assert_eq!(a.manhattan_distance(&a), 0);
        }

        #[test]
        fn test_from_zero() {
            let a = Point::new(0, 0);
            let b = Point::new(3, 3);
            assert_eq!(a.manhattan_distance(&b), 6);
            assert_eq!(b.manhattan_distance(&a), 6);
        }

        #[test]
        fn test_with_negative_x() {
            let a = Point::new(-2, 0);
            let b = Point::new(3, 3);
            assert_eq!(a.manhattan_distance(&b), 8);
            assert_eq!(b.manhattan_distance(&a), 8);
        }

        #[test]
        fn test_with_negative_y() {
            let a = Point::new(0, -4);
            let b = Point::new(3, 3);
            assert_eq!(a.manhattan_distance(&b), 10);
            assert_eq!(b.manhattan_distance(&a), 10);
        }

        #[test]
        fn test_with_negative_x_y() {
            let a = Point::new(-3, -4);
            let b = Point::new(3, 3);
            assert_eq!(a.manhattan_distance(&b), 13);
            assert_eq!(b.manhattan_distance(&a), 13);
        }
    }

    mod test_line {
        use super::*;

        #[test]
        fn test_intersection_simple() {
            let segment_1 = Segment::new_from_coordinates(-2, 0, 2, 0);
            let segment_2 = Segment::new_from_coordinates(0, -2, 0, 2);
            let expected_point = Some(Point::new(0, 0));
            assert_eq!(segment_1.get_intersection(&segment_2), expected_point);
            assert_eq!(segment_2.get_intersection(&segment_1), expected_point);

        }

        #[test]
        fn test_self() {
            let segment = Segment::new_from_coordinates(1, 3, 4, 3);
            assert_eq!(segment.get_intersection(&segment), None);
        }

        #[test]
        fn test_parallel() {
            let segment_1 = Segment::new_from_coordinates(0, 0, 3, 0);
            let segment_2 = Segment::new_from_coordinates(0, 3, 3, 3);

            assert_eq!(segment_1.get_intersection(&segment_2), None);
            assert_eq!(segment_2.get_intersection(&segment_1), None);
        }

        #[test]
        fn test_collinear() {
            let segment_1 = Segment::new_from_coordinates(0, 0, 3, 0);
            let segment_2 = Segment::new_from_coordinates(4, 0, 6, 0);

            assert_eq!(segment_1.get_intersection(&segment_2), None);
            assert_eq!(segment_2.get_intersection(&segment_1), None);

        }

        #[test]
        fn test_no_intersection() {
            let segment_1 = Segment::new_from_coordinates(0, 0, 7, 0);
            let segment_2 = Segment::new_from_coordinates(6, 7, 6, 3);

            assert_eq!(segment_1.get_intersection(&segment_2), None);
            assert_eq!(segment_2.get_intersection(&segment_1), None);
        }

        #[test]
        fn test_no_intersection_2() {
            let segment_1 = Segment::new_from_coordinates(8, 0, 8, 5);
            let segment_2 = Segment::new_from_coordinates(6, 3, 2, 3);

            assert_eq!(segment_1.get_intersection(&segment_2), None);
            assert_eq!(segment_2.get_intersection(&segment_1), None);
        }
    }

    mod test_parse_wire {
        use super::*;

        #[test]
        fn test_wire_1() {
            let input = "R8,U5,L5,D3";
            let expected_output = vec![
                Segment::new_from_coordinates(0, 0, 8, 0),
                Segment::new_from_coordinates(8, 0, 8, 5),
                Segment::new_from_coordinates(8, 5, 3, 5),
                Segment::new_from_coordinates(3, 5, 3, 2)
            ];

            assert_eq!(parse_wire(input), expected_output);
        }

        #[test]
        fn test_wire_2() {
            let input = "U7,R6,D4,L4";
            let expected_output = vec![
                Segment::new_from_coordinates(0, 0, 0, 7),
                Segment::new_from_coordinates(0, 7, 6, 7),
                Segment::new_from_coordinates(6, 7, 6, 3),
                Segment::new_from_coordinates(6, 3, 2, 3)
            ];

            assert_eq!(parse_wire(input), expected_output);
        }
    }

    mod find_distance_to_nearest_intersection {
        use super::*;

        #[test]
        fn test_simple() {
            let wire_1 = parse_wire("R8,U5,L5,D3");
            let wire_2 = parse_wire("U7,R6,D4,L4");

            let actual_distance = find_distance_to_nearest_intersection(wire_1, wire_2);

            assert_eq!(actual_distance, 6);
        }
    }
}