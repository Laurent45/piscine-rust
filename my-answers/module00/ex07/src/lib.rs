
fn only_wildcard(str: &[u8]) -> bool {
    for &c in str {
        if c != b'*' {
            return false;
        }
    }
    true
}

pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    if query.is_empty() {
        return only_wildcard(pattern);
    }

    let mut i = 0;
    let mut wildcard = false;

    for &c in query {
        // There is no pattern char to compare with c and the wildcard mode in disable
        if i == pattern.len() && !wildcard {
            return false;
        }


        if pattern[i] == b'*' {
            wildcard = true;
            i += 1;
            // Return true because all the remaining char in query will match
            if i == pattern.len() {
                return true;
            }
        }

        if c == pattern[i] {
            wildcard = false;
            i += 1;
        } else if c != pattern[i] && !wildcard {
            return false;
        }
    }
    // If there are remaining char is pattern, they must only be wildcard
    only_wildcard(&pattern[i..])
}

#[cfg(test)]
mod tests {

    use super::strpcmp;

    #[test]
    fn query_should_match_pattern() {
        assert!(strpcmp(b"abc", b"abc"));
        assert!(strpcmp(b"abcd", b"ab*"));
        assert!(strpcmp(b"dcab", b"*ab"));
        assert!(strpcmp(b"ab000cd", b"ab*cd"));
        assert!(strpcmp(b"abcd", b"ab*cd"));
        assert!(strpcmp(b"", b"****"));
        assert!(strpcmp(b"", b""));
    }

    #[test]
    fn query_should_not_match_pattern() {
        assert!(!strpcmp(b"abc", b"aac"));
        assert!(!strpcmp(b"abcd", b"abce"));
        assert!(!strpcmp(b"cab", b"ab*"));
        assert!(!strpcmp(b"abc", b"*ab"));
        assert!(!strpcmp(b"", b"****a"));
        assert!(!strpcmp(b"abc", b""));
        assert!(!strpcmp(b"abcde", b"ab*ef"));
    }


    #[test]
    fn exact_match() {
        assert!(strpcmp(b"abc", b"abc"));
        assert!(strpcmp(b"", b""));
        assert!(strpcmp(b"hello", b"hello"));

        assert!(!strpcmp(b"", b"abc"));
        assert!(!strpcmp(b"abc", b""));
        assert!(!strpcmp(b"abc", b"def"));
    }

    #[test]
    fn match_begining() {
        assert!(strpcmp(b"ab", b"ab*"));
        assert!(strpcmp(b"abc", b"ab*"));
        assert!(strpcmp(b"abcd", b"ab*"));

        assert!(!strpcmp(b"aab", b"ab*"));
    }

    #[test]
    fn match_end() {
        assert!(strpcmp(b"ab", b"*ab"));
        assert!(strpcmp(b"cab", b"*ab"));
        assert!(strpcmp(b"dcab", b"*ab"));

        assert!(!strpcmp(b"abc", b"*ab"));
    }

    #[test]
    fn match_start_and_end() {
        assert!(strpcmp(b"abcd", b"ab*cd"));
        assert!(strpcmp(b"ab0cd", b"ab*cd"));
        assert!(strpcmp(b"ab00cd", b"ab*cd"));
        assert!(strpcmp(b"ababcd", b"ab*cd"));

        assert!(!strpcmp(b"ab0dd", b"ab*cd"));
        assert!(!strpcmp(b"ab0cdd", b"ab*cd"));
        assert!(!strpcmp(b"bb0cd", b"ab*cd"));
        assert!(!strpcmp(b"aab0cd", b"ab*cd"));
    }

    #[test]
    fn contains_substring() {
        assert!(strpcmp(b"ab000ab", b"*000*"));
        assert!(strpcmp(b"000", b"*000*"));
        assert!(strpcmp(b"000ab", b"*000*"));
        assert!(strpcmp(b"ab000", b"*000*"));

        assert!(!strpcmp(b"ab00", b"*000*"));
        assert!(!strpcmp(b"ab00ab", b"*000*"));
        assert!(!strpcmp(b"00ab", b"*000*"));
    }

    #[test]
    fn full_match() {
        assert!(strpcmp(b"", b"*"));
        assert!(strpcmp(b"abc", b"*"));
    }

}
