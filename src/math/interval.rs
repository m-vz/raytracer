use std::collections::Bound;
use std::ops::{Range, RangeBounds};

#[derive(Clone)]
pub struct Interval(pub Range<f64>);

impl Interval {
    pub fn start(&self) -> f64 {
        self.0.start
    }

    pub fn end(&self) -> f64 {
        self.0.end
    }

    pub fn set_start(&mut self, start: f64) {
        self.0.start = start;
    }

    pub fn set_end(&mut self, end: f64) {
        self.0.end = end;
    }

    pub fn expand(&mut self, delta: f64) {
        let half_delta = delta / 2.0;

        self.0 = self.start() - half_delta..self.end() + half_delta;
    }

    pub fn expanded(&self, delta: f64) -> Self {
        let half_delta = delta / 2.0;

        Self(self.start() - half_delta..self.end() + half_delta)
    }

    pub fn combine(&mut self, rhs: &Interval) {
        self.set_start(self.start().min(rhs.start()));
        self.set_end(self.end().max(rhs.end()));
    }

    pub fn combined(&self, rhs: &Interval) -> Self {
        Self(self.start().min(rhs.start())..self.end().max(rhs.end()))
    }
}

impl RangeBounds<f64> for Interval {
    fn start_bound(&self) -> Bound<&f64> {
        self.0.start_bound()
    }

    fn end_bound(&self) -> Bound<&f64> {
        self.0.end_bound()
    }

    fn contains<U>(&self, item: &U) -> bool
    where
        f64: PartialOrd<U>,
        U: ?Sized + PartialOrd<f64>,
    {
        self.0.contains(item)
    }
}

impl From<Range<f64>> for Interval {
    fn from(value: Range<f64>) -> Self {
        Self(value)
    }
}

impl From<Interval> for Range<f64> {
    fn from(value: Interval) -> Self {
        value.0
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self(0.0..0.0)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::Interval;

    #[test]
    fn expand() {
        let interval = Interval(0.0..1.0);
        let expanded = interval.expanded(0.1);

        assert_approx_eq!(f64, expanded.start(), -0.05);
        assert_approx_eq!(f64, expanded.end(), 1.05);
    }
}
