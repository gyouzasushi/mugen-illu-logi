use itertools::Itertools;
use rand::prelude::*;
use std::fmt;
use std::{cmp::Ordering, collections::VecDeque};

pub type Hints = (Vec<Vec<usize>>, Vec<Vec<usize>>);
pub fn gen_inner(h: usize, w: usize, seed: u64) -> (Vec<bool>, Hints) {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let (grid, hints) = loop {
        let mut board = Board::new(
            h,
            w,
            (0..h)
                .map(|_| (0..w).map(|_| Some(rng.gen_bool(0.5))).collect())
                .collect(),
        );
        let hints = board.get_hints();

        let mut full_or_empty_line_doesnt_exist = true;
        full_or_empty_line_doesnt_exist &= hints.0.iter().all(|hint| hint.get(0) != Some(&w));
        full_or_empty_line_doesnt_exist &= hints.0.iter().all(|hint| !hint.is_empty());
        full_or_empty_line_doesnt_exist &= hints.1.iter().all(|hint| hint.get(0) != Some(&h));
        full_or_empty_line_doesnt_exist &= hints.1.iter().all(|hint| !hint.is_empty());

        let mut solver = Solver::new(h, w, hints.clone());
        if solver.solve() && full_or_empty_line_doesnt_exist {
            break (solver.board.get_plain(), hints);
        }
    };
    (grid, hints)
}

