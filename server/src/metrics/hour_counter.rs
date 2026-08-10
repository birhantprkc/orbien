use chrono::{DateTime, Local, Timelike};
use std::sync::Mutex;

#[derive(Debug)]
pub struct HourCounter {
    inner: Mutex<Inner>,
}

#[derive(Debug)]
struct Inner {
    reserve_hours: usize,
    counts: Vec<i64>,
    last_hour: DateTime<Local>,
}

impl HourCounter {
    pub fn new(reserve_hours: usize) -> Self {
        let reserve_hours = reserve_hours.max(1);
        Self {
            inner: Mutex::new(Inner {
                reserve_hours,
                counts: vec![0; reserve_hours],
                last_hour: truncate_hour(Local::now()),
            }),
        }
    }

    pub fn last_hours(&self, hours: usize) -> Vec<i64> {
        let mut g = self.inner.lock().expect("hour counter lock");
        g.rotate(Local::now());
        let n = hours.min(g.reserve_hours);
        g.counts[..n].to_vec()
    }

    #[allow(dead_code)]
    pub fn current_hour_count(&self) -> i64 {
        self.last_hours(1).first().copied().unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn window_sum(&self, hours: usize) -> i64 {
        self.last_hours(hours).into_iter().sum()
    }

    pub fn inc(&self, delta: i64) {
        let mut g = self.inner.lock().expect("hour counter lock");
        g.rotate(Local::now());
        g.counts[0] = g.counts[0].saturating_add(delta);
    }
}

impl Inner {
    fn rotate(&mut self, now: DateTime<Local>) {
        let current = truncate_hour(now);
        if current <= self.last_hour {
            return;
        }
        let elapsed = current.signed_duration_since(self.last_hour);
        let hours = elapsed.num_hours();
        if hours <= 0 {
            return;
        }
        let shift = hours as usize;
        if shift >= self.reserve_hours {
            self.counts.fill(0);
        } else {
            self.counts.rotate_right(shift);
            for slot in self.counts.iter_mut().take(shift) {
                *slot = 0;
            }
        }
        self.last_hour = current;
    }
}

fn truncate_hour(t: DateTime<Local>) -> DateTime<Local> {
    t.with_minute(0)
        .and_then(|t| t.with_second(0))
        .and_then(|t| t.with_nanosecond(0))
        .unwrap_or(t)
}
