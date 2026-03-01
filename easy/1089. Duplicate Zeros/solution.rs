impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let mut temp = Vec::with_capacity(n);

        // Build new array with duplicated zeros
        for &x in arr.iter() {
            if temp.len() == n {
                break;
            }

            temp.push(x);

            if x == 0 && temp.len() < n {
                temp.push(0);
            }
        }

        // Copy back to original array
        for i in 0..n {
            arr[i] = temp[i];
        }
    }
}