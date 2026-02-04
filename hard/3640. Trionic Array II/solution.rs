impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let len = nums.len();

        // states for the three segments of a trionic pattern
        let mut prev_state: (Option<i64>, Option<i64>, Option<i64>) = (None, None, None);
        let mut curr_state: (Option<i64>, Option<i64>, Option<i64>) = (None, None, None);

        // best completed trionic sum so far
        let mut res: Option<i64> = None;

        for i in 1..len {
            // first increasing segment
            curr_state.0 = match prev_state.0 {
                None => {
                    if nums[i - 1] < nums[i] {
                        Some((nums[i - 1] + nums[i]) as i64)
                    } else {
                        None
                    }
                }
                Some(x) => {
                    if nums[i - 1] < nums[i] {
                        Some((nums[i - 1] as i64).max(x) + nums[i] as i64)
                    } else {
                        None
                    }
                }
            };

            // decreasing segment
            curr_state.1 = match prev_state.1 {
                None => {
                    if nums[i - 1] > nums[i] {
                        match prev_state.0 {
                            None => None,
                            Some(x) => Some(x + nums[i] as i64)
                        }
                    } else {
                        None
                    }
                }
                Some(x) => {
                    if nums[i - 1] > nums[i] {
                        Some(x + nums[i] as i64)
                    } else {
                        None
                    }
                }
            };

            // second increasing segment
            curr_state.2 = match prev_state.2 {
                None => {
                    if nums[i - 1] < nums[i] {
                        match prev_state.1 {
                            None => None,
                            Some(x) => Some(x + nums[i] as i64)
                        }
                    } else {
                        None
                    }
                }
                Some(x) => {
                    if nums[i - 1] < nums[i] {
                        Some(x + nums[i] as i64)
                    } else {
                        None
                    }
                }
            };

            // update answer when finishing the pattern
            match curr_state.2 {
                Some(x) => {
                    res = match res {
                        Some(some_res) => Some(some_res.max(x)),
                        _ => Some(x),
                    }
                }
                None => {}
            };

            prev_state = curr_state;
        }

        match res {
            Some(some_res) => some_res,
            None => unreachable!(),
        }
    }
}