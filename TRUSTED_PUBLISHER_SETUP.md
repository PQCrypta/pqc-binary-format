# Trusted Publisher Setup Guide

This guide explains how to set up GitHub Actions as a Trusted Publisher on crates.io for automated, secure publishing without manual API tokens.

## ✅ What We Created

We've added two publishing workflows:

1. **`.github/workflows/publish.yml`** - Main publishing workflow (recommended for Trusted Publisher)
2. **`.github/workflows/release.yml`** - Advanced release management workflow

## 📋 Trusted Publisher Configuration

### On crates.io

Go to: https://crates.io/settings/tokens

Click **"Add a new Trusted Publisher"**

Fill in the form:

```
Publisher: GitHub
Repository owner: PQCrypta
Repository name: pqcrypta-community
Workflow filename: publish.yml
Environment name: crates-io
```

### Why These Values?

- **Repository owner:** `PQCrypta` - Your GitHub organization
- **Repository name:** `pqcrypta-community` - This repository
- **Workflow filename:** `publish.yml` - The workflow we just created
- **Environment name:** `crates-io` - GitHub environment (provides extra security)

## 🔐 GitHub Environment Setup

**IMPORTANT:** You need to create the `crates-io` environment in your GitHub repository.

### Step 1: Go to Repository Settings

1. Visit: https://github.com/PQCrypta/pqcrypta-community/settings/environments
2. Click **"New environment"**
3. Name: `crates-io`
4. Click **"Configure environment"**

### Step 2: Configure Environment Protection (Recommended)

Add protection rules:

- ✅ **Required reviewers:** Add yourself or trusted maintainers
- ✅ **Wait timer:** 0 minutes (or add delay if you want review time)
- ✅ **Deployment branches:** Only `main` branch

### Step 3: Add Secret (Temporary)

While Trusted Publisher is being set up, add:

- **Name:** `CARGO_REGISTRY_TOKEN`
- **Value:** Your crates.io API token (the one you used earlier)

**Note:** Once Trusted Publisher is fully configured, this token won't be needed anymore - GitHub will use OIDC authentication.

## 🚀 How to Publish

### Option 1: Manual Workflow Dispatch (Easiest)

1. Go to: https://github.com/PQCrypta/pqcrypta-community/actions/workflows/publish.yml
2. Click **"Run workflow"**
3. Select branch: `main`
4. Click **"Run workflow"**

The workflow will:
- ✅ Run all tests
- ✅ Run all examples
- ✅ Package and verify
- ✅ Publish to crates.io

### Option 2: Create a GitHub Release

1. Go to: https://github.com/PQCrypta/pqcrypta-community/releases/new
2. Click **"Choose a tag"**
3. Type: `v1.0.1` (or next version)
4. Click **"Create new tag"**
5. Title: `Release v1.0.1`
6. Click **"Publish release"**

This automatically triggers the `publish.yml` workflow.

### Option 3: Push a Tag

```bash
# Update version in Cargo.toml first
sed -i 's/version = "1.0.0"/version = "1.0.1"/' Cargo.toml

# Commit the change
git add Cargo.toml
git commit -m "chore: bump version to 1.0.1"
git push

# Create and push tag
git tag v1.0.1
git push origin v1.0.1
```

The `release.yml` workflow triggers on version tags.

## 🔍 Workflow Differences

### publish.yml (Recommended)

**Triggers:**
- Manual workflow dispatch
- GitHub release publication

**Best for:**
- Quick releases
- Hotfixes
- Manual control

**Usage:**
```bash
# Just click "Run workflow" in GitHub Actions UI
```

### release.yml (Advanced)

**Triggers:**
- Git tags matching `v*.*.*`
- Manual workflow dispatch with version input

**Best for:**
- Automated version bumping
- Release automation
- CI/CD pipelines

**Usage:**
```bash
git tag v1.0.1
git push origin v1.0.1
```

## ✅ Verification Steps

### After Setting Up Trusted Publisher

1. **Remove the old API token** from GitHub secrets (optional but recommended)
2. **Test the workflow:**
   - Go to Actions → Publish → Run workflow
   - Watch it run (should succeed!)

### What Should Happen

```
✓ Checkout repository
✓ Install Rust
✓ Cache dependencies
✓ Run tests (21 tests pass)
✓ Run examples (all 3 work)
✓ Verify package (builds successfully)
✓ Publish to crates.io (using Trusted Publisher OIDC)
✓ Verify publication (checks crates.io API)
```

## 🎯 Benefits of Trusted Publisher

### Security
- ✅ No long-lived API tokens
- ✅ OIDC authentication (temporary credentials)
- ✅ Scoped to specific repository
- ✅ Scoped to specific workflow

### Convenience
- ✅ No token rotation needed
- ✅ No secret management
- ✅ Automated and secure

### Transparency
- ✅ Clear audit trail
- ✅ Visible in GitHub Actions logs
- ✅ Repository-scoped permissions

## 🔧 Troubleshooting

### "Workflow not found" Error

**Problem:** crates.io can't find the workflow file.

**Solution:**
1. Ensure workflows are pushed to GitHub
2. Check file exists: `.github/workflows/publish.yml`
3. Wait a few minutes for GitHub to index

### "Environment not found" Error

**Problem:** The `crates-io` environment doesn't exist.

**Solution:**
1. Go to repository Settings → Environments
2. Create environment named exactly: `crates-io`

### "Permission denied" Error

**Problem:** Workflow doesn't have necessary permissions.

**Solution:**
Check the workflow has:
```yaml
permissions:
  contents: read
  id-token: write
```

### Publication Fails

**Problem:** Tests fail or package is invalid.

**Solution:**
1. Run tests locally: `cargo test --all`
2. Check examples work: `cargo run --example basic_usage`
3. Verify package: `cargo package --list`

## 📚 Additional Resources

- **Trusted Publishers Documentation:** https://doc.rust-lang.org/cargo/reference/publishing.html#trusted-publishers
- **GitHub Actions Docs:** https://docs.github.com/en/actions
- **Crates.io Publishing Guide:** https://doc.rust-lang.org/cargo/reference/publishing.html

## 🎯 Summary

**To complete Trusted Publisher setup:**

1. ✅ Workflows created and pushed to GitHub
2. ⏳ Go to crates.io → Settings → Add Trusted Publisher
3. ⏳ Fill in form (see values above)
4. ⏳ Create `crates-io` environment on GitHub
5. ⏳ Test by running workflow manually

**After setup, you can publish new versions without manual token management!**

---

**Need help?** Check the GitHub Actions logs or open an issue.
