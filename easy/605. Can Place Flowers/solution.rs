impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let len = flowerbed.len();

        for i in 0..len {
            if flowerbed[i] == 0 {
                let left_empty = i == 0 || flowerbed[i - 1] == 0;
                let right_empty = i == len - 1 || flowerbed[i + 1] == 0;

                if left_empty && right_empty {
                    flowerbed[i] = 1;
                    count += 1;

                    if count >= n {
                        return true;
                    }
                }
            }
        }

        count >= n
    }
}