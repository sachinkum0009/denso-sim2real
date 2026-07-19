# Release Checklist

1. Update the repo
  1. `just check`
  2. Update `README.md` changelog
  3. Update version in Cargo.toml
  4. commit/push

2. Tag:

```bash
git tag -a X.X.X -m "release vX.X.X"
git push orign main vX.X.X
```
