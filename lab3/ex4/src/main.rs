///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng
///////////////////////////////////////////


#[derive(Debug, Clone)]
struct MyData {
    val1: i32,
    val2: String,
}


fn main() {
    let d = MyData {
        val1: 35,
        val2: String::from("Hello World"),
    };

    let both = d.get_both();
    let x = d.get_val1();
    let y = d.get_val2();
    // println!("{:?} {} {}", both, x, y);
}


impl MyData {
    pub fn get_val1(&self) -> &i32 {
        return &self.val1;
    }

    pub fn get_val2(&self) -> &str {
        return &self.val2;
    }

    pub fn get_both(&self) -> (&i32, &str) {
        return (&self.val1, &self.val2);
    }
}