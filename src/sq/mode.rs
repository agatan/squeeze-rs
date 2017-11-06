extern crate core;
use self::core::str::StrExt;
use std;

use sq::result;
pub use self::Mode::{Normal, Regexp, Fuzzy};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Regexp,
    Fuzzy,
}

impl Mode {
    pub fn refine_string(&self, target: &str, needle: &str) -> Option<result::Result> {
        match *self {
            Mode::Normal => refine_normal_string(target, needle),
            Mode::Regexp => None,
            Mode::Fuzzy => None,
        }
    }

    pub fn refine_vec(&self, candidates: &[String], needle: &str) -> Vec<result::Result> {
        match *self {
            Mode::Normal => {
                candidates
                    .iter()
                    .flat_map(|s| refine_normal_string(s, needle).into_iter())
                    .collect()
            }
            Mode::Regexp => vec![],
            Mode::Fuzzy => vec![],
        }
    }
}

fn refine_normal_string(target: &str, needle: &str) -> Option<result::Result> {
    let mut idx = 0usize;
    let mut matches = std::collections::BTreeSet::new();
    for n in needle.split_terminator(' ').filter(|s| !s.is_empty()) {
        if let Some(start) = target.slice_chars(idx, target.len()).find_str(n) {
            idx = start;
            for x in idx..idx + n.len() {
                matches.insert(x);
            }
        }
    }
    if matches.is_empty() {
        None
    } else {
        Some(result::Result::new(target, matches))
    }
}

#[cfg(test)]
mod test {
    use sq::result;
    use super::*;
    use std;
    use std::collections::BTreeSet;

    #[test]
    fn refine_string_test() {
        assert_eq!(None, Normal.refine_string("abc", "d"));
        assert_eq!(Some(result::Result::new("abc", vec![1].into_iter().collect())),
                   Normal.refine_string("abc", "b"));
        assert_eq!(Some(result::Result::new("abc", vec![0, 2].into_iter().collect())),
                   Normal.refine_string("abc", "a c"));
        assert_eq!(Some(result::Result::new("abc", vec![0, 2].into_iter().collect())),
                   Normal.refine_string("abc", "a  c "));
    }
}
