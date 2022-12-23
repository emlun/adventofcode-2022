use crate::common::Solution;

const CUBE_SIDE: usize = 50;

struct Connections {
    from: (usize, usize),
    up: Connection,
    down: Connection,
    left: Connection,
    right: Connection,
}

struct Connection {
    to: (usize, usize),
    rot: usize,
}

const MY_CUBE_CONNECTIONS: [Connections; 6] = [
    Connections {
        from: (1, 0),
        right: Connection { to: (2, 0), rot: 0 },
        down: Connection { to: (1, 1), rot: 0 },
        left: Connection { to: (0, 2), rot: 2 },
        up: Connection { to: (0, 3), rot: 1 },
    },
    Connections {
        from: (1, 1),
        right: Connection { to: (2, 0), rot: 3 },
        down: Connection { to: (1, 2), rot: 0 },
        left: Connection { to: (0, 2), rot: 3 },
        up: Connection { to: (1, 0), rot: 0 },
    },
    Connections {
        from: (1, 2),
        right: Connection { to: (2, 0), rot: 2 },
        down: Connection { to: (0, 3), rot: 1 },
        left: Connection { to: (0, 2), rot: 0 },
        up: Connection { to: (1, 1), rot: 0 },
    },
    Connections {
        from: (0, 2),
        right: Connection { to: (1, 2), rot: 0 },
        down: Connection { to: (0, 3), rot: 0 },
        left: Connection { to: (1, 0), rot: 2 },
        up: Connection { to: (1, 1), rot: 1 },
    },
    Connections {
        from: (0, 3),
        right: Connection { to: (1, 2), rot: 3 },
        down: Connection { to: (2, 0), rot: 0 },
        left: Connection { to: (1, 0), rot: 3 },
        up: Connection { to: (0, 2), rot: 1 },
    },
    Connections {
        from: (2, 0),
        right: Connection { to: (1, 2), rot: 2 },
        down: Connection { to: (1, 1), rot: 1 },
        left: Connection { to: (1, 0), rot: 0 },
        up: Connection { to: (0, 3), rot: 0 },
    },
];

// const MY_CUBE_CONNECTIONS: [Connections; 6] = [
//     Connections {
//         from: (2, 0),
//         right: Connection { to: (3, 2), rot: 2 },
//         down: Connection { to: (2, 1), rot: 0 },
//         left: Connection { to: (1, 1), rot: 3 },
//         up: Connection { to: (0, 1), rot: 2 },
//     },
//     Connections {
//         from: (0, 1),
//         right: Connection { to: (0, 2), rot: 0 },
//         down: Connection { to: (2, 2), rot: 2 },
//         left: Connection { to: (3, 2), rot: 1 },
//         up: Connection { to: (2, 0), rot: 2 },
//     },
//     Connections {
//         from: (1, 1),
//         right: Connection { to: (2, 1), rot: 0 },
//         down: Connection { to: (2, 2), rot: 3 },
//         left: Connection { to: (0, 1), rot: 0 },
//         up: Connection { to: (2, 0), rot: 1 },
//     },
//     Connections {
//         from: (2, 1),
//         right: Connection { to: (3, 2), rot: 1 },
//         down: Connection { to: (2, 2), rot: 0 },
//         left: Connection { to: (1, 1), rot: 0 },
//         up: Connection { to: (2, 0), rot: 0 },
//     },
//     Connections {
//         from: (2, 2),
//         right: Connection { to: (3, 2), rot: 0 },
//         down: Connection { to: (0, 1), rot: 2 },
//         left: Connection { to: (1, 1), rot: 1 },
//         up: Connection { to: (2, 1), rot: 0 },
//     },
//     Connections {
//         from: (3, 2),
//         right: Connection { to: (2, 0), rot: 2 },
//         down: Connection { to: (0, 1), rot: 3 },
//         left: Connection { to: (2, 2), rot: 0 },
//         up: Connection { to: (2, 1), rot: 3 },
//     },
// ];

fn rot((x, y): (isize, isize), r: usize) -> (isize, isize) {
    if r >= 1 {
        rot(((CUBE_SIDE - 1) as isize - y, x), r - 1)
    } else {
        (x, y)
    }
}

struct Map {
    minic: Vec<usize>,
    maxxc: Vec<usize>,
    minir: Vec<usize>,
    maxxr: Vec<usize>,
    walls: Vec<Vec<bool>>,
}

impl Map {
    fn new(h: usize) -> Self {
        Self {
            minic: Vec::with_capacity(h),
            maxxc: Vec::with_capacity(h),
            minir: Vec::with_capacity(h),
            maxxr: Vec::with_capacity(h),
            walls: Vec::with_capacity(h),
        }
    }
}

