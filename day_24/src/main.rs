#![allow(unused_assignments)]

fn monad(input: &[u32; 14]) -> i32 {
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    w = input[0] as i32;
    x = 14;
    x = if x == w { 0 } else { 1 };
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = w;
    y += 12;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[1] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 13;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[2] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 12;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[3] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 14;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 5;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[4] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 13;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[5] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -7;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[6] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -13;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 15;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[7] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 10;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 14;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[8] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -7;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[9] as i32;
    x = 0;
    x += z;
    x %= 26;
    x += 11;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 14;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[10] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[11] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -2;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 5;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[12] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 14;
    y *= x;
    z += y;

    println!("{}", z);

    w = input[13] as i32;
    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -14;
    x = if x == w { 0 } else { 1 };
    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y = 0;
    y += w;
    y += 4;
    y *= x;
    z += y;

    println!("{}", z);

    z
}

fn find_largest_monad_aux(input: &mut [u32; 14], char_index: usize) -> bool {
    if char_index == input.len() {
        monad(input) == 0
    } else {
        for c in (1..=9).rev() {
            input[char_index] = c;

            if char_index == 6 {
                println!("{:?}", input);
            }
            if find_largest_monad_aux(input, char_index + 1) {
                return true;
            }
        }
        false
    }
}

fn find_smallest_monad_aux(input: &mut [u32; 14], char_index: usize) -> bool {
    if char_index == input.len() {
        monad(input) == 0
    } else {
        for c in (1..=9) {
            input[char_index] = c;

            if char_index == 6 {
                println!("{:?}", input);
            }
            if find_smallest_monad_aux(input, char_index + 1) {
                return true;
            }
        }
        false
    }
}

// fn main() {
//     let mut monad = [9; 14];

//     find_largest_monad_aux(&mut monad, 0);

//     println!("Last {:?}", monad);
// }

fn main() {
    // let input = [9, 9, 7, 9, 9, 2, 1, 2, 9, 4, 9, 9, 6, 7];
    let input = [3, 4, 1, 9, 8, 1, 1, 1, 8, 1, 6, 3, 1, 1];
    monad(&input);
}
