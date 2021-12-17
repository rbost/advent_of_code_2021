fn is_in_trench(x_range: (i32, i32), y_range: (i32, i32), (x, y): (i32, i32)) -> bool {
    ((x_range.0 <= x) && (x <= x_range.1)) && ((y_range.0 <= y) && (y <= y_range.1))
}

fn gen_trajectory(
    (vx, vy): (i32, i32),
    x_trench_range: (i32, i32),
    y_trench_range: (i32, i32),
) -> Vec<((i32, i32), (i32, i32))> {
    let (mut vx, mut vy) = (vx, vy);
    let (mut x, mut y) = (0, 0);
    (0..)
        .map(|_iter| {
            let res = ((x, y), (vx, vy));
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1
            };
            vy -= 1;

            res
        })
        .take_while(|((_x, y), _)| *y >= y_trench_range.0.min(y_trench_range.1))
        .take_while(|((x, _y), _)| *x <= x_trench_range.0.max(x_trench_range.1))
        .collect()
}

fn intersects(
    trajectory: &[((i32, i32), (i32, i32))],
    x_trench_range: (i32, i32),
    y_trench_range: (i32, i32),
) -> bool {
    is_in_trench(x_trench_range, y_trench_range, trajectory.last().unwrap().0)
}

fn vertical_velocity_ok(v: i32, min: i32, max: i32) -> bool {
    let mut x = 0;
    let mut v = v;
    while x <= max {
        if x >= min {
            return true;
        }
        x += v;
        v += 1;
    }
    false
}

fn main() {
    let (x_min, x_max) = (57, 116);
    let (y_min, y_max) = (-198, -148);
    // let (x_min, x_max): (i32, i32) = (20, 30);
    // let (y_min, y_max): (i32, i32) = (-10, -5);

    let rev_bb_min = -y_max;
    let rev_bb_max = -y_min;

    let vx_max = (0..=x_max)
        .filter(|v| vertical_velocity_ok(*v, x_min, x_max))
        .max()
        .unwrap();
    let vy_max = (0..=rev_bb_max)
        .filter(|v| vertical_velocity_ok(*v, rev_bb_min, rev_bb_max))
        .max()
        .unwrap();

    println!("vx_max {:?}", vx_max);
    println!("vy_max {:?}", vy_max);

    let count = (1..=vx_max)
        .map(|vx| {
            (-vy_max..=vy_max)
                .filter(move |vy| {
                    let traj =
                        gen_trajectory((vx as i32, *vy as i32), (x_min, x_max), (y_min, y_max));
                    intersects(&traj, (x_min, x_max), (y_min, y_max))
                })
                .count()
        })
        .sum::<usize>();

    println!("count {:?}", count);
}
