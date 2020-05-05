pub struct Fibonacci {
    a1: f32,
    a2: f32,
    count: u32,
    size: u32,
}

impl Fibonacci {
    pub fn new(size: u32) -> Fibonacci {
        Fibonacci {
            a1: 0.0,
            a2: 1.0,
            count: 0,
            size,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.size {
            return None;
        }

        let res = match self.count {
            0 => self.a1,
            _ => {
                let res = self.a1 + self.a2;
                self.a1 = self.a2;
                self.a2 = res;
                res
            }
        };
        self.count += 1;
        Some(res)
    }
}
