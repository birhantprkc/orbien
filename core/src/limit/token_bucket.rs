use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct BandwidthLimiter {
    rate: f64,

    burst: f64,
    state: Mutex<State>,
}

struct State {
    tokens: f64,
    last: Instant,
}

impl BandwidthLimiter {
    pub fn new(bytes_per_sec: u64) -> Self {
        let b = bytes_per_sec.max(1) as f64;
        Self {
            rate: b,
            burst: b,
            state: Mutex::new(State {
                tokens: b,
                last: Instant::now(),
            }),
        }
    }

    pub fn burst(&self) -> usize {
        self.burst as usize
    }

    pub fn bytes_per_sec(&self) -> u64 {
        self.rate as u64
    }

    pub fn try_wait_n(&self, n: usize) -> bool {
        if n == 0 {
            return true;
        }
        let n = n as f64;
        if n > self.burst + f64::EPSILON {
            return false;
        }
        let mut st = self.state.lock().unwrap_or_else(|p| p.into_inner());
        st.refill(self.rate, self.burst);
        if st.tokens >= n {
            st.tokens -= n;
            true
        } else {
            false
        }
    }

    pub async fn wait_n(&self, n: usize) {
        if n == 0 {
            return;
        }
        let n = n as f64;
        debug_assert!(n <= self.burst + f64::EPSILON);

        loop {
            let sleep_for = {
                let mut st = self.state.lock().unwrap_or_else(|p| p.into_inner());
                st.refill(self.rate, self.burst);
                if st.tokens >= n {
                    st.tokens -= n;
                    return;
                }
                let need = n - st.tokens;
                Duration::from_secs_f64(need / self.rate)
            };
            tokio::time::sleep(sleep_for).await;
        }
    }
}

impl State {
    fn refill(&mut self, rate: f64, burst: f64) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens = (self.tokens + elapsed * rate).min(burst);
    }
}
