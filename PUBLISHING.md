# Publishing Guide

Complete workflow for publishing PQC Binary Format to all package registries.

## 📦 Package Registries Overview

| Registry | Account Required | Auth Method | Auto-Publish |
|----------|------------------|-------------|--------------|
| **crates.io** (Rust) | ✅ Published v1.0.7 | API Token | ✅ Done |
| **PyPI** (Python) | ⏳ Next | Trusted Publisher (GitHub Actions) | ✅ Can automate |
| **npm** (JavaScript) | ⏳ Next | API Token | ❌ Manual |
| **pkg.go.dev** (Go) | ⏳ Next | Git tags only | ✅ Automatic |

---

## 1️⃣ crates.io (Rust Package)

### Setup (One-time)

1. **Create crates.io Account**
   - Go to https://crates.io/
   - Click "Log in with GitHub"
   - Authorize crates.io to access your GitHub account

2. **Generate API Token**
   ```bash
   # Visit https://crates.io/settings/tokens
   # Click "New Token"
   # Give it a name like "PQCrypta Publishing"
   # Copy the token (you'll only see it once!)
   ```

3. **Store Token Locally** (for manual publishing)
   ```bash
   cargo login <YOUR_TOKEN>
   # Token is stored in ~/.cargo/credentials.toml
   ```

4. **Store Token in GitHub Secrets** (for automated publishing)
   - Go to GitHub repo → Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: Paste your crates.io token

### Publishing Process

**Option A: Manual Publish**
```bash
# 1. Ensure you're on a clean main branch
git status

# 2. Build and test
cargo build --release
cargo test --all-features

# 3. Dry run to check for issues
cargo publish --dry-run

# 4. Publish for real
cargo publish

# 5. Tag the release
git tag v1.0.7
git push origin v1.0.7
```

**Option B: Automated via GitHub Actions** (Already configured)
```yaml
# .github/workflows/publish.yml
name: Publish to crates.io
on:
  push:
    tags:
      - 'v*'
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

**To trigger automated publish:**
```bash
git tag v1.0.7
git push origin v1.0.7
# GitHub Actions will automatically publish to crates.io
```

### Post-Publication

- Package appears at: https://crates.io/crates/pqc-binary-format
- Documentation auto-generated at: https://docs.rs/pqc-binary-format
- Allow ~10 minutes for docs to build

---

## 2️⃣ PyPI (Python Package)

### Setup (One-time)

**✨ RECOMMENDED: Use Trusted Publisher (No API tokens needed!)**

1. **Create PyPI Account**
   - Go to https://pypi.org/account/register/
   - Verify your email address
   - Enable 2FA (required for new projects)

2. **Configure Trusted Publisher** (Easiest method!)
   - Go to https://pypi.org/manage/account/publishing/
   - Click "Add a new pending publisher"
   - Fill in:
     - **PyPI Project Name**: `pqc-binary-format`
     - **Owner**: `PQCrypta`
     - **Repository name**: `pqcrypta-community`
     - **Workflow name**: `publish-python.yml`
     - **Environment name**: Leave blank OR use `pypi`
   - Click "Add"

   This allows GitHub Actions to publish without API tokens!

**Alternative: API Token Method**

If you prefer traditional tokens:
```bash
# 1. Go to https://pypi.org/manage/account/token/
# 2. Create token scoped to "pqc-binary-format" project
# 3. Add to GitHub Secrets as PYPI_API_TOKEN
```

### Publishing Process

**Option A: Manual Publish** (using maturin)
```bash
# 1. Ensure maturin is installed
pip install maturin

# 2. Build the wheel
maturin build --release

# 3. Publish to PyPI
maturin publish

# When prompted, enter:
# Username: __token__
# Password: <your PyPI API token>
```

**Option B: Automated via GitHub Actions with Trusted Publisher**

Create `.github/workflows/publish-python.yml`:
```yaml
name: Publish Python to PyPI

on:
  push:
    tags:
      - 'v*'

permissions:
  id-token: write  # IMPORTANT for trusted publishing
  contents: read

