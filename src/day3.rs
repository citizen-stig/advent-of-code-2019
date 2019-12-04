use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
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

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn all_points(&self) -> HashSet<Point> {
        let points: HashSet<Point> = if self.is_vertical() {
            let (start, end) = if self.start.y < self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };
            (start + 1..end).map(|y| Point::new(self.start.x, y)).collect()
        } else {
            let (start, end) = if self.start.x < self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };
            (start + 1..end).map(|x| Point::new(x, self.start.y)).collect()
        };
        points
    }

    pub fn get_intersection(&self, other: &Segment) -> Option<Point> {
        match self.all_points().intersection(&other.all_points()).next() {
            None => None,
            Some(point) => Some(point.clone())
        }
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


// I'm not proud of this solution, it can be done much more efficient using sweep line algorithm
// https://www.youtube.com/watch?v=dePDHVovJlE
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
                }
                None => ()
            }
        }
    }

    min_distance.unwrap()
}

pub fn solve() -> i32 {
    let filename = "input/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let wire_1 = parse_wire(&lines.next().unwrap().expect("cannot read line"));
    let wire_2 = parse_wire(&lines.next().unwrap().expect("cannot read second line"));

    find_distance_to_nearest_intersection(wire_1, wire_2)
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

    mod test_segment {
        use super::*;

        #[test]
        fn test_points_horizontal() {
            let segment = Segment::new_from_coordinates(-2, 0, 2, 0);
            let points = segment.all_points();
            println!("{:?}", points);
        }

        #[test]
        fn test_intersection_simple() {
            let segment_1 = Segment::new_from_coordinates(-2, 0, 2, 0);
            let segment_2 = Segment::new_from_coordinates(0, -2, 0, 2);
            let expected_point = Some(Point::new(0, 0));
            assert_eq!(segment_1.get_intersection(&segment_2), expected_point);
            assert_eq!(segment_2.get_intersection(&segment_1), expected_point);
        }

        #[test]
        #[ignore]
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

        #[test]
        fn test_case_1() {
            let wire_1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
            let wire_2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");

            let actual_distance = find_distance_to_nearest_intersection(wire_1, wire_2);

            assert_eq!(actual_distance, 159);
        }

        #[test]
        fn test_case_2() {
            let wire_1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
            let wire_2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

            let actual_distance = find_distance_to_nearest_intersection(wire_1, wire_2);

            assert_eq!(actual_distance, 135);
        }
    }
}