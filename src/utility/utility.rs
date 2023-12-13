pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn compare_edge(a: &str, b: &str) -> bool {
    a == reverse_string(b)
}
