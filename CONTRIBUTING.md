# Contributing

## Commit Message Format

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automatic version management and changelog generation.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc.)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries

### Examples

```
feat: add dark mode support
fix: resolve memory leak in CPU monitoring
docs: update README with installation instructions
refactor: simplify status bar update logic
perf: optimize CPU usage calculation
```

### Version Bumping

- **PATCH** (0.0.x): `fix:` commits
- **MINOR** (0.x.0): `feat:` commits
- **MAJOR** (x.0.0): Breaking changes (include `BREAKING CHANGE:` in footer)

### Releasing

Versions are automatically created when commits are pushed to the `main` branch. The release workflow will:

1. Analyze commit messages
2. Determine the appropriate version bump
3. Create a new tag
4. Build and release the application
5. Update CHANGELOG.md
