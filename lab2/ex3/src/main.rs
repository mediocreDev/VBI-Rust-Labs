// ====================================// BÀI 3:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================

fn main() {}

pub fn iter_num(num: i32) -> bool {
    let num_str = num.to_string();
    let chars = num_str.chars();
    let len = chars.count();
    // Solution 1: replace line 12 to this
    // let len = chars.clone().count();

    println!("Len = {:?}", len);

    // Solution 2: keep line 12, recall chars() to make a new iterator
    // for c in chars {
    for c in num_str.chars() {
        println!(">>> {:?}", c);
    }

    return true;
}
