use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

pub type Hints = (Vec<Vec<usize>>, Vec<Vec<usize>>);
pub fn gen_inner(h: usize, w: usize, seed: u64) -> (Vec<bool>, Hints) {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let (grid, hints) = loop {
        let mut board = Board::new(h, w, (0..h * w).map(|_| Some(rng.gen_bool(0.5))).collect());
        let hints = board.get_hints();

        let mut full_or_empty_line_doesnt_exist = true;
        full_or_empty_line_doesnt_exist &= hints.0.iter().all(|hint| hint.get(0) != Some(&w));
        full_or_empty_line_doesnt_exist &= hints.0.iter().all(|hint| !hint.is_empty());
        full_or_empty_line_doesnt_exist &= hints.1.iter().all(|hint| hint.get(0) != Some(&h));
        full_or_empty_line_doesnt_exist &= hints.1.iter().all(|hint| !hint.is_empty());

        let mut solver = Solver::new(h, w, hints.clone());
        if solver.solve() && full_or_empty_line_doesnt_exist {
            println!("{:?}", solver.board);
            break (solver.board.get_plain(), hints);
        }
    };
    (grid, hints)
}

struct Board {
    h: usize,
    w: usize,
    grid: Vec<Option<bool>>,
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, grid) in self.grid.iter().enumerate() {
            write!(
                f,
                "{}",
                match grid {
                    Some(true) => "o",
                    Some(false) => ".",
                    None => "?",
                }
            )?;
            if i % self.w == self.w - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Board {
    fn new(h: usize, w: usize, grid: Vec<Option<bool>>) -> Self {
        Self { h, w, grid }
    }
    fn get(&self, y: usize, x: usize) -> Option<bool> {
        assert!(y < self.h);
        assert!(x < self.w);
        self.grid[y * self.w + x]
    }
    fn set(&mut self, y: usize, x: usize, val: Option<bool>) {
        assert!(y < self.h);
        assert!(x < self.w);
        self.grid[y * self.w + x] = val;
    }
    fn transpose(&mut self) {
        self.grid = (0..self.grid.len())
            .map(|i| self.get(i % self.h, i / self.h))
            .collect();
        std::mem::swap(&mut self.h, &mut self.w);
    }
    fn compress(&self, y: usize) -> Vec<usize> {
        let mut ret = Vec::<usize>::new();
        let mut i = 0;
        while i < self.w {
            let mut j = i;
            while j < self.w && self.get(y, i) == self.get(y, j) {
                j += 1;
            }
            if self.get(y, i) == Some(true) {
                ret.push(j - i);
            }
            i = j;
        }
        ret
    }
    fn get_plain(&self) -> Vec<bool> {
        self.grid.iter().map(|&f| f.unwrap()).collect()
    }
    fn get_hints(&mut self) -> Hints {
        let hints_horizontal = (0..self.h).map(|y| self.compress(y)).collect::<Vec<_>>();
        self.transpose();
        let hints_vertical = (0..self.h).map(|y| self.compress(y)).collect::<Vec<_>>();
        self.transpose();
        (hints_horizontal, hints_vertical)
    }
}

struct Solver {
    hints: Hints,
    board: Board,
}
impl Solver {
    fn new(h: usize, w: usize, hints: Hints) -> Self {
        Self {
            hints,
            board: Board::new(h, w, vec![None; h * w]),
        }
    }
    fn solve(&mut self) -> bool {
        loop {
            let mut upd = false;
            for _ in 0..2 {
                for y in 0..self.board.h {
                    upd |= self.solve_line(y);
                }
                self.board.transpose();
                std::mem::swap(&mut self.hints.0, &mut self.hints.1);
            }
            if !upd {
                break;
            }
        }
        self.board.grid.iter().all(|&f| f.is_some())
    }
    fn solve_line(&mut self, y: usize) -> bool {
        let empty_indice = (0..self.board.w)
            .filter(|&x| self.board.get(y, x).is_none())
            .collect::<Vec<_>>();
        let need: usize = self.hints.0[y].iter().sum::<usize>()
            - (0..self.board.w)
                .filter(|&x| self.board.get(y, x) == Some(true))
                .count();
        let mut can_be_true = vec![false; self.board.w];
        let mut can_be_false = vec![false; self.board.w];
        for true_indice in empty_indice.iter().combinations(need) {
            for &x in &empty_indice {
                self.board.set(y, x, Some(false));
            }
            for &&x in &true_indice {
                self.board.set(y, x, Some(true));
            }
            if self.board.compress(y) == self.hints.0[y] {
                for &x in &empty_indice {
                    match self.board.get(y, x) {
                        Some(true) => can_be_true[x] = true,
                        Some(false) => can_be_false[x] = true,
                        None => unreachable!(),
                    }
                }
            }
            for &x in &empty_indice {
                self.board.set(y, x, None);
            }
        }

        let mut upd = false;
        for &x in &empty_indice {
            if can_be_true[x] && !can_be_false[x] {
                self.board.set(y, x, Some(true));
                upd = true;
            }
            if !can_be_true[x] && can_be_false[x] {
                self.board.set(y, x, Some(false));
                upd = true;
            }
        }
        upd
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn test_transpose() {
        let mut board = Board::new(2, 3, vec![None; 6]);
        board.grid = vec![Some(true), Some(false), None, None, Some(true), Some(false)];
        println!("{:?}\n", board);
        board.transpose();
        println!("{:?}\n", board);
        board.transpose();
        println!("{:?}\n", board);
    }

    #[test]
    fn test_combination() {
        for comb in ([1, 5, 7, 9]).iter().combinations(2) {
            println!("{:?}", comb);
        }
    }

    #[test]
    fn test_compress() {
        let board = Board::new(
            1,
            10,
            vec![
                Some(false),
                Some(true),
                Some(true),
                Some(true),
                Some(false),
                Some(true),
                Some(true),
                Some(true),
                Some(true),
                Some(false),
            ],
        );
        assert_eq!(board.compress(0), vec![3, 4]);

        let board = Board::new(
            1,
            10,
            vec![
                Some(true),
                Some(true),
                Some(false),
                Some(false),
                Some(true),
                Some(true),
                Some(true),
                Some(false),
                Some(false),
                Some(true),
            ],
        );
        assert_eq!(board.compress(0), vec![2, 3, 1]);
    }

    #[test]
    fn test_solve() {
        let mut solver = Solver::new(
            5,
            5,
            (
                vec![vec![2], vec![2], vec![3], vec![5], vec![4]],
                vec![vec![2], vec![2], vec![3], vec![5], vec![4]],
            ),
        );
        if solver.solve() {
            println!("{:?}\n", solver.board);
        } else {
            println!("cannot solve\n");
        }

        let mut solver = Solver::new(
            15,
            15,
            (
                vec![
                    vec![2, 2],
                    vec![1, 1, 1, 1],
                    vec![15],
                    vec![4, 1, 4],
                    vec![3, 3, 3],
                    vec![1, 1, 1],
                    vec![1, 3, 1],
                    vec![1, 1, 1],
                    vec![3, 1, 1, 3],
                    vec![1, 1, 1, 1],
                    vec![3, 3, 3],
                    vec![1, 1, 1],
                    vec![2, 2],
                    vec![2, 2],
                    vec![11],
                ],
                vec![
                    vec![4, 7],
                    vec![1, 4, 1, 1, 2],
                    vec![5, 1, 1, 2],
                    vec![2, 1],
                    vec![1, 2, 1],
                    vec![1, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![6, 2, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![2, 1],
                    vec![5, 1, 1, 2],
                    vec![1, 4, 1, 1, 2],
                    vec![4, 7],
                ],
            ),
        );
        if solver.solve() {
            println!("{:?}\n", solver.board);
        } else {
            println!("cannot solve\n");
        }
    }
}
