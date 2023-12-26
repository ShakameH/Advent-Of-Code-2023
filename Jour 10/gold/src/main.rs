use std::fs;
use std::collections::VecDeque;
use std::f64;

const _EPS: f64 = 0.00001;
const _MIN: f64 = f64::MIN_POSITIVE;
const _MAX: f64 = f64::MAX;

#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
#[derive(Debug)]
struct Edge {
    pt1: Point,
    pt2: Point,
}

impl Edge {
    fn new(pt1: (f64, f64), pt2: (f64, f64)) -> Edge {
        Edge {
            pt1: Point { x: pt1.0, y: pt1.1 },
            pt2: Point { x: pt2.0, y: pt2.1 },
        }
    }
}

#[derive(Debug)]
struct Polygon {
    edges: Vec<Edge>,
}

fn find_starting_position(array: &mut Vec<Vec<(char, i32)>>) -> (usize, usize) {
    let row_len: usize = array[0].len();
    let column_len: usize = array.len();
    for i in 0..column_len {
        for j in 0..row_len {
            if array[i][j].0 == 'S' {
                return (j, i);
            }
        }
    }
    (0, 0)
}

fn path_finder(array: &mut Vec<Vec<(char, i32)>>, array_point: &mut Vec<(usize, usize)>, position: (usize, usize), length: i32) {
    let mut stack = VecDeque::new();
    let max_row = array[0].len();
    let max_col = array.len();
    
    stack.push_back((position, length));

    while let Some(((x, y), len)) = stack.pop_back() {
        if array[y][x].1 == -1 || array[y][x].1 > len + 1 {
            array[y][x].1 = len + 1;
            match array[y][x].0 {
                'S' => {
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '|' => {
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '-' => {
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                }
                'L' => {
                    array_point.push((x, y));
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                }
                'J' => {
                    array_point.push((x, y));
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '7' => {
                    array_point.push((x, y));
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                }
                'F' => {
                    array_point.push((x, y));
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                }
                _ => {},
            }
        }
    }
}

fn pt_in_polygon(pt: &Point, poly: &Polygon) -> bool {
    let count = poly.edges
        .iter()
        .filter(|edge| ray_intersect_seg(pt, edge))
        .count();

    count % 2 == 1
}

fn ray_intersect_seg(p: &Point, edge: &Edge) -> bool {
    let mut pt = p.clone();
    let (mut a, mut b): (&Point, &Point) = (&edge.pt1, &edge.pt2);
    if a.y > b.y {
        std::mem::swap(&mut a, &mut b);
    }
    if pt.y == a.y || pt.y == b.y {
        pt.y += _EPS;
    }
    if (pt.y > b.y || pt.y < a.y) || pt.x > a.x.max(b.x) {
        false
    } else if pt.x < a.x.min(b.x) {
        true
    } else {
        let m_red = if (a.x - b.x).abs() > _MIN {
            (b.y - a.y) / (b.x - a.x)
        } else {
            _MAX
        };
        let m_blue = if (a.x - pt.x).abs() > _MIN {
            (pt.y - a.y) / (pt.x - a.x)
        } else {
            _MAX
        };
        m_blue >= m_red
    }
}


fn main() {
    let mut array: Vec<Vec<(char, i32)>> = vec![];
    let mut array_point: Vec<(usize, usize)> = vec![];
    let mut polygon = Polygon { edges: vec![]};
    let p = |x, y| Point {x, y};
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");

    for line in file_content.lines() {
        let line_vec: Vec<(char,i32)> = line.chars().map(|c| (c, -1)).collect();
        array.push(line_vec);
    }
    let start_position: (usize, usize) = find_starting_position(&mut array);
    path_finder(&mut array, &mut array_point, start_position, -1);

    for i in 0.. array_point.len() - 1 {
        polygon.edges.push(Edge::new((array_point[i].0 as f64, array_point[i].1 as f64), (array_point[i+1].0 as f64, array_point[i+1].1 as f64)))
    }

    let column_len = array.len();
    let row_len = array[0].len();
    
    let mut result = 0;
    let mut number_in_loop = 0;
    for i in 0..column_len {
        for j in 0..row_len {
            if array[i][j].1 == -1 {
                if pt_in_polygon(&p(j as f64, i as f64), &polygon) == true {
                    array[i][j].1 = -2;
                    number_in_loop += 1;
                }
            }
        }
    }
    println!("{}", number_in_loop);
}
