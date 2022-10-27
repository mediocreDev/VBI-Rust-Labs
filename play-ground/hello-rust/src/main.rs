fn main() {
  /* Playing with variables, constants - START */
  let x = 5;
  // x = 1;  // error[E0384]: cannot assign twice to immutable variable `x`
  let mut y;
  y = 2;
  y = 3;
  let user_name = "hq.my";  // snake_case => variable convention

  const A_CONSTANT: i32 = 999;  // SNAKE_CASE => constant convention

  let x = "Test"; // shadowing

  let a = 5;
  let b = 5i8;
  let c: i16 = 5;
  let d: u8 = 5;

  let e = 1.2;
  let f = 1.2f32;

  let h = 'x';
  let i: char = '😎';

  let arr: [i8;3] = [1,2,3];
  let [k,l,m] = arr;

  let tup: (i8, f32, &str) = (1, 1.2, "test");
  let (o,p,q) = tup;

  let parentArr: [i32; 4] = [1, 2, 3, 4]; // Parent Array
  let slice1: &[i32] = &parentArr; // Slicing whole array
  let slice2 = &parentArr[0..4]; // From 0th position to 4th(excluding)
  let slice3 = &parentArr[..]; // Slicing whole array
  let slice4 = &parentArr[1..3]; // [2, 3]
  let slice5 = &parentArr[1..]; // [2, 3, 4]
  let slice6 = &parentArr[..3]; // [1, 2, 3]

  /*
    Question:
      - const vs. immutable
      - shadowing vs. mutable
      - 
  */
  /* Playing with variables, constants - END */
  println!("{:?}", arr);
  println!("{}", arr[0]);

  println!("{:?}", tup);
  println!("{}", tup.0);

  print!("\x1B[2J\x1B[1;1H");println!("\nPlayground ends normally\n");
}
