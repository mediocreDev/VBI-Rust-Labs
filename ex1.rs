fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let mut i = 0;
    while i < org_arr.len() {
        let mut j = 0;
        while j < sub_arr.len() {
            if org_arr[i + j] == sub_arr[j] {
                j = j + 1;
            } else {
                break;
            }
        }
        if j == sub_arr.len() {
            println!("Existed");
            break;
        } else {
            i = i + 1;
        }
    }
    println!("No existance");
}
