use itertools::Itertools;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle, Text};
use svg::Document;

const D: i32 = 24;
const OFFSET_X: i32 = 200;
const OFFSET_Y: i32 = 200;
const THEME: &str = "#7BC96F";

pub fn vis_grid_inner(h: usize, w: usize, d: i32, board: &Vec<Vec<Option<bool>>>) -> String {
    let mut doc = Document::new()
        .set("id", "vis")
        .set("viewBox", (0, 0, d * w as i32, d * h as i32))
        .set("width", d * w as i32)
        .set("height", d * h as i32);
    // grids
    for y in 0..h {
        for x in 0..w {
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "fill",
                        if board[y][x] == Some(true) {
                            "black"
                        } else {
                            "white"
                        },
                    )
                    .set("x", x as i32 * d)
                    .set("y", y as i32 * d)
                    .set("width", d)
                    .set("height", d)
                    .set("stroke-width", 0),
            );
        }
    }
    doc.to_string()
}

pub fn vis_gif_inner(h: usize, w: usize, d: i32, boards: &Vec<Vec<Vec<Option<bool>>>>) -> String {
    boards
        .iter()
        .map(|board| vis_grid_inner(h, w, d, board))
        .join("$")
}

pub fn vis_board_inner(
    h: usize,
    w: usize,
    board: &Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
) -> String {
    let hints_hidden = get_hints_hidden(board, hints);

    let mut doc = Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                -5,
                -5,
                D * w as i32 + OFFSET_X + 10,
                D * h as i32 + OFFSET_Y + 10,
            ),
        )
        .set("width", D * w as i32 + OFFSET_X + 10)
        .set("height", D * h as i32 + OFFSET_Y + 10);

    // grids
    for y in 0..h {
        for x in 0..w {
            doc = doc.add(
                Rectangle::new()
                    .set(
                        "fill",
                        if board[y][x] == Some(true) {
                            "black"
                        } else {
                            "white"
                        },
                    )
                    .set("x", x as i32 * D + OFFSET_X)
                    .set("y", y as i32 * D + OFFSET_Y)
                    .set("width", D)
                    .set("height", D)
                    .set("stroke", "black")
                    .set("stroke-width", 1),
            );
        }
    }

    // crosses
    for y in 0..h {
        for x in 0..w {
            if board[y][x] == Some(false) {
                doc = doc.add(
                    Path::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 0.5)
                        .set(
                            "d",
                            Data::new()
                                .move_to((x as i32 * D + OFFSET_X, y as i32 * D + OFFSET_Y))
                                .line_by((D, D))
                                .move_by((0, -D))
                                .line_by((-D, D)),
                        ),
                );
            }
        }
    }

    // frames
    for y in (0..h).step_by(5) {
        for x in (0..w).step_by(5) {
            doc = doc.add(
                Rectangle::new()
                    .set("fill-opacity", 0)
                    .set("x", x as i32 * D + OFFSET_X)
                    .set("y", y as i32 * D + OFFSET_Y)
                    .set("width", D * 5)
                    .set("height", D * 5)
                    .set("stroke", "black")
                    .set("stroke-width", 4),
            );
        }
    }

    // hints
    for (y, hints) in hints.0.iter().enumerate() {
        for (x, num) in hints.iter().enumerate() {
            doc = doc.add(
                Text::new()
                    .set("x", OFFSET_X - D * (hints.len() - 1 - x) as i32 - D / 2)
                    .set("y", OFFSET_Y + D * y as i32 + D / 2)
                    .set(
                        "fill",
                        if hints_hidden.0[y][x] {
                            "lightgray"
                        } else {
                            "black"
                        },
                    )
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "middle")
                    .add(svg::node::Text::new(format!("{}", num))),
            );
        }
    }

    for (x, hints) in hints.1.iter().enumerate() {
        for (y, num) in hints.iter().enumerate() {
            doc = doc.add(
                Text::new()
                    .set("x", OFFSET_X + D * x as i32 + D / 2)
                    .set("y", OFFSET_Y - D * (hints.len() - 1 - y) as i32 - D / 2)
                    .set(
                        "fill",
                        if hints_hidden.1[x][y] {
                            "lightgray"
                        } else {
                            "black"
                        },
                    )
                    .set("text-anchor", "middle")
                    .add(svg::node::Text::new(format!("{}", num))),
            );
        }
    }

    doc.to_string()
}

