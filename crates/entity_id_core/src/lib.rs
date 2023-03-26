mod derive;

pub use derive::*;

pub trait EntityId:
    std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + Clone + Copy
{
}

#[inline]
pub fn unprefix_id(value: &str) -> &str {
    value.split('_').last().unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unprefix_id_returns_the_id_without_the_prefix() {
        let unprefixed = unprefix_id("todo_123");

        assert_eq!(unprefixed, "123");
    }

    #[test]
    fn unprefix_id_returns_the_id_without_the_prefix_when_the_prefix_has_multiple_segments() {
        let unprefixed = unprefix_id("a_b_c_245");

        assert_eq!(unprefixed, "245");
    }
}
