fn main() {
    // let mut x = 20;

    // while x >= 10 {
    //     x = x / 2;
    // }
    // println!("Final value: {x}");

    // // 1..=5 to go until the index 5
    // for x in 1..5 {
    //     println!("x : {x}");
    // }

    // for elem in [1, 2, 3, 4, 5] {
    //     println!("elem: {elem}");
    // }

    // let mut i = 0;
    // loop {
    //     i += 1;
    //     println!("i: {i}");
    //     if i > 10 {
    //         break;
    //     }
    // }
    // i = 0;
    // loop {
    //     i += 1;
    //     if i > 5 {
    //         break;
    //     }
    //     if i % 2 == 0 {
    //         continue;
    //     }
    //     println!("{}", i);
    // }

    // Labels - point which loops should be "break" or "continue"
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}