pub fn vis_cursor_inner(h: usize, w: usize, y: usize, x: usize) -> String {
    let mut doc = Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                -5,
                -5,
                D * w as i32 + OFFSET_X + 10,
                D * h as i32 + OFFSET_Y + 10,
            ),
        )
        .set("width", D * w as i32 + OFFSET_X + 10)
        .set("height", D * h as i32 + OFFSET_Y + 10);

    // cursor
    doc = doc.add(
        Rectangle::new()
            .set("x", x as i32 * D + OFFSET_X)
            .set("y", y as i32 * D + OFFSET_Y)
            .set("width", D)
            .set("height", D)
            .set("fill-opacity", 0)
            .set("stroke", THEME)
            .set("stroke-width", 4),
    );

    // highlights
    doc = doc.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", D * y as i32 + OFFSET_Y)
            .set("width", D * w as i32 + OFFSET_X)
            .set("height", D)
            .set("fill", THEME)
            .set("fill-opacity", 0.2)
            .set("stroke", THEME)
            .set("stroke-width", 0),
    );
    doc = doc.add(
        Rectangle::new()
            .set("x", D * x as i32 + OFFSET_X)
            .set("y", 0)
            .set("width", D)
            .set("height", D * h as i32 + OFFSET_X)
            .set("fill", THEME)
            .set("fill-opacity", 0.2)
            .set("stroke", THEME)
            .set("stroke-width", 0),
    );

    doc.to_string()
}

