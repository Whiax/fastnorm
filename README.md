# fastnorm

Fast, multithreaded text normalization for Python, written in Rust (PyO3 + rayon).
Releases the GIL and processes a whole batch per call.

## What it does

For each string, in order:

1. ASCII: lowercase. Non-ASCII: Unicode NFKC, then lowercase.
2. Collapse all whitespace runs to a single space and trim.
3. Replace curly quotes (`‘ ’` → `'`, `“ ”` → `"`) and en/em dashes (→ `-`).
4. Delete zero-width characters (U+200B–U+200D, U+FEFF) and control characters
   (`\x00–\x08`, `\x0b`, `\x0c`, `\x0e–\x1f`).
5. Collapse repeated identical punctuation (`! ? . , ; : -`) to one character.
6. Strip.

## Install

Download the wheel from the GitHub Actions artifacts or Releases, then:

```bash
pip install fastnorm-0.1.0-cp38-abi3-win_amd64.whl
```

Build from source (needs Rust): `pip install maturin && maturin develop --release`

## Usage

```python
import fastnorm

texts = ["Hello   WORLD!!!", "ＡＢＣ…  “quoted”", "a\u200bb\x00c"]
print(fastnorm.normalize_batch(texts))
# ['hello world!', "abc. "quoted"", 'abc']
```

Use batches of thousands of strings; tiny batches are dominated by call overhead.

## Pure-Python fallback

```python
try:
    from fastnorm import normalize_batch
except ImportError:
    def normalize_batch(texts):
        return [normalize_text(t) for t in texts]
```


## Notes

- Output is designed to match the reference Python `normalize_text`; verify on your data
  with `assert normalize_batch(xs) == [normalize_text(x) for x in xs]`.
- Unicode version differences between Python and Rust can affect very rare characters.
- Strings containing lone surrogates raise an error.
