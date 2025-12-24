pub fn wrap_once<'a>(s: &'a str, width: usize) -> (&'a str, &'a str) {
  if s.len() <= width {
    return (s, "");
  }

  let chunk = (width + 1).min(s.len());
  match s[..chunk].rfind(' ') {
    Some(end) => (&s[..end], &s[end + 1..]),
    None => (&s[..width], &s[width..])
  }
}

pub fn wrap<'a>(mut s: &'a str, width: usize) -> impl Iterator<Item = &'a str> {
  std::iter::from_fn(move || {
    if s.is_empty() {
      return None;
    }

    let line;
    (line, s) = wrap_once(s, width);

    Some(line)
  })
}