mod part1;
mod part2;

pub struct SplitSlice<'a> {
    slices: (&'a [i32], &'a [i32]),
    idx: usize,
}

impl<'a> SplitSlice<'a> {
    pub fn from_split_at(src: &'a [i32], at: usize) -> Self {
        let splt = src.split_at(at);
        SplitSlice {
            slices: (splt.0, splt.1),
            idx: 0,
        }
    }
    pub fn from_slice(src: &'a [i32]) -> Self {
        SplitSlice {
            slices: (src, &[]),
            idx: 0,
        }
    }
}

pub fn skip_at_slice(slc: &[i32], at: usize) -> SplitSlice {
    if at == 0 {
        return SplitSlice::from_slice(&slc[1..]);
    }

    if at == slc.len() - 1 {
        return SplitSlice::from_slice(&slc[..at]);
    }

    let (s0, s1) = slc.split_at(at);
    SplitSlice {
        slices: (s0, &s1[1..]),
        idx: 0,
    }
}

impl<'a> Iterator for SplitSlice<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let l0 = self.slices.0.len();
        let l1 = self.slices.1.len();
        if self.idx < l0 {
            self.idx += 1;
            return Some(self.slices.0[self.idx - 1]);
        }
        let idx = self.idx - l0;
        if idx < l1 {
            self.idx += 1;
            return Some(self.slices.1[idx]);
        }
        None
    }
}

pub fn record_is_safe(levels: impl IntoIterator<Item = i32>) -> bool {
    let mut levels = levels.into_iter();
    let mut l0 = levels.next().unwrap();
    let mut l1 = levels.next().unwrap();
    let d0 = l1 - l0;
    if d0.abs() > 3 || d0 == 0 {
        return false;
    }
    for l in levels {
        l0 = l1;
        l1 = l;
        if l1 > l0 && d0 < 0 || l1 < l0 && d0 > 0 || l1 == l0 {
            return false;
        }
        let d = l1.abs_diff(l0);
        if !(1..=3).contains(&d) {
            return false;
        }
    }
    true
}

pub use part1::part1;
pub use part2::part2;
