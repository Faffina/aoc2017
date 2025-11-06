pub fn first() {
    let mut i = 1;
    let target = 312051;
    while target > i * i {
        i += 2;
    }
    let mut distance:i64 = i / 2;

    let bottom_right = i * i;
    let bottom_left = bottom_right - i + 1;
    let top_left = bottom_left - i + 1;
    let top_right = top_left - i + 1;
    let botoom_right_alt = top_right - i + 2;

    let i = i / 2;
    let mid_bottom = bottom_right - i;
    let mid_left = bottom_left - i;
    let mid_top = top_left - i;
    let mid_right = top_right - i;

    let sides = [
        (bottom_right, mid_bottom, bottom_left),
        (bottom_left, mid_left, top_left),
        (top_left, mid_top, top_right),
        (top_right, mid_right, botoom_right_alt),
    ];

    for (left, mid, right) in sides {
        if left >= target && target >= right {
            distance += (target - mid).abs();
        }
    }

    println!("part one: {distance}");
}

pub fn second() {
    let target = 312051;
    const GRID_SIZE: usize = 101;
    let mut grid = vec![vec![0u64; GRID_SIZE]; GRID_SIZE];
    let mut x = GRID_SIZE / 2;
    let mut y = GRID_SIZE / 2;
    grid[y][x] = 1;

    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)]; 
    let mut step = 1;

    'outer: loop {
        for d in 0..4 {
            let (dx, dy) = directions[d];
            for _ in 0..step {
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;

                let mut sum = 0;
                for ny in y.saturating_sub(1)..=y + 1 {
                    for nx in x.saturating_sub(1)..=x + 1 {
                        if nx == x && ny == y { continue; }
                        if ny < GRID_SIZE && nx < GRID_SIZE {
                            sum += grid[ny][nx];
                        }
                    }
                }

                grid[y][x] = sum;

                if sum > target {
                    println!("first value larger than {} is {}", target, sum);
                    break 'outer;
                }
            }

            if d % 2 == 1 {
                step += 1;
            }
        }
    }
}