jobs:
  publish-pypi:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install maturin
        run: pip install maturin

      - uses: dtolnay/rust-toolchain@stable

      - name: Build wheels
        run: maturin build --release

      - name: Publish to PyPI
        uses: pypa/gh-action-pypi-publish@release/v1
        with:
          packages-dir: target/wheels/
```

**To publish:**
```bash
git tag v1.0.7
git push origin v1.0.7
# GitHub Actions automatically publishes to PyPI
```

### Post-Publication

- Package appears at: https://pypi.org/project/pqc-binary-format/
- Install with: `pip install pqc-binary-format`
- Allow ~5 minutes for package to be available

---

## 3️⃣ npm (JavaScript/WASM Package)

### Setup (One-time)

1. **Create npm Account**
   - Go to https://www.npmjs.com/signup
   - Verify your email address
   - Enable 2FA (highly recommended)

2. **Generate Access Token**
   ```bash
   # Option 1: Via website
   # Go to https://www.npmjs.com/settings/<username>/tokens
   # Click "Generate New Token" → "Automation"
   # Copy the token

   # Option 2: Via CLI
   npm login
   npm token create --read-only=false
   ```

3. **Store Token in GitHub Secrets**
   - Go to GitHub repo → Settings → Secrets → Actions
   - Name: `NPM_TOKEN`
   - Value: Paste your npm token

4. **Update package.json** (if needed)
   ```json
   {
     "name": "@pqcrypta/pqc-binary-format",
     "version": "1.0.7",
     "repository": {
       "type": "git",
       "url": "https://github.com/PQCrypta/pqcrypta-community.git"
     },
     "publishConfig": {
       "access": "public"
     }
   }
   ```

### Publishing Process

**Option A: Manual Publish**
```bash
# 1. Build WASM package
wasm-pack build --target web --features wasm

# 2. Navigate to pkg directory
cd pkg

# 3. Login to npm (if not already)
npm login

# 4. Publish
npm publish --access public

# If using scoped package (@pqcrypta/...)
npm publish --access public
```

**Option B: Automated via GitHub Actions**

Create `.github/workflows/publish-npm.yml`:
```yaml
name: Publish to npm

on:
  push:
    tags:
      - 'v*'

jobs:
  publish-npm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - uses: jetli/wasm-pack-action@v0.4.0

      - name: Build WASM
        run: wasm-pack build --target web --features wasm

      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org'

      - name: Publish to npm
        working-directory: ./pkg
        run: npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

**To publish:**
```bash
git tag v1.0.7
git push origin v1.0.7
```

### Post-Publication

- Package appears at: https://www.npmjs.com/package/pqc-binary-format
- Install with: `npm install pqc-binary-format`
- Instant availability (no delay)

---

## 4️⃣ pkg.go.dev (Go Package)

### Setup

**✨ NO ACCOUNT NEEDED!** pkg.go.dev automatically indexes public GitHub repositories.

### Requirements

1. **Valid go.mod file** ✅ (Already created at `bindings/go/go.mod`)
2. **Public GitHub repository** ✅ (Already public)
3. **Semantic version tags** ✅ (Use `v1.0.7` format)

### Publishing Process

**Automatic indexing via Git tags:**

```bash
# 1. Ensure bindings/go/go.mod is correct
cat bindings/go/go.mod
# Should show:
# module github.com/PQCrypta/pqcrypta-community/bindings/go
# go 1.22

# 2. Create and push a tag
git tag bindings/go/v1.0.7
git push origin bindings/go/v1.0.7

# 3. Trigger indexing (optional, usually automatic)
# Visit: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go@v1.0.7
# Click "Request" if not indexed
```

**Alternative: Version everything together**
```bash
# Use main version tag
git tag v1.0.7
git push origin v1.0.7

# Go module at: github.com/PQCrypta/pqcrypta-community/bindings/go
```

### Post-Publication

- Package appears at: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go
- Import with: `import "github.com/PQCrypta/pqcrypta-community/bindings/go"`
- Allow ~10-30 minutes for first indexing
- Subsequent updates are near-instant

---

## 🚀 Complete Publishing Workflow (All Platforms)

### Pre-Publishing Checklist

