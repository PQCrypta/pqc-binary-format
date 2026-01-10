# ✅ Trusted Publisher Setup - CORRECTED GUIDE

## 🎯 **IMPORTANT: No API Token Needed!**

With Trusted Publisher, you do **NOT** need to add any API token to GitHub secrets. That's the whole point - it uses secure OIDC authentication instead!

---

## 📋 **Step 1: Fill Out Crates.io Form**

Go to: https://crates.io/settings/tokens

Click: **"Add a new Trusted Publisher"**

Fill in **EXACTLY** these values:

```
Publisher: GitHub
Repository owner: PQCrypta
Repository name: pqcrypta-community
Workflow filename: publish.yml
Environment name: crates-io
```

Click **"Add"**

✅ **That's it for crates.io!** No token needed.

---

## 📋 **Step 2: Create GitHub Environment**

Go to: https://github.com/PQCrypta/pqcrypta-community/settings/environments

### 2a. Create Environment

1. Click **"New environment"**
2. Name: `crates-io` (must be exact)
3. Click **"Configure environment"**

### 2b. Configure Protection Rules (Optional but Recommended)

**Deployment branches:**
- Select **"Selected branches"**
- Add rule: `main`

**Required reviewers:** (Optional)
- Add yourself if you want approval before publishing

**Wait timer:**
- Set to `0` (or add minutes if you want delay)

### 2c. Environment Secrets

**❌ DO NOT ADD `CARGO_REGISTRY_TOKEN`** - Not needed with Trusted Publisher!

The workflow uses OIDC (OpenID Connect) authentication automatically.

---

## 📋 **Step 3: Test the Workflow**

### Option A: Manual Workflow Run (Easiest)

1. Go to: https://github.com/PQCrypta/pqcrypta-community/actions/workflows/publish.yml
2. Click **"Run workflow"** button
3. Select branch: `main`
4. Click **"Run workflow"**

### What Happens:

```
✓ Checkout code
✓ Install Rust
✓ Run all tests (21 tests)
✓ Run all examples (3 examples)
✓ Package verification
✓ Publish to crates.io (using OIDC - no token!)
✓ Verify publication
```

### Option B: Create a GitHub Release

1. Go to: https://github.com/PQCrypta/pqcrypta-community/releases/new
2. Click "Choose a tag"
3. Type: `v1.0.1` (or next version)
4. Title: `v1.0.1`
5. Click **"Publish release"**

Automatically triggers the workflow!

---

## 🔍 **How It Works (Technical)**

### Traditional Method (What You Did Manually)
```bash
cargo login <api-token>
cargo publish
```
- ❌ Requires long-lived API token
- ❌ Token stored in GitHub secrets
- ❌ Token can be leaked/stolen
- ❌ Needs rotation

### Trusted Publisher (OIDC)
```yaml
permissions:
  id-token: write  # Request OIDC token from GitHub
  contents: read

steps:
  - run: cargo publish  # No --token flag needed!
```

GitHub Actions:
1. Requests temporary OIDC token from GitHub
2. Presents token to crates.io
3. Crates.io verifies token matches registered publisher
4. Publishes package
5. Token expires immediately

✅ **No secrets to manage!**

---

## 🎯 **Summary: What You Need to Do**

| Step | What to Do | Where |
|------|-----------|-------|
| 1 | Add Trusted Publisher | https://crates.io/settings/tokens |
| 2 | Create `crates-io` environment | https://github.com/PQCrypta/pqcrypta-community/settings/environments |
| 3 | Test workflow | https://github.com/PQCrypta/pqcrypta-community/actions/workflows/publish.yml |

**Total time:** ~2 minutes
**Secrets needed:** 0
**Tokens needed:** 0

---

## ✅ **Crates.io Form Values (Copy-Paste Ready)**

```
Publisher: GitHub
Repository owner: PQCrypta
Repository name: pqcrypta-community
Workflow filename: publish.yml
Environment name: crates-io
```

---

## 🚨 **Common Mistakes to Avoid**

### ❌ DON'T:
- Add `CARGO_REGISTRY_TOKEN` to GitHub secrets (not needed!)
- Use `--token` flag in `cargo publish` (OIDC handles it)
- Store API tokens anywhere

### ✅ DO:
- Create the `crates-io` environment
- Use exact values from above
- Test with manual workflow run first

---

## 🔧 **Troubleshooting**

### "Authentication failed"

**Problem:** Trusted Publisher not configured correctly.

**Solution:**
1. Check environment name is exactly `crates-io`
2. Verify workflow filename is exactly `publish.yml`
3. Ensure repository owner/name are correct

### "Environment not found"

**Problem:** GitHub environment doesn't exist.

**Solution:**
1. Go to Settings → Environments
2. Create environment named `crates-io`

### "Permission denied"

**Problem:** Workflow lacks OIDC permissions.

**Solution:**
Already configured in workflow:
```yaml
permissions:
  id-token: write
  contents: read
```

---

## 🎉 **After Setup is Complete**

You can publish new versions by simply:

1. **Update version** in `Cargo.toml`
2. **Commit and push** to `main`
3. **Click "Run workflow"** in GitHub Actions

Or create a GitHub Release and it publishes automatically!

**No tokens, no secrets, fully automated!** 🚀

---

## 📚 **Additional Resources**

- **Cargo Trusted Publishers:** https://doc.rust-lang.org/cargo/reference/publishing.html#trusted-publishers
- **GitHub OIDC:** https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect
- **Crates.io Blog:** https://blog.rust-lang.org/2023/06/23/Trusted-Publishing.html

---

**Questions?** The workflow is already set up - just complete steps 1 and 2 above!