pub fn set_inner(
    y: usize,
    x: usize,
    val: Option<bool>,
    board: &mut Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
) {
    let (h, w) = (board.len(), board[0].len());

    board[y][x] = val;

    // y軸方向
    let true_count = (0..w).filter(|&x| board[y][x] == Some(true)).count();
    let none_count = (0..w).filter(|&x| board[y][x].is_none()).count();
    if true_count > hints.0[y].iter().sum::<i32>() as usize || none_count == 0 {
    } else {
        // (1) None <- true として完全一致か？
        let line = (0..w).map(|x| board[y][x] != Some(false)).collect_vec();
        let filled = compress(&line);
        if filled == hints.0[y] {
            for x in 0..w {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(true), board, hints);
                }
            }
        }
        // (2) None <- false として完全一致か？
        let line = (0..w).map(|x| board[y][x] == Some(true)).collect_vec();
        let filled = compress(&line);
        if filled == hints.0[y] {
            for x in 0..w {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(false), board, hints);
                }
            }
        }
        // (3) 前から見てどこまで一致するか？
        if (0..w).any(|x| board[y][x].is_none()) {
            let mut x = 0;
            let mut i = 0;
            while x < w && i < hints.0[y].len() {
                let mut nx = x;
                while nx < w && board[y][x] == board[y][nx] {
                    nx += 1;
                }
                let len = (nx - x) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.0[y][i] {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if x > 0 && board[y][x - 1] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                x = nx;
            }
        }
        // (4) 後から見てどこまで一致するか？
        if (0..w).any(|x| board[y][x].is_none()) {
            let mut x = w - 1;
            let mut i = hints.0[y].len() - 1;
            while x < w && i < hints.0[y].len() {
                let mut nx = x;
                while nx < w && board[y][x] == board[y][nx] {
                    nx -= 1;
                }
                let len = (x - nx) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.0[y][i] {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if x + 1 < w && board[y][x + 1] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                x = nx;
            }
        }
    }

    // x軸方向
    let true_count = (0..h).filter(|&y| board[y][x] == Some(true)).count();
    let none_count = (0..h).filter(|&y| board[y][x].is_none()).count();
    if true_count > hints.1[x].iter().sum::<i32>() as usize || none_count == 0 {
    } else {
        // (1) None <- true として完全一致か？
        let line = (0..h).map(|y| board[y][x] != Some(false)).collect_vec();
        let filled = compress(&line);
        if filled == hints.1[x] {
            for y in 0..h {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(true), board, hints);
                }
            }
        }
        // (2) None <- false として完全一致か？
        let line = (0..h).map(|y| board[y][x] == Some(true)).collect_vec();
        let filled = compress(&line);
        if filled == hints.1[x] {
            for y in 0..h {
                if board[y][x].is_none() {
                    set_inner(y, x, Some(false), board, hints);
                }
            }
        }
        // (3) 前から見てどこまで一致するか？
        if (0..h).any(|y| board[y][x].is_none()) {
            let mut y = 0;
            let mut i = 0;
            while y < h && i < hints.1[x].len() {
                let mut ny = y;
                while ny < h && board[y][x] == board[ny][x] {
                    ny += 1;
                }
                let len = (ny - y) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.1[x][i] {
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if y > 0 && board[y - 1][x] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                y = ny;
            }
        }
        // (4) 後から見てどこまで一致するか？
        if (0..h).any(|y| board[y][x].is_none()) {
            let mut y = h - 1;
            let mut i = hints.1[x].len() - 1;
            while y < h && i < hints.1[x].len() {
                let mut ny = y;
                while ny < h && board[y][x] == board[ny][x] {
                    ny -= 1;
                }
                let len = (y - ny) as i32;
                match board[y][x] {
                    Some(true) => {
                        if len != hints.1[x][i] {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                    Some(false) => {}
                    None => {
                        if y + 1 < h && board[y + 1][x] == Some(true) {
                            set_inner(y, x, Some(false), board, hints);
                        }
                        break;
                    }
                }
                y = ny;
            }
        }
    }
}

fn compress(vec: &[bool]) -> Vec<i32> {
    let mut ret = Vec::<i32>::new();
    let mut i = 0;
    while i < vec.len() {
        let mut ni = i;
        while ni < vec.len() && vec[i] == vec[ni] {
            ni += 1;
        }
        if vec[i] {
            ret.push((ni - i) as i32);
        }
        i = ni;
    }
    ret
}

fn get_hints_hidden(
    board: &Vec<Vec<Option<bool>>>,
    hints: &(Vec<Vec<i32>>, Vec<Vec<i32>>),
) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let (h, w) = (board.len(), board[0].len());
    let mut hints_hidden = (
        hints
            .0
            .iter()
            .map(|hints| hints.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        hints
            .1
            .iter()
            .map(|hints| hints.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    for y in 0..h {
        // y軸方向
        let true_count = (0..w).filter(|&x| board[y][x] == Some(true)).count();
        let none_count = (0..w).filter(|&x| board[y][x].is_none()).count();
        if true_count > hints.0[y].iter().sum::<i32>() as usize {
            // 全部間違っていることにする
            for hidden in &mut hints_hidden.0[y] {
                *hidden = false;
            }
        } else if none_count == 0 {
            let line = (0..w).map(|x| board[y][x].unwrap()).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                // 全部正しいことにする
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            } else {
                // 全部間違っていることにする
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = false;
                }
            }
        } else {
            // (1) None <- true として完全一致か？
            let line = (0..w).map(|x| board[y][x] != Some(false)).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            }
            // (2) None <- false として完全一致か？
            let line = (0..w).map(|x| board[y][x] == Some(true)).collect_vec();
            let filled = compress(&line);
            if filled == hints.0[y] {
                for hidden in &mut hints_hidden.0[y] {
                    *hidden = true;
                }
            }
            // (3) 前から見てどこまで一致するか？
            if (0..w).any(|x| board[y][x].is_none()) {
                let mut x = 0;
                let mut i = 0;
                while x < w && i < hints.0[y].len() {
                    let mut nx = x;
                    while nx < w && board[y][x] == board[y][nx] {
                        nx += 1;
                    }
                    let len = (nx - x) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.0[y][i] {
                                break;
                            } else {
                                hints_hidden.0[y][i] = true;
                                i += 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    x = nx;
                }
            }
            // (4) 後から見てどこまで一致するか？
            if (0..w).any(|x| board[y][x].is_none()) {
                let mut x = w - 1;
                let mut i = hints.0[y].len() - 1;
                while x < w && i < hints.0[y].len() {
                    let mut nx = x;
                    while nx < w && board[y][x] == board[y][nx] {
                        nx -= 1;
                    }
                    let len = (x - nx) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.0[y][i] {
                                break;
                            } else {
                                hints_hidden.0[y][i] = true;
                                i -= 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    x = nx;
                }
            }
        }
    }
    for x in 0..w {
        let true_count = (0..h).filter(|&y| board[y][x] == Some(true)).count();
        let none_count = (0..h).filter(|&y| board[y][x].is_none()).count();
        if true_count > hints.1[x].iter().sum::<i32>() as usize {
            for hidden in &mut hints_hidden.1[x] {
                *hidden = false;
            }
        } else if none_count == 0 {
            let line = (0..h).map(|y| board[y][x].unwrap()).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                // 全部正しいことにする
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            } else {
                // 全部間違っていることにする
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = false;
                }
            }
        } else {
            // (1) None <- true として完全一致か？
            let line = (0..h).map(|y| board[y][x] != Some(false)).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            }
            // (2) None <- false として完全一致か？
            let line = (0..h).map(|y| board[y][x] == Some(true)).collect_vec();
            let filled = compress(&line);
            if filled == hints.1[x] {
                for hidden in &mut hints_hidden.1[x] {
                    *hidden = true;
                }
            }
            // (3) 前から見てどこまで一致するか？
            if (0..h).any(|y| board[y][x].is_none()) {
                let mut y = 0;
                let mut i = 0;
                while y < h && i < hints.1[x].len() {
                    let mut ny = y;
                    while ny < h && board[y][x] == board[ny][x] {
                        ny += 1;
                    }
                    let len = (ny - y) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.1[x][i] {
                                break;
                            } else {
                                hints_hidden.1[x][i] = true;
                                i += 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    y = ny;
                }
            }
            // (4) 後から見てどこまで一致するか？
            if (0..h).any(|y| board[y][x].is_none()) {
                let mut y = h - 1;
                let mut i = hints.1[x].len() - 1;
                while y < h && i < hints.1[x].len() {
                    let mut ny = y;
                    while ny < h && board[y][x] == board[ny][x] {
                        ny -= 1;
                    }
                    let len = (y - ny) as i32;
                    match board[y][x] {
                        Some(true) => {
                            if len != hints.1[x][i] {
                                break;
                            } else {
                                hints_hidden.1[x][i] = true;
                                i -= 1;
                            }
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    y = ny;
                }
            }
        }
    }

    hints_hidden
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
