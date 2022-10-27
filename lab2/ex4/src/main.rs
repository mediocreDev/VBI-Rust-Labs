// ====================================
// BÀI 4:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================

// Solution 1: Refacter code
// fn main() {
//     let mut v = vec![1, 2, 3];
//     go(&v);

//     // still need v here, so I can't pass ownership to the "go' method above
//     v.push(4);
//     println!("{}", v.len())
// }

// fn go(v: &Vec<i32>) {
//     for i in v {
//         println!("{}", i);
//     }
// }

// Solution 2: Use slice
fn main() {
    let mut v = vec![1, 2, 3];

    go(&mut v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

fn go(v: &mut Vec<i32>) {
    for i in &*v {
        // create a slice of the value which is pointed by v and iterate it
        // that slice is an immutable slice which will be drop when out of for scope
        // , so the mutable reference is still valid
        println!("{}", i);
    }
    v.push(4);
}
