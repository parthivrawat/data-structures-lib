# Security Policy

## Security posture

`data-structures-lib` is a pure, in-memory data structures library. It does **not** perform any of the following:

- Network I/O
- File system I/O
- Secret handling or credential management
- Untrusted input parsing beyond the data-structure operations themselves
- Use of `unsafe` code in its released source (the previously reported `std::mem::zeroed` issue in `rust/src/linear.rs` has been resolved)

All four language implementations (Python, TypeScript, Go, Rust) are intended to be **zero-runtime-dependency** packages. The only third-party dependencies are declared as development dependencies (e.g., `rand` for Rust property tests, `pytest` for Python tests, `vitest` for TypeScript tests) and are not part of any shipped artifact.

## Known security-relevant changes

| Version | Change |
|---|---|
| 1.1.0 | Removed unsound `unsafe { std::mem::zeroed() }` usage from the Rust linked-list implementation. Nodes now store values as `Option<T>` and use `value.take()` on removal. |

## Vulnerability classes considered out of scope for a data-structures crate

- **Traditional injection / XSS / CSRF / SSRF**: the library does not process web requests, SQL, HTML, or remote URLs.
- **Authentication / authorization**: the library has no user identity or permission model.
- **Secrets leakage**: the library does not read or write secrets.

## Internal invariants

Some implementations use language-native non-null assertions (e.g., Rust `unwrap` or TypeScript non-null assertions) for conditions that are guaranteed by internal invariants after bounds or existence checks. Invalid user input should still surface as the documented `Error`/`Result`/`exception` types rather than panics.

## Reporting a security issue

If you believe you have found a security-relevant bug, please report it privately using GitHub's "Report a vulnerability" flow on the repository's Security tab (requires private vulnerability reporting to be enabled). Do not open a public issue for security reports. Include:

- The language implementation affected
- A minimal reproduction example
- The expected vs. actual behavior
- The version of `data-structures-lib` you are using
