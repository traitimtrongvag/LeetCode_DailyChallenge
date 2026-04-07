// perimeter length = 2*(w+h)-4
// num = current step index on border (1-based)

struct Robot {
    w: i32,
    h: i32,
    p: i32, // position index on cycle
}

impl Robot {
    fn new(w: i32, h: i32) -> Self {
        Self { w, h, p: 0 }
    }

    fn step(&mut self, k: i32) {
        let cycle = self.w * 2 + self.h * 2 - 4;
        self.p = (self.p + k + cycle - 1) % cycle + 1;
    }

    fn get_pos(&self) -> Vec<i32> {
        let (w, h, p) = (self.w, self.h, self.p);

        if p < w {
            vec![p, 0]
        } else if p < w + h - 1 {
            vec![w - 1, p - w + 1]
        } else if p < 2 * w + h - 2 {
            vec![2 * w + h - 3 - p, h - 1]
        } else {
            vec![0, 2 * w + 2 * h - 4 - p]
        }
    }

    fn get_dir(&self) -> String {
        let (w, h, p) = (self.w, self.h, self.p);

        if p == 0 {
            return "East".to_string();
        }

        if p < w {
            "East"
        } else if p < w + h - 1 {
            "North"
        } else if p < 2 * w + h - 2 {
            "West"
        } else {
            "South"
        }
        .to_string()
    }
}