fn solve_a(map: &Map, path_len: &[usize], path_turn: &[bool]) -> usize {
    let mut c = map.minic[0];
    let mut r = map.minir[c];
    let mut dir = 0;

    for (i, l) in path_len.iter().copied().enumerate() {
        let h = map.maxxr[c] - map.minir[c];
        let w = map.maxxc[r] - map.minic[r];

        let (dr, dc) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, w - 1),
            3 => (h - 1, 0),
            _ => unimplemented!(),
        };

        let dx = (1..=l)
            .take_while(|dx| {
                let nr = map.minir[c] + (r - map.minir[c] + dr * dx) % h;
                let nc = map.minic[r] + (c - map.minic[r] + dc * dx) % w;
                !map.walls[nr][nc]
            })
            .last()
            .unwrap_or(0);

        r = if dr == 0 {
            r
        } else {
            map.minir[c] + (r - map.minir[c] + dr * dx) % h
        };
        c = if dc == 0 {
            c
        } else {
            map.minic[r] + (c - map.minic[r] + dc * dx) % w
        };

        dir = match path_turn.get(i) {
            Some(true) => (dir + 1) % 4,
            Some(false) => (dir + 3) % 4,
            None => dir,
        }
    }

    (r + 1) * 1000 + (c + 1) * 4 + dir
}

fn solve_b(map: &Map, path_len: &[usize], path_turn: &[bool]) -> usize {
    let mut c = map.minic[0];
    let mut r = map.minir[c];
    let mut dir = 0;

    let mut poss = vec![];

    for (i, l) in path_len.iter().copied().enumerate() {
        for _ in 1..=l {
            poss.push((r, c, dir));

            let (dr, dc): (isize, isize) = match dir {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                3 => (-1, 0),
                _ => unimplemented!(),
            };

            let mut nr: isize = r as isize + dr;
            let mut nc: isize = c as isize + dc;
            let mut ndir = dir;

            if nr < map.minir[c] as isize
                || nr >= map.maxxr[c] as isize
                || nc < map.minic[r] as isize
                || nc >= map.maxxc[r] as isize
            {
                let face_x: usize = c / CUBE_SIDE;
                let face_y: usize = r / CUBE_SIDE;
                let connections = MY_CUBE_CONNECTIONS
                    .iter()
                    .find(|conn| conn.from == (face_x, face_y))
                    .unwrap();
                let connection = match dir {
                    0 => &connections.right,
                    1 => &connections.down,
                    2 => &connections.left,
                    3 => &connections.up,
                    _ => unimplemented!(),
                };

                let nrl = nr.rem_euclid(CUBE_SIDE as isize);
                let ncl = nc.rem_euclid(CUBE_SIDE as isize);

                let (nclt, nrlt) = rot((ncl, nrl), connection.rot);

                nr = (connection.to.1 * CUBE_SIDE) as isize + nrlt;
                nc = (connection.to.0 * CUBE_SIDE) as isize + nclt;
                ndir = (dir + connection.rot) % 4;
            }

            if map.walls[usize::try_from(nr).unwrap()][usize::try_from(nc).unwrap()] {
                break;
            } else {
                r = nr as usize;
                c = nc as usize;
                dir = ndir;
            }
        }

        dir = match path_turn.get(i) {
            Some(true) => (dir + 1) % 4,
            Some(false) => (dir + 3) % 4,
            None => dir,
        };
    }

    (r + 1) * 1000 + (c + 1) * 4 + dir
}

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Map = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .fold(Map::new(lines.len()), |mut map, (r, line)| {
            map.maxxc.push(0);
            map.minir
                .resize(std::cmp::max(map.minir.len(), line.len()), 0);
            map.maxxr
                .resize(std::cmp::max(map.maxxr.len(), line.len()), 0);
            map = line.chars().enumerate().fold(map, |mut map, (c, chr)| {
                if chr == '.' || chr == '#' {
                    if map.minic.len() <= r {
                        map.minic.push(c);
                    }
                    if map.minir[c] == 0 {
                        map.minir[c] = r + 1;
                    }
                    map.maxxc[r] = c + 1;
                    map.maxxr[c] = r + 1;
                }
                map
            });

            map.walls.push(line.chars().map(|chr| chr == '#').collect());

            map
        });
    map.minir = map.minir.iter().map(|minir| minir - 1).collect();

    let path_len: Vec<usize> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .last()
        .unwrap()
        .split(|chr| chr == 'L' || chr == 'R')
        .map(|s| s.parse().unwrap())
        .collect();
    let path_turn: Vec<bool> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .last()
        .unwrap()
        .chars()
        .filter(|chr| *chr == 'L' || *chr == 'R')
        .map(|chr| chr == 'R')
        .collect();

    (
        solve_a(&map, &path_len, &path_turn).to_string(),
        solve_b(&map, &path_len, &path_turn).to_string(),
    )
}
