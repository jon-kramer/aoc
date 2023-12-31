use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Interval(pub u64, pub u64);

impl Interval {
    pub fn intersect(&self, other: &Interval) -> Option<Interval> {
        let start = max(self.0, other.0);
        let end = min(self.1, other.1);
        if start < end {
            Some(Interval(start, end))
        } else {
            None
        }
    }
    pub fn diff(&self, other: &Interval) -> Vec<Interval> {
        // Three parts:
        //  - Before
        //  - Intersection
        //  - After
        //  Before and After make up the difference
        match self.intersect(other) {
            Some(intersection) => {
                let mut differences = Vec::new();

                if self.0 < intersection.0 {
                    differences.push(Interval(self.0, intersection.0));
                }

                if self.1 > intersection.1 {
                    differences.push(Interval(intersection.1, self.1));
                }

                differences
            }
            None => vec![self.clone()],
        }
    }
}

// impl to create interval IntervalSet
//
// functions to act on intervals themselves?

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use crate::interval::Interval;

    //==========================
    // Interval Intersect Tests
    //==========================

    #[test]
    fn interval_intersect_under_no_overlap() {
        let i1 = Interval(1, 5);
        let i2 = Interval(5, 10);
        let i_res = None;
        assert_eq!(i1.intersect(&i2), i_res);
    }
    #[test]
    fn interval_intersect_under_some_overlap() {
        let i1 = Interval(1, 6);
        let i2 = Interval(5, 10);
        let i_res = Some(Interval(5, 6));
        assert_eq!(i1.intersect(&i2), i_res);
    }
    #[test]
    fn interval_intersect_inside_overlap() {
        let i1 = Interval(1, 5);
        let i2 = Interval(1, 10);
        let i_res = Some(Interval(1, 5));
        assert_eq!(i1.intersect(&i2), i_res);
    }
    #[test]
    fn interval_intersect_super_overlap() {
        let i1 = Interval(1, 10);
        let i2 = Interval(2, 6);
        let i_res = Some(Interval(2, 6));
        assert_eq!(i1.intersect(&i2), i_res);
    }
    #[test]
    fn interval_intersect_over_some_overlap() {
        let i1 = Interval(5, 15);
        let i2 = Interval(1, 10);
        let i_res = Some(Interval(5, 10));
        assert_eq!(i1.intersect(&i2), i_res);
    }
    #[test]
    fn interval_intersect_over_no_overlap() {
        let i1 = Interval(5, 10);
        let i2 = Interval(1, 5);
        let i_res = None;
        assert_eq!(i1.intersect(&i2), i_res);
    }

    //=====================
    // Interval Diff Tests
    //=====================

    #[test]
    fn interval_diff_under_no_overlap() {
        let i1 = Interval(1, 5);
        let i2 = Interval(5, 10);
        let i_res = vec![i1.clone()];
        assert_eq!(i1.diff(&i2), i_res);
    }
    #[test]
    fn interval_diff_under_some_overlap() {
        let i1 = Interval(1, 6);
        let i2 = Interval(5, 10);
        let i_res = vec![Interval(1, 5)];
        assert_eq!(i1.diff(&i2), i_res);
    }
    #[test]
    fn interval_diff_inside_overlap() {
        let i1 = Interval(1, 5);
        let i2 = Interval(1, 10);
        let i_res = vec![];
        assert_eq!(i1.diff(&i2), i_res);
    }
    #[test]
    fn interval_diff_super_overlap() {
        let i1 = Interval(1, 10);
        let i2 = Interval(2, 6);
        let i_res = vec![Interval(1, 2), Interval(6, 10)];
        assert_eq!(i1.diff(&i2), i_res);
    }
    #[test]
    fn interval_diff_over_some_overlap() {
        let i1 = Interval(5, 15);
        let i2 = Interval(1, 10);
        let i_res = vec![Interval(10, 15)];
        assert_eq!(i1.diff(&i2), i_res);
    }
    #[test]
    fn interval_diff_over_no_overlap() {
        let i1 = Interval(5, 10);
        let i2 = Interval(1, 5);
        let i_res = vec![i1.clone()];
        assert_eq!(i1.diff(&i2), i_res);
    }
}
