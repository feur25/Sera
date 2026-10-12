use std::time::Instant;

pub struct Bucket {
    max_per_sec: f64,
    tokens: f64,
    last_refill: Instant,
}

impl Bucket {
    pub fn new(max_per_sec: f64) -> Self {
        let max_per_sec = max_per_sec.max(0.0);
        let capacity = if max_per_sec <= 0.0 { 0.0 } else { max_per_sec.max(1.0) };
        Bucket { max_per_sec, tokens: capacity, last_refill: Instant::now() }
    }

    pub fn try_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.last_refill = now;
        self.tokens = (self.tokens + elapsed * self.max_per_sec).min(self.max_per_sec.max(1.0));
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caps_at_configured_rate() {
        let mut b = Bucket::new(3.0);
        let mut allowed = 0;
        for _ in 0..10 {
            if b.try_consume() {
                allowed += 1;
            }
        }
        assert_eq!(allowed, 3);
    }

    #[test]
    fn refills_over_time() {
        let mut b = Bucket::new(5.0);
        for _ in 0..5 {
            b.try_consume();
        }
        std::thread::sleep(std::time::Duration::from_millis(400));
        assert!(b.try_consume());
    }
}
