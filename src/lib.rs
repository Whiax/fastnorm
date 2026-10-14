use pyo3::prelude::*;
use rayon::prelude::*;
use unicode_normalization::UnicodeNormalization;

// Python's str.split() also treats \x1c-\x1f as whitespace; Rust doesn't
fn is_py_ws(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}
fn is_rep(c: char) -> bool {
    matches!(c, '!' | '?' | '.' | ',' | ';' | ':' | '-')
}

fn norm(s: &str) -> String {
    let low = s.to_lowercase();
    let t: String = if low.is_ascii() { low } else { low.nfkc().collect() };

    // 1) collapse whitespace
    let mut a = String::with_capacity(t.len());
    for w in t.split(is_py_ws).filter(|w| !w.is_empty()) {
        if !a.is_empty() { a.push(' '); }
        a.push_str(w);
    }

    // 2) map/delete rare chars + 3) collapse repeated punctuation
    let mut out = String::with_capacity(a.len());
    let mut last: Option<char> = None;
    for c in a.chars() {
        let c = match c {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            '\u{2013}' | '\u{2014}' => '-',
            '\u{200b}'..='\u{200d}' | '\u{feff}' | '\0'..='\u{8}'
            | '\u{b}' | '\u{c}' | '\u{e}'..='\u{1f}' => continue,
            c => c,
        };
        if is_rep(c) {
            if last == Some(c) { continue; }
            last = Some(c);
        } else {
            last = None;
        }
        out.push(c);
    }
    out.trim_matches(is_py_ws).to_string()
}

#[pyfunction]
fn normalize_batch(py: Python<'_>, texts: Vec<String>) -> Vec<String> {
    // allow_threads is named `detach` in newer PyO3 versions
    py.allow_threads(|| texts.par_iter().map(|s| norm(s)).collect())
}

#[pymodule]
fn fastnorm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(normalize_batch, m)?)
}