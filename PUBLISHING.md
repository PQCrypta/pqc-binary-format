# Publishing

A release is a tag. Pushing `vX.Y.Z` publishes that version everywhere:

| Registry | Workflow | How it signs in | Published now |
|----------|----------|-----------------|---------------|
| [crates.io](https://crates.io/crates/pqc-binary-format) | `publish.yml` | Trusted publishing (OIDC), environment `crates-io` | 2.0.2 |
| [PyPI](https://pypi.org/project/pqc-binary-format/) | `publish-python.yml` | Trusted publishing (OIDC) | 2.0.2 (one cp311 Linux wheel; abi3 wheels for every platform from the next release) |
| [npm](https://www.npmjs.com/package/pqc-binary-format) | `publish-npm.yml` | Trusted publishing (OIDC), npm 11.5.1 or later | 2.0.2 |
| [Go module proxy](https://pkg.go.dev/github.com/PQCrypta/pqc-binary-format/bindings/go/v2) | `go-module-tag.yml` | `GITHUB_TOKEN` (pushes the `bindings/go/` tag) | 1.0.14; 2.x from the next release |

No workflow holds a registry token. Each one skips a version that is
already published, so running one again is safe.

## One-time setup

Trusted publishing is set up on each registry for this repository and the
workflow's file name. PyPI is done. The other two:

- **crates.io**: crate settings → *Trusted Publishing* → add GitHub,
  owner `PQCrypta`, repository `pqc-binary-format`, workflow `publish.yml`,
  environment `crates-io`. Until then the crates.io job fails at
  *Get OIDC token* ("No Trusted Publishing config found").
- **npm**: package settings → *Trusted publisher* → GitHub Actions,
  organization `PQCrypta`, repository `pqc-binary-format`, workflow
  `publish-npm.yml`. Then delete the `NPM_TOKEN` repository secret, which
  nothing uses any more (it had expired: the 2.0.x publishes failed and
  were done by hand).

## Releasing

1. Set the version in `Cargo.toml`, `pyproject.toml` and
   `bindings/javascript/package.json`, and add the release to
   `CHANGELOG.md`. The crates.io workflow refuses a tag that disagrees
   with any of the three.
2. Merge to `main` with CI passing.
3. Tag and push:

   ```bash
   git tag -a v2.0.3 -m "pqc-binary-format 2.0.3"
   git push origin v2.0.3
   ```

The tag runs:

- **crates.io**: tests and examples, `cargo package`, then `cargo publish`.
- **PyPI**: abi3 wheels for Linux (manylinux and musllinux, x86-64 and
  aarch64), macOS (arm64 and x86-64) and Windows (x64), each good for
  every CPython from `requires-python` up, plus the sdist.
- **npm**: `wasm-pack build --target bundler -- --features wasm`, then
  `npm publish` from `pkg/` with provenance.
- **Go**: `go vet` and `go test` in `bindings/go`, then the tag
  `bindings/go/vX.Y.Z` (Go finds a module in a subdirectory by a tag
  carrying its path), then a fetch from the module proxy so the version is
  served at once.

## The Go module path

From v2, Go requires the major version at the end of the module path
(semantic import versioning). The module is
`github.com/PQCrypta/pqc-binary-format/bindings/go/v2`; at v3 it becomes
`…/bindings/go/v3`. `go-module-tag.yml` refuses a tag whose major version
the path does not carry. The 2.0.x tags were made before this was right,
which is why the proxy still serves 1.0.14:

```bash
go get github.com/PQCrypta/pqc-binary-format/bindings/go/v2
```

## Checking a release

```bash
v=2.0.3
curl -s https://crates.io/api/v1/crates/pqc-binary-format | jq -r .crate.max_version
curl -s https://pypi.org/pypi/pqc-binary-format/json | jq -r '.urls[].filename'
npm view pqc-binary-format version dist.attestations.url
curl -s "https://proxy.golang.org/github.com/!p!q!crypta/pqc-binary-format/bindings/go/v2/@v/v$v.info"
```
