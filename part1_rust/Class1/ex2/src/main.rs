// Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
// Ví dụ: let input = “adbcdaDd”.

// Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”

// Nhập s = ‘d’ => in ra kết quả : 4, “abca”

use std::io;

fn main() {
    let input = "adbcdaDd";
    println!("Enter your character: ");
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();
    let c_lowercase = s.trim().to_lowercase();
    let c_uppercase = c_lowercase.to_uppercase();
    let count = input.matches(&c_lowercase).count() + input.matches(&c_uppercase).count();
    let result_1 = input.replace(&c_lowercase, "");
    let result_2 = result_1.replace(&c_uppercase, "");
    println!("{},{}", count, result_2);
}
