///////////////////////////////////////////
// BAI 1
// Yêu cầu :
// + Sửa code liên quan tới vấn đề generic type (thay đổi ở định nghĩa struct)
///////////////////////////////////////////

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point {
//         x: 5,
//         y: "hello".to_string(),
//     };

//     println!("Success!");
// }

///////////////////////////////////////////
// BAI 2
// Yêu cầu :
// + Implement hàm sum dưới đây, sao cho việc kiểm tra assert_eq đúng
///////////////////////////////////////////

//Implement the generic function below.
// use std::ops::Add;

// fn sum<T>(x: T, y: T) -> T
// where
//     T: Copy + Add<T, Output = T>,
// {
//     x + y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }

///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng
///////////////////////////////////////////

// fn main() {
//     let a = A {
//         p: Some("p".to_string()),
//     };
//     a.a();
// }

// struct A {
//     p: Option<String>,
// }

// impl A {
//     fn a(self) -> Self {
//         self.b();
//         self
//     }
//     fn b(&self) {
//         print!("b: {}", self.p.as_ref().unwrap())
//     }
// }

///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng
///////////////////////////////////////////

// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }

// fn main() {
//     let d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };

//     let x = d.get_val1();
//     let y = d.get_val2();
//     let both = &d.get_both();
// }

// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1.clone();
//     }

//     pub fn get_val2(&self) -> String {
//         return self.val2.clone();
//     }

//     pub fn get_both(self) -> (i32, String) {
//         let mut myData = self;
//         return (myData.val1, myData.val2);
//     }
// }