```bash
# 1. Version bump (already done for v1.0.7)
grep "version" Cargo.toml pyproject.toml bindings/python/setup.py bindings/javascript/package.json

# 2. Update CHANGELOG.md
# Add release notes for v1.0.7 ✅

# 3. Run all tests
cargo test --all-features
cargo clippy
cargo fmt --check

# 4. Test examples (already validated ✅)

# 5. Clean working directory
git status
# Should show: "nothing to commit, working tree clean"

# 6. Commit and push everything
git push origin main
```

### Publishing Steps (Recommended Order)

**Step 1: Create Git Tag**
```bash
git tag -a v1.0.7 -m "Release v1.0.7: Production-ready language bindings"
git push origin v1.0.7

# For Go sub-module (optional)
git tag bindings/go/v1.0.7
git push origin bindings/go/v1.0.7
```

**Step 2: Publish to crates.io** (Automated via GitHub Actions)
- GitHub Actions will trigger on tag push
- Wait ~5 minutes for completion
- Check: https://crates.io/crates/pqc-binary-format

**Step 3: Publish to PyPI** (Manual or automated)
```bash
# Manual:
maturin publish

# Or wait for GitHub Actions if configured
```

**Step 4: Publish to npm** (Manual or automated)
```bash
# Manual:
wasm-pack build --target web --features wasm
cd pkg && npm publish --access public

# Or wait for GitHub Actions if configured
```

**Step 5: Verify pkg.go.dev**
- Wait ~30 minutes
- Visit: https://pkg.go.dev/github.com/PQCrypta/pqcrypta-community/bindings/go
- If not indexed, click "Request"

---

## 🎯 Quick Start for First-Time Publishing

### Minimum Required Setup

1. **crates.io**: Create account, get token, run `cargo login`
2. **PyPI**: Create account, enable 2FA, configure Trusted Publisher
3. **npm**: Create account, get token, run `npm login`
4. **pkg.go.dev**: Nothing! (automatic)

### First Publish Commands

```bash
# Publish everything
git tag v1.0.7
git push origin v1.0.7

# Crates.io (if not using GitHub Actions)
cargo publish

# PyPI
maturin publish

# npm
wasm-pack build --target web --features wasm
cd pkg && npm publish --access public

# pkg.go.dev
# Automatic within 30 minutes of tag push
```

---

## 📋 Post-Publication Verification

### Check All Packages

```bash
# crates.io
curl -s https://crates.io/api/v1/crates/pqc-binary-format | jq '.crate.newest_version'

# PyPI
curl -s https://pypi.org/pypi/pqc-binary-format/json | jq '.info.version'

# npm
npm view pqc-binary-format version

# pkg.go.dev
curl -s "https://api.github.com/repos/PQCrypta/pqcrypta-community/tags" | jq '.[0].name'
```

### Test Installation

```bash
# Rust
cargo add pqc-binary-format@1.0.7

# Python
pip install pqc-binary-format==1.0.7

# JavaScript
npm install pqc-binary-format@1.0.7

# Go
go get github.com/PQCrypta/pqcrypta-community/bindings/go@v1.0.7
```

---

## 🔧 Troubleshooting

### crates.io

**Error: "already uploaded"**
- Version already exists, bump version number

**Error: "authentication required"**
```bash
cargo login <token>
```

### PyPI

**Error: "403 Forbidden"**
- Check if Trusted Publisher is configured correctly
- Ensure 2FA is enabled
- Verify API token has correct scope

**Error: "package name already taken"**
- Choose different name OR request ownership transfer

### npm

**Error: "You do not have permission"**
```bash
npm login
npm publish --access public  # For scoped packages
```

**Error: "version already exists"**
- Bump version number, npm doesn't allow overwriting

### pkg.go.dev

**Not appearing after 30 minutes:**
1. Visit the URL directly and click "Request"
2. Ensure go.mod has correct module path
3. Check tag format is `v1.0.7` (with 'v' prefix)

---

## 📞 Support

- **crates.io**: https://crates.io/policies
- **PyPI**: https://pypi.org/help/
- **npm**: https://docs.npmjs.com/
- **pkg.go.dev**: https://go.dev/about

## 🎉 Ready to Publish!

All prerequisites are documented. Follow the workflows above to publish v1.0.7 to all four package registries!
