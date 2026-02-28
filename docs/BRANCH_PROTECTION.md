# Branch Protection Rules

This document describes the branch protection rules configured for this repository.

## Protected Branches

### `main` (Production)

The main branch contains production-ready code.

**Protection Rules:**

| Rule | Setting | Description |
|------|---------|-------------|
| Require pull request reviews | ✅ 1 approval | All changes must be reviewed |
| Dismiss stale reviews | ✅ Enabled | Reviews dismissed on new commits |
| Require status checks | ✅ Enabled | CI must pass before merge |
| Require branches to be up to date | ✅ Enabled | Must rebase before merge |
| Required status checks | `fmt`, `clippy`, `test`, `coverage` | All CI checks must pass |
| Include administrators | ✅ Enabled | Rules apply to admins |
| Restrict pushes | ✅ Enabled | No direct pushes allowed |
| Require linear history | ✅ Enabled | Squash merge only |

### `develop` (Development)

The develop branch contains the latest development changes.

**Protection Rules:**

| Rule | Setting | Description |
|------|---------|-------------|
| Require pull request reviews | ✅ 1 approval | Recommended, can be disabled for hotfixes |
| Require status checks | ✅ Enabled | CI must pass |
| Required status checks | `fmt`, `clippy`, `test` | Basic checks must pass |
| Allow force pushes | ❌ Disabled | Prevent history rewriting |
| Allow deletions | ❌ Disabled | Prevent branch deletion |

## Required CI Checks

### For `main` branch:

```yaml
required_status_checks:
  strict: true
  contexts:
    - Format           # cargo fmt --check
    - Clippy           # cargo clippy -- -D warnings
    - Test (ubuntu-latest)
    - Test (windows-latest)
    - Test (macos-latest)
    - Code Coverage    # ≥85% threshold
```

### For `develop` branch:

```yaml
required_status_checks:
  strict: true
  contexts:
    - Format
    - Clippy
    - Test (ubuntu-latest)
```

## Coverage Requirements

- **Minimum line coverage**: 85%
- **Patch coverage**: 85% (for PRs)
- **Coverage gate**: PRs will be blocked if coverage drops below threshold

## GitHub Repository Settings

Configure these in **Settings → Branches → Branch protection rules**:

### Step-by-step Configuration

1. Go to repository Settings
2. Click "Branches" in the left sidebar
3. Click "Add rule" for each protected branch
4. Configure as specified above
5. Click "Create" or "Save changes"

### Required Status Checks Configuration

In the branch protection rule:

1. ✅ Check "Require status checks to pass before merging"
2. ✅ Check "Require branches to be up to date before merging"
3. Search and add each required check:
   - `fmt`
   - `clippy`
   - `Test (ubuntu-latest)`
   - `Test (windows-latest)`
   - `Test (macos-latest)`
   - `Code Coverage`

## Merge Strategy

- **Preferred**: Squash and merge
- **Commit message format**: Use PR title as commit message
- **Delete head branches**: ✅ Enabled (auto-delete after merge)

## Emergency Procedures

If you need to bypass protection rules (emergency hotfix):

1. Contact a repository administrator
2. Administrator can temporarily disable rules
3. Apply fix directly to `main`
4. Re-enable protection rules immediately
5. Document the incident

## See Also

- [CONTRIBUTING.md](./CONTRIBUTING.md) - Contribution guidelines
- [CI/CD Workflows](../.github/workflows/) - GitHub Actions configuration
