# Merge Strategy for Android/Arch Branch

## Merging from main/develop
```bash
# Update from main while keeping platform restrictions
git checkout feature/android-arch-only
git merge main --strategy-option=ours --no-commit
# Review changes, keep only Android/Arch relevant updates
git commit -m "Merge main, maintaining platform restrictions"