// idea:
// - sort by position
// - stack keeps indices of 'R'
// - when 'L' comes → fight with stack top

impl Solution {
    pub fn survived_robots_healths(pos: Vec<i32>, mut hp: Vec<i32>, dir: String) -> Vec<i32> {
        let mut idx: Vec<usize> = (0..pos.len()).collect();
        let d = dir.as_bytes();

        idx.sort_unstable_by_key(|&i| pos[i]);

        let mut st: Vec<usize> = Vec::new();

        for &i in &idx {
            if d[i] == b'R' {
                st.push(i);
                continue;
            }

            while let Some(&j) = st.last() {
                if d[j] == b'L' { break; }

                if hp[j] < hp[i] {
                    st.pop();
                    hp[i] -= 1;
                } else if hp[j] == hp[i] {
                    st.pop();
                    hp[i] = 0;
                    break;
                } else {
                    hp[j] -= 1;
                    hp[i] = 0;
                    break;
                }
            }

            if hp[i] > 0 {
                st.push(i);
            }
        }

        st.sort_unstable();
        st.into_iter().map(|i| hp[i]).collect()
    }
}