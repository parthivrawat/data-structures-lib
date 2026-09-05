# Contributing

Thank you for your interest in contributing to the Data Structures Library. This guide covers environment setup, testing, and our conventions for branches, commits, and releases.

## Repository Layout

```
data-structures-lib/
├── python/     # PyPI package
├── typescript/ # npm package
├── go/         # Go module
├── rust/       # crates.io package
└── README.md
```

Each language directory is an independent package with its own build and test toolchain.

## Environment Setup

### Python

Requires Python 3.x and `pip`.

```sh
cd python
pip install -e ".[dev]"
```

### TypeScript

Requires Node.js and `npm`.

```sh
cd typescript
npm install
```

### Go

Requires the Go toolchain (see `go/go.mod` for the minimum version).

```sh
cd go
go mod download
```

### Rust

Requires Rust and Cargo (via [rustup](https://rustup.rs/)).

```sh
cd rust
cargo build
```

## Running Tests

Run tests from within each language directory:

| Language   | Command                          |
|------------|----------------------------------|
| Python     | `pytest`                         |
| TypeScript | `npm test`                       |
| Go         | `go test ./...`                  |
| Rust       | `cargo test`                     |

Before submitting a change, also verify the package builds:

- **TypeScript**: `npm run build`
- **Go**: `go build ./...`
- **Rust**: `cargo build`

### Test Guidelines

- Add or update tests for every behavior change.
- Cover edge cases: empty structures, single-element structures, boundary indices, and error paths.
- Keep test names consistent with the existing suite in each language.

## Code Style

- Follow the existing conventions of the file and language you are modifying.
- Keep implementations zero-dependency — do not add third-party libraries.
- Maintain API parity: when adding or changing an operation in one language, consider whether the same change applies to the other three. Cross-language naming conventions are documented in the README's API mapping table.
- Error semantics differ per language (exceptions vs. returned errors) — see the README's error mapping table before introducing new error types.

## Branch Conventions

- `main` — always releasable; do not commit directly.
- `feature/<short-description>` — new features and data structures.
- `fix/<short-description>` — bug fixes.
- `docs/<short-description>` — documentation-only changes.
- `chore/<short-description>` — build, CI, and maintenance work.

Use lowercase with hyphens, e.g. `feature/avl-tree-go` or `fix/stack-peek-empty`.

## Commit Conventions

Write clear, imperative commit messages prefixed by scope and language when relevant:

```
python: make SinglyLinkedList.append O(1)
typescript: add NotFoundError for missing keys
go: replace reflect.DeepEqual with == in linked lists
docs: add error mapping table to README
```

Keep commits small and atomic — one logical change per commit.

## Pull Requests

1. Ensure all tests pass for every language you touched.
2. Keep PRs focused; unrelated changes belong in separate PRs.
3. Describe what changed, why, and any cross-language implications.
4. Update `CHANGELOG.md` under an "Unreleased" section (or the pending version) for user-facing changes.

## Release Process

1. **Bump the version** in each language package's metadata (`python/pyproject.toml` and `python/data_structures_lib/__init__.py`, `typescript/package.json` + `package-lock.json`, `rust/Cargo.toml` and `rust/README.md`, and `go` via a git tag), keeping all four in sync.
2. **Update `CHANGELOG.md`**: move pending changes under a new `## [X.Y.Z] - YYYY-MM-DD` section following the Keep a Changelog format.
3. **Merge to `main`** via a release PR.
4. **Tag the release**: `git tag vX.Y.Z && git push origin vX.Y.Z`.
5. **Publish each package**:
   - **Python**: build the sdist/wheel (`python -m build` in `python/`) and upload with `twine upload dist/*` to PyPI.
   - **TypeScript**: `npm publish` from `typescript/` (ensure `npm run build` ran first).
   - **Go**: the module is published via the git tag; verify it resolves at `pkg.go.dev` (e.g., `go list -m github.com/parthivrawat/data-structures-lib/go@vX.Y.Z`).
   - **Rust**: `cargo publish` from `rust/`.
6. Verify each registry shows the new version.

## Reporting Issues

Open an issue describing the problem, the language(s) affected, steps to reproduce, and expected vs. actual behavior. Include a minimal code example when possible.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
