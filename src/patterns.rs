fn index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

pub fn set_alive(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    if x < width && y < height {
        cells[index(width, x, y)] = true;
    }
}

pub fn add_block(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_beehive(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_loaf(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_boat(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_tub(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (0, 1), (2, 1), (1, 2)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_blinker(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for dx in 0..3 {
        set_alive(cells, width, height, x + dx, y);
    }
}

pub fn add_toad(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_beacon(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 2),
        (2, 3),
        (3, 3),
    ] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_glider(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_lwss(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    for (dx, dy) in [
        (1, 0),
        (4, 0),
        (0, 1),
        (0, 2),
        (4, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
    ] {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn add_pulsar(cells: &mut [bool], width: usize, height: usize, x: usize, y: usize) {
    const POINTS: &[(usize, usize)] = &[
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (0, 8),
        (5, 8),
        (7, 8),
        (12, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ];

    for &(dx, dy) in POINTS {
        set_alive(cells, width, height, x + dx, y + dy);
    }
}

pub fn create_initial_pattern(width: usize, height: usize) -> Vec<bool> {
    let mut cells = vec![false; width * height];

    add_block(&mut cells, width, height, 4, 4);
    add_beehive(&mut cells, width, height, 18, 6);
    add_loaf(&mut cells, width, height, 35, 7);
    add_boat(&mut cells, width, height, 52, 6);
    add_tub(&mut cells, width, height, 70, 8);
    add_blinker(&mut cells, width, height, 10, 30);
    add_toad(&mut cells, width, height, 28, 28);
    add_beacon(&mut cells, width, height, 48, 28);
    add_glider(&mut cells, width, height, 70, 28);
    add_lwss(&mut cells, width, height, 12, 58);
    add_pulsar(&mut cells, width, height, 43, 55);
    add_glider(&mut cells, width, height, 82, 65);
    add_lwss(&mut cells, width, height, 68, 82);

    cells
}
