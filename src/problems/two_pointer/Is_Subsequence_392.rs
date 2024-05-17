pub fn is_subsequence(s: String, t: String) -> bool {
    // keeps track of index of s string
    let mut p1 = 0;
    // keeps track of index of t string
    let mut p2 = 0;

    // convert both string to vecs for easy indexing
    let s1 = s.chars().collect::<Vec<char>>();
    let s2 = t.chars().collect::<Vec<char>>();

    // while both pointer in bounds
    while p1 < s1.len() && p2 < s2.len() {
        // bump p2 if no match
        if s1[p1] != s2[p2] {
            p2 += 1;
        // if match bump both
        } else {
            p1 += 1;
            p2 += 1;
        };
    }

    // since p1 is updated inside while loop,
    // if there is valid subsequence, p1 will be 1 bigger than possible valid index
    // which is also the length of s1, in other words all matches were found
    if p1 == s1.len() {
        return true;
    } else {
        return false;
    }
}
