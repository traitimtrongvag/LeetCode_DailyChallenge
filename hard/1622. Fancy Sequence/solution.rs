const MOD: i64 = 1_000_000_007;

struct Fancy {
    nums: Vec<i64>,
    mul: i64,
    add: i64,
}

impl Fancy {

    fn new() -> Self {
        Fancy {
            nums: Vec::new(),
            mul: 1,
            add: 0,
        }
    }
    
    fn append(&mut self, val: i32) {
        let v = val as i64;
        let inv = Self::mod_inv(self.mul);
        let stored = ((v - self.add + MOD) % MOD * inv) % MOD;
        self.nums.push(stored);
    }
    
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }
    
    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mul = (self.mul * m) % MOD;
        self.add = (self.add * m) % MOD;
    }
    
    fn get_index(&self, idx: i32) -> i32 {
        let i = idx as usize;
        if i >= self.nums.len() {
            return -1;
        }

        let v = self.nums[i];
        ((v * self.mul + self.add) % MOD) as i32
    }

    fn mod_pow(mut a: i64, mut e: i64) -> i64 {
        let mut res = 1;
        while e > 0 {
            if e & 1 == 1 {
                res = res * a % MOD;
            }
            a = a * a % MOD;
            e >>= 1;
        }
        res
    }

    fn mod_inv(x: i64) -> i64 {
        Self::mod_pow(x, MOD - 2)
    }
}