// Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//         let sub_arr = [6,8,10];

fn is_sub_array(org: &[i32], sub: &[i32]) -> bool {
    let mut i = 0;
    let mut j = 0;

    while i < org.len() && j < sub.len() {
        if org[i] == sub[j] {
            i += 1;
            j += 1;

            if j == sub.len() {
                return true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }

    return false;
}

fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
    let result = is_sub_array(&org_arr, &sub_arr);
    if result {
        println!("array 2 is subarray of array 1");
    } else {
        println!("array 2 is not subarray of array 1");
    }
}
