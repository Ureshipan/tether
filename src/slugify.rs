pub fn base_slug(title: &str) -> String {
    let s = slug::slugify(title);
    if s.is_empty() { "shard".to_string() } else { s }
}

pub fn unique_slug(title: &str, taken: &dyn Fn(&str) -> bool) -> String {
    let base = base_slug(title);
    if !taken(&base) { return base; }
    let mut n = 2;
    loop {
        let cand = format!("{}-{}", base, n);
        if !taken(&cand) { return cand; }
        n += 1;
    }
}
