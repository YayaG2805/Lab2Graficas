use crate::framebuffer::{ALIVE_COLOR, Framebuffer};
use crate::patterns::create_initial_pattern;

#[derive(Debug, Clone)]
pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub current: Vec<bool>,
    next: Vec<bool>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let current = create_initial_pattern(width, height);
        let next = vec![false; width * height];

        Self {
            width,
            height,
            current,
            next,
        }
    }

    pub fn from_cells(width: usize, height: usize, current: Vec<bool>) -> Self {
        assert_eq!(current.len(), width * height);
        let next = vec![false; width * height];

        Self {
            width,
            height,
            current,
            next,
        }
    }

    pub fn reset(&mut self) {
        self.current = create_initial_pattern(self.width, self.height);
        self.next.fill(false);
    }

    pub fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let alive = self.current[idx];
                let neighbors = count_live_neighbors(self, x, y);

                self.next[idx] = matches!((alive, neighbors), (true, 2 | 3) | (false, 3));
            }
        }

        std::mem::swap(&mut self.current, &mut self.next);
        self.next.fill(false);
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.current[self.index(x, y)]
    }

    pub fn render(&self, framebuffer: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if self.is_alive(x, y) {
                    ALIVE_COLOR
                } else {
                    framebuffer.background_color
                };
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
                debug_assert!(framebuffer.get_color(x as isize, y as isize).is_some());
            }
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

pub fn count_live_neighbors(game: &GameOfLife, x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = (x as isize + dx).rem_euclid(game.width as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(game.height as isize) as usize;

            if game.current[ny * game.width + nx] {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framebuffer::DEAD_COLOR;
    use crate::patterns::{add_blinker, add_block, set_alive};

    fn empty_game(width: usize, height: usize) -> GameOfLife {
        GameOfLife::from_cells(width, height, vec![false; width * height])
    }

    #[test]
    fn live_cell_with_one_neighbor_dies() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 2, 2);
        set_alive(&mut game.current, 5, 5, 2, 3);

        game.step();

        assert!(!game.is_alive(2, 2));
    }

    #[test]
    fn live_cell_with_two_neighbors_survives() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 2, 2);
        set_alive(&mut game.current, 5, 5, 1, 2);
        set_alive(&mut game.current, 5, 5, 3, 2);

        game.step();

        assert!(game.is_alive(2, 2));
    }

    #[test]
    fn live_cell_with_three_neighbors_survives() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 2, 2);
        set_alive(&mut game.current, 5, 5, 1, 2);
        set_alive(&mut game.current, 5, 5, 3, 2);
        set_alive(&mut game.current, 5, 5, 2, 1);

        game.step();

        assert!(game.is_alive(2, 2));
    }

    #[test]
    fn live_cell_with_four_neighbors_dies() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 2, 2);
        set_alive(&mut game.current, 5, 5, 1, 2);
        set_alive(&mut game.current, 5, 5, 3, 2);
        set_alive(&mut game.current, 5, 5, 2, 1);
        set_alive(&mut game.current, 5, 5, 2, 3);

        game.step();

        assert!(!game.is_alive(2, 2));
    }

    #[test]
    fn dead_cell_with_three_neighbors_becomes_alive() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 1, 2);
        set_alive(&mut game.current, 5, 5, 3, 2);
        set_alive(&mut game.current, 5, 5, 2, 1);

        game.step();

        assert!(game.is_alive(2, 2));
    }

    #[test]
    fn block_is_stable_after_one_generation() {
        let mut cells = vec![false; 6 * 6];
        add_block(&mut cells, 6, 6, 2, 2);
        let expected = cells.clone();
        let mut game = GameOfLife::from_cells(6, 6, cells);

        game.step();

        assert_eq!(game.current, expected);
    }

    #[test]
    fn blinker_rotates_after_one_generation() {
        let mut cells = vec![false; 7 * 7];
        add_blinker(&mut cells, 7, 7, 2, 3);
        let mut game = GameOfLife::from_cells(7, 7, cells);

        game.step();

        assert!(game.is_alive(3, 2));
        assert!(game.is_alive(3, 3));
        assert!(game.is_alive(3, 4));
        assert!(!game.is_alive(2, 3));
        assert!(!game.is_alive(4, 3));
    }

    #[test]
    fn wrap_around_counts_neighbors_from_opposite_edges() {
        let mut game = empty_game(5, 5);
        set_alive(&mut game.current, 5, 5, 4, 0);
        set_alive(&mut game.current, 5, 5, 0, 4);
        set_alive(&mut game.current, 5, 5, 4, 4);

        assert_eq!(count_live_neighbors(&game, 0, 0), 3);
    }

    #[test]
    fn state_size_stays_the_same_after_step() {
        let mut game = GameOfLife::new(100, 100);

        game.step();

        assert_eq!(game.current.len(), 100 * 100);
    }

    #[test]
    fn get_color_handles_invalid_coordinates() {
        let framebuffer = Framebuffer::new(10, 10);

        assert_eq!(framebuffer.get_color(-1, 0), None);
        assert_eq!(framebuffer.get_color(10, 0), None);
        assert_eq!(framebuffer.get_color(0, 10), None);
        assert_eq!(framebuffer.get_color(0, 0), Some(DEAD_COLOR));
    }
}
