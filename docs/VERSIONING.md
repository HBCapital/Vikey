# Versioning Guide

Vikey follows [Semantic Versioning 2.0.0](https://semver.org/).

## Version Format: MAJOR.MINOR.PATCH

### MAJOR (Currently: 0)

- Increment when making incompatible API changes
- Version 0.x.x indicates pre-release/unstable API
- Will become 1.0.0 when API is stable

### MINOR

- Increment when adding functionality in a backward-compatible manner
- Reset PATCH to 0 when incrementing MINOR

### PATCH

- Increment when making backward-compatible bug fixes
- Small improvements and fixes

## Current Version: 0.2.0

### Version History

**0.2.0** (2025-12-05)

- Major refactor: Monorepo architecture
- Trait-based plugin system
- Separated vikey-core (generic) from vikey-vietnamese

**0.1.0** (2025-12-04)

- Initial implementation
- Basic Vietnamese input support

## Automatic Versioning

When pushing to git:

- **Bug fixes, small changes** → Increment PATCH (0.2.0 → 0.2.1)
- **New features, refactors** → Increment MINOR (0.2.0 → 0.3.0)
- **Breaking changes** → Increment MAJOR (0.2.0 → 1.0.0)

Update version in:

1. `crates/vikey-core/Cargo.toml`
2. `crates/vikey-vietnamese/Cargo.toml`
3. `crates/vikey-nom/Cargo.toml`
4. `CHANGELOG.md`

## Commit Message Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: Add new feature (MINOR bump)
fix: Bug fix (PATCH bump)
refactor: Code refactoring (MINOR bump)
docs: Documentation only (PATCH bump)
chore: Maintenance (PATCH bump)

BREAKING CHANGE: ... (MAJOR bump)
```

## Example

```bash
# Bug fix: 0.2.0 → 0.2.1
git commit -m "fix(core): resolve buffer overflow issue"

# New feature: 0.2.1 → 0.3.0
git commit -m "feat(vietnamese): add VNI double-key support"

# Breaking change: 0.3.0 → 1.0.0
git commit -m "feat(core): redesign Engine API

BREAKING CHANGE: Engine.process() now returns Result instead of Action"
```