struct Board {
    h: usize,
    w: usize,
    grid: Vec<Vec<Option<bool>>>,
}
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for grid in self.grid.iter() {
            for grid in grid {
                write!(
                    f,
                    "{}",
                    match grid {
                        Some(true) => "o",
                        Some(false) => ".",
                        None => "?",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn new(h: usize, w: usize, grid: Vec<Vec<Option<bool>>>) -> Self {
        Self { h, w, grid }
    }
    fn set(&mut self, y: usize, x: usize, val: Option<bool>) {
        self.grid[y][x] = val;
    }
    fn compress_horizontal(&self, y: usize) -> Vec<usize> {
        let mut ret = Vec::<usize>::new();
        let mut i = 0;
        while i < self.w {
            let mut j = i;
            while j < self.w && self.grid[y][i] == self.grid[y][j] {
                j += 1;
            }
            if self.grid[y][i] == Some(true) {
                ret.push(j - i);
            }
            i = j;
        }
        ret
    }
    fn compress_vertical(&self, x: usize) -> Vec<usize> {
        let mut ret = Vec::<usize>::new();
        let mut i = 0;
        while i < self.h {
            let mut j = i;
            while j < self.h && self.grid[i][x] == self.grid[j][x] {
                j += 1;
            }
            if self.grid[i][x] == Some(true) {
                ret.push(j - i);
            }
            i = j;
        }
        ret
    }
    fn get_plain(&self) -> Vec<bool> {
        self.grid.iter().flatten().map(|&f| f.unwrap()).collect()
    }
    fn get_hints(&mut self) -> Hints {
        let hints_horizontal = (0..self.h)
            .map(|y| self.compress_horizontal(y))
            .collect::<Vec<_>>();
        let hints_vertical = (0..self.w)
            .map(|x| self.compress_vertical(x))
            .collect::<Vec<_>>();
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
            board: Board::new(h, w, vec![vec![None; w]; h]),
        }
    }

    fn set(
        &mut self,
        y: usize,
        x: usize,
        val: Option<bool>,
        q: &mut VecDeque<(usize, bool)>,
        used: &mut (Vec<bool>, Vec<bool>),
    ) {
        if self.board.grid[y][x] == val {
            return;
        }
        self.board.set(y, x, val);
        if !used.0[y] {
            q.push_back((y, true));
            used.0[y] = true;
        }
        if !used.1[x] {
            q.push_back((x, false));
            used.1[x] = true;
        }
    }

    fn solve(&mut self) -> bool {
        let mut q = VecDeque::<(usize, bool)>::new();
        let mut used = (vec![false; self.board.h], vec![false; self.board.w]);

        for y in 0..self.board.h {
            self.solve_line_initial_horizontal(y, &mut q, &mut used);
        }
        for x in 0..self.board.w {
            self.solve_line_initial_vertical(x, &mut q, &mut used);
        }

        while let Some((i, is_horizontal)) = q.pop_front() {
            if is_horizontal {
                let y = i;
                used.0[y] = false;
                self.solve_line_horizontal(y, &mut q, &mut used);
            } else {
                let x = i;
                used.1[x] = false;
                self.solve_line_vertical(x, &mut q, &mut used);
            }
        }
        self.board.grid.iter().flatten().all(|&f| f.is_some())
    }

    fn solve_line_initial_horizontal(
        &mut self,
        y: usize,
        q: &mut VecDeque<(usize, bool)>,
        used: &mut (Vec<bool>, Vec<bool>),
    ) {
        let mut l_sum = 0_usize;
        let mut r_sum = self.hints.0[y].iter().sum::<usize>();
        let n = self.hints.0[y].len();
        for i in 0..self.hints.0[y].len() {
            let hint = self.hints.0[y][i];
            r_sum -= hint;
            let l0 = l_sum + i;
            let r0 = l0 + hint - 1;
            let r1 = self.board.w - 1 - (r_sum + (n - 1 - i));
            let l1 = r1 - (hint - 1);
            if l1 > r0 {
                continue;
            }
            for x in l1..=r0 {
                self.set(y, x, Some(true), q, used);
            }
            l_sum += hint;
        }
    }
    fn solve_line_initial_vertical(
        &mut self,
        x: usize,
        q: &mut VecDeque<(usize, bool)>,
        used: &mut (Vec<bool>, Vec<bool>),
    ) {
        let mut l_sum = 0_usize;
        let mut r_sum = self.hints.1[x].iter().sum::<usize>();
        let n = self.hints.1[x].len();
        for i in 0..self.hints.1[x].len() {
            let hint = self.hints.1[x][i];
            r_sum -= hint;
            let l0 = l_sum + i;
            let r0 = l0 + hint - 1;
            let r1 = self.board.w - 1 - (r_sum + (n - 1 - i));
            let l1 = r1 - (hint - 1);
            if l1 > r0 {
                continue;
            }
            for y in l1..=r0 {
                self.set(y, x, Some(true), q, used);
            }
            l_sum += hint;
        }
    }

    fn solve_line_horizontal(
        &mut self,
        y: usize,
        q: &mut VecDeque<(usize, bool)>,
        used: &mut (Vec<bool>, Vec<bool>),
    ) {
        // 未確定マスの埋め方を全探索
        if (0..self.board.w)
            .filter(|&x| self.board.grid[y][x].is_none())
            .count()
            <= 13
        {
            let empty_indice = (0..self.board.w)
                .filter(|&x| self.board.grid[y][x].is_none())
                .collect::<Vec<_>>();
            let need: usize = self.hints.0[y].iter().sum::<usize>()
                - (0..self.board.w)
                    .filter(|&x| self.board.grid[y][x] == Some(true))
                    .count();
            let mut can_be_true = vec![false; self.board.w];
            let mut can_be_false = vec![false; self.board.w];
            for true_indice in empty_indice.iter().combinations(need) {
                for &x in &empty_indice {
                    self.board.grid[y][x] = Some(false);
                }
                for &&x in &true_indice {
                    self.board.grid[y][x] = Some(true);
                }
                if self.board.compress_horizontal(y) == self.hints.0[y] {
                    for &x in &empty_indice {
                        match self.board.grid[y][x] {
                            Some(true) => can_be_true[x] = true,
                            Some(false) => can_be_false[x] = true,
                            None => unreachable!(),
                        }
                    }
                }
                for &x in &empty_indice {
                    self.board.grid[y][x] = None;
                }
            }
            for &x in &empty_indice {
                if can_be_true[x] && !can_be_false[x] {
                    self.set(y, x, Some(true), q, used);
                }
                if !can_be_true[x] && can_be_false[x] {
                    self.set(y, x, Some(false), q, used);
                }
            }
        }

        // 前から見て確定するマスを埋めていく
        let mut x = 0;
        let mut i = 0;
        while x < self.board.w && i < self.hints.0[y].len() {
            let mut nx = x;
            while nx < self.board.w
                && (self.board.grid[y][x] == Some(false)) == (self.board.grid[y][nx] == Some(false))
            {
                nx += 1;
            }
            let len = nx - x;
            match self.board.grid[y][x] {
                Some(true) | None => match len.cmp(&self.hints.0[y][i]) {
                    Ordering::Equal => {
                        if (x..nx).any(|x| self.board.grid[y][x] == Some(true)) {
                            for x in x..nx {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    Ordering::Greater => {
                        nx = x + self.hints.0[y][i];
                        if self.board.grid[y][x] == Some(true) {
                            for x in x..nx {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i += 1;
                        } else if self.board.grid[y][nx] == Some(true) {
                            self.set(y, x, Some(false), q, used);
                            nx = x + 1;
                        } else if let Some(x) =
                            (x..nx).find(|&x| self.board.grid[y][x] == Some(true))
                        {
                            for x in x..nx {
                                self.set(y, x, Some(true), q, used);
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                    Ordering::Less => {
                        assert!((x..nx).all(|x| self.board.grid[y][x] != Some(true)));
                        for x in x..nx {
                            if self.board.grid[y][x].is_none() {
                                self.set(y, x, Some(false), q, used);
                            }
                        }
                    }
                },
                Some(false) => {}
            }
            x = nx;
        }

        // 後ろから見て確定するマスを埋めていく
        let mut x = self.board.w - 1;
        let mut i = self.hints.0[y].len() - 1;
        while x < self.board.w && i < self.hints.0[y].len() {
            let mut nx = x;
            while nx < self.board.w
                && (self.board.grid[y][x] == Some(false)) == (self.board.grid[y][nx] == Some(false))
            {
                nx = nx.wrapping_sub(1);
            }
            let len = x.wrapping_sub(nx);
            match self.board.grid[y][x] {
                Some(true) | None => match len.cmp(&self.hints.0[y][i]) {
                    Ordering::Equal => {
                        if (nx.wrapping_add(1)..=x).any(|x| self.board.grid[y][x] == Some(true)) {
                            for x in nx.wrapping_add(1)..=x {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i = i.wrapping_sub(1);
                        } else {
                            break;
                        }
                    }
                    Ordering::Greater => {
                        nx = x.wrapping_sub(self.hints.0[y][i]);
                        if self.board.grid[y][x] == Some(true) {
                            for x in nx.wrapping_add(1)..=x {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i = i.wrapping_sub(1);
                        } else if self.board.grid[y][nx] == Some(true) {
                            self.set(y, x, Some(false), q, used);
                            nx = x.wrapping_sub(1);
                        } else if let Some(x) = (nx.wrapping_add(1)..=x)
                            .rev()
                            .find(|&x| self.board.grid[y][x] == Some(true))
                        {
                            for x in nx.wrapping_add(1)..=x {
                                self.set(y, x, Some(true), q, used);
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                    Ordering::Less => {
                        assert!(
                            (nx.wrapping_add(1)..=x).all(|x| self.board.grid[y][x] != Some(true))
                        );
                        for x in nx.wrapping_add(1)..=x {
                            if self.board.grid[y][x].is_none() {
                                self.set(y, x, Some(false), q, used);
                            }
                        }
                    }
                },
                Some(false) => {}
            }
            x = nx;
        }
    }

    fn solve_line_vertical(
        &mut self,
        x: usize,
        q: &mut VecDeque<(usize, bool)>,
        used: &mut (Vec<bool>, Vec<bool>),
    ) {
        // 未確定マスの埋め方を全探索
        if (0..self.board.h)
            .filter(|&y| self.board.grid[y][x].is_none())
            .count()
            <= 13
        {
            let empty_indice = (0..self.board.h)
                .filter(|&y| self.board.grid[y][x].is_none())
                .collect::<Vec<_>>();
            let need: usize = self.hints.1[x].iter().sum::<usize>()
                - (0..self.board.h)
                    .filter(|&y| self.board.grid[y][x] == Some(true))
                    .count();
            let mut can_be_true = vec![false; self.board.h];
            let mut can_be_false = vec![false; self.board.h];
            for true_indice in empty_indice.iter().combinations(need) {
                for &y in &empty_indice {
                    self.board.grid[y][x] = Some(false);
                }
                for &&y in &true_indice {
                    self.board.grid[y][x] = Some(true);
                }
                if self.board.compress_vertical(x) == self.hints.1[x] {
                    for &y in &empty_indice {
                        match self.board.grid[y][x] {
                            Some(true) => can_be_true[y] = true,
                            Some(false) => can_be_false[y] = true,
                            None => unreachable!(),
                        }
                    }
                }
                for &y in &empty_indice {
                    self.board.grid[y][x] = None;
                }
            }
            for &y in &empty_indice {
                if can_be_true[y] && !can_be_false[y] {
                    self.set(y, x, Some(true), q, used);
                }
                if !can_be_true[y] && can_be_false[y] {
                    self.set(y, x, Some(false), q, used);
                }
            }
        }

        // 前から見て確定するマスを埋めていく
        let mut y = 0;
        let mut i = 0;
        while y < self.board.h && i < self.hints.1[x].len() {
            let mut ny = y;
            while ny < self.board.h
                && (self.board.grid[y][x] == Some(false)) == (self.board.grid[ny][x] == Some(false))
            {
                ny += 1;
            }
            let len = ny - y;
            match self.board.grid[y][x] {
                Some(true) | None => match len.cmp(&self.hints.1[x][i]) {
                    Ordering::Equal => {
                        if (y..ny).any(|y| self.board.grid[y][x] == Some(true)) {
                            for y in y..ny {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    Ordering::Greater => {
                        ny = y + self.hints.1[x][i];
                        if self.board.grid[y][x] == Some(true) {
                            for y in y..ny {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i += 1;
                        } else if self.board.grid[ny][x] == Some(true) {
                            self.set(y, x, Some(false), q, used);
                            ny = y + 1;
                        } else if let Some(y) =
                            (y..ny).find(|&y| self.board.grid[y][x] == Some(true))
                        {
                            for y in y..ny {
                                self.set(y, x, Some(true), q, used);
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                    Ordering::Less => {
                        assert!((y..ny).all(|y| self.board.grid[y][x] != Some(true)));
                        for y in y..ny {
                            if self.board.grid[y][x].is_none() {
                                self.set(y, x, Some(false), q, used);
                            }
                        }
                    }
                },
                Some(false) => {}
            }
            y = ny;
        }

        // 後ろから見て確定するマスを埋めていく
        let mut y = self.board.h - 1;
        let mut i = self.hints.1[x].len() - 1;
        while y < self.board.h && i < self.hints.1[x].len() {
            let mut ny = y;
            while ny < self.board.h
                && (self.board.grid[y][x] == Some(false)) == (self.board.grid[ny][x] == Some(false))
            {
                ny = ny.wrapping_sub(1);
            }
            let len = y.wrapping_sub(ny);
            match self.board.grid[y][x] {
                Some(true) | None => match len.cmp(&self.hints.1[x][i]) {
                    Ordering::Equal => {
                        if (ny.wrapping_add(1)..=y).any(|y| self.board.grid[y][x] == Some(true)) {
                            for y in ny.wrapping_add(1)..=y {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i = i.wrapping_sub(1);
                        } else {
                            break;
                        }
                    }
                    Ordering::Greater => {
                        ny = y.wrapping_sub(self.hints.1[x][i]);
                        if self.board.grid[y][x] == Some(true) {
                            for y in ny.wrapping_add(1)..=y {
                                if self.board.grid[y][x].is_none() {
                                    self.set(y, x, Some(true), q, used);
                                }
                            }
                            i = i.wrapping_sub(1);
                        } else if self.board.grid[ny][x] == Some(true) {
                            self.set(y, x, Some(false), q, used);
                            ny = y.wrapping_sub(1);
                        } else if let Some(y) = (ny.wrapping_add(1)..=y)
                            .rev()
                            .find(|&y| self.board.grid[y][x] == Some(true))
                        {
                            for y in ny.wrapping_add(1)..=y {
                                self.set(y, x, Some(true), q, used);
                            }
                            break;
                        } else {
                            break;
                        }
                    }
                    Ordering::Less => {
                        assert!(
                            (ny.wrapping_add(1)..=y).all(|y| self.board.grid[y][x] != Some(true))
                        );
                        for y in ny.wrapping_add(1)..=y {
                            if self.board.grid[y][x].is_none() {
                                self.set(y, x, Some(false), q, used);
                            }
                        }
                    }
                },
                Some(false) => {}
            }
            y = ny;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
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

    #[test]
    fn test_solve_random() {
        use rand::prelude::*;
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
        let (h, w) = (20, 20);
        let mut board = Board::new(
            h,
            w,
            (0..h)
                .map(|_| (0..w).map(|_| Some(rng.gen_bool(0.5))).collect())
                .collect(),
        );
        let hints = board.get_hints();
        let mut solver = Solver::new(h, w, hints);
        solver.solve();
    }
}
