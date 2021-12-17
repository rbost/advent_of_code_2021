fn vertical_velocity_ok(v: u32, min: u32, max: u32) -> bool {
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
    let (y_min, y_max) = (-198, -148);
    // let (y_min, y_max) = (-10, -5);

    let rev_bb_min = -y_max as u32;
    let rev_bb_max = -y_min as u32;

    let v_max = (0..rev_bb_max)
        .filter(|v| vertical_velocity_ok(*v, rev_bb_min, rev_bb_max))
        .max()
        .unwrap();

    println!("y_max {:?}", (v_max + 1) * v_max / 2);
}
