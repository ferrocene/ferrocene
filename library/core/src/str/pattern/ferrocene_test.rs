use super::*;

pub fn test_two_way_searcher_next_back() {
    let haystack = "Việt Namacbaabcaabaaba";
    let needle = "Vic";
    let mut searcher = TwoWaySearcher::new(needle.as_bytes(), haystack.len());

    assert!(
        searcher.next_back::<MatchOnly>(haystack.as_bytes(), needle.as_bytes(), false).is_none()
    );
}
