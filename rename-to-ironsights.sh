#!/bin/bash
# rename-to-ironsights.sh
# Complete script to rename IronSights to IronSights

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Renaming Project to IronSights       ${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found. Please run from project root.${NC}"
    exit 1
fi

# Backup important files
echo -e "${YELLOW}Creating backup...${NC}"
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"
cp -r Cargo.toml Cargo.lock README.md PKGBUILD package.json "$BACKUP_DIR/" 2>/dev/null || true
echo -e "${GREEN}✓ Backup created in $BACKUP_DIR${NC}"

# Function to replace text in files
replace_in_files() {
    local search="$1"
    local replace="$2"
    local file_pattern="$3"
    
    echo -e "${BLUE}  Replacing '$search' with '$replace' in $file_pattern files...${NC}"
    
    find . -type f -name "$file_pattern" \
        -not -path "./.git/*" \
        -not -path "./target/*" \
        -not -path "./node_modules/*" \
        -not -path "./$BACKUP_DIR/*" \
        -exec sed -i "s|$search|$replace|g" {} \;
}

# Function to rename files
rename_files() {
    local old_pattern="$1"
    local new_pattern="$2"
    
    echo -e "${BLUE}  Renaming files from '$old_pattern' to '$new_pattern'...${NC}"
    
    find . -type f -name "*$old_pattern*" \
        -not -path "./.git/*" \
        -not -path "./target/*" \
        -not -path "./node_modules/*" \
        -not -path "./$BACKUP_DIR/*" | while read -r file; do
        newfile="${file//$old_pattern/$new_pattern}"
        if [ "$file" != "$newfile" ]; then
            mv "$file" "$newfile"
            echo "    Renamed: $(basename "$file") → $(basename "$newfile")"
        fi
    done
}

# Function to rename directories
rename_dirs() {
    local old_pattern="$1"
    local new_pattern="$2"
    
    echo -e "${BLUE}  Renaming directories from '$old_pattern' to '$new_pattern'...${NC}"
    
    find . -type d -name "*$old_pattern*" \
        -not -path "./.git/*" \
        -not -path "./target/*" \
        -not -path "./node_modules/*" \
        -not -path "./$BACKUP_DIR/*" \
        -depth | while read -r dir; do
        newdir="${dir//$old_pattern/$new_pattern}"
        if [ "$dir" != "$newdir" ]; then
            mv "$dir" "$newdir"
            echo "    Renamed: $(basename "$dir") → $(basename "$newdir")"
        fi
    done
}

echo -e "${YELLOW}Step 1: Updating text in all files...${NC}"

# Replace in Rust files
replace_in_files "IronSights" "IronSights" "*.rs"
replace_in_files "ironsights" "ironsights" "*.rs"
replace_in_files "ironsights" "ironsights" "*.rs"
replace_in_files "BallisticsAnalyzer" "IronSights" "*.rs"
replace_in_files "BALLISTICS_ANALYZER" "IRONSIGHTS" "*.rs"

# Replace in TOML files
replace_in_files "IronSights" "IronSights" "*.toml"
replace_in_files "ironsights" "ironsights" "*.toml"
replace_in_files "ironsights" "ironsights" "*.toml"
replace_in_files "ballistics-desktop" "ironsights-desktop" "*.toml"
replace_in_files "ballistics-mobile" "ironsights-mobile" "*.toml"
replace_in_files "ballistics_core" "ironsights_core" "*.toml"

# Replace in Cargo.toml workspace members
sed -i 's|"ballistics_core"|"ironsights_core"|g' Cargo.toml
sed -i 's|"ballistics-desktop"|"ironsights-desktop"|g' Cargo.toml
sed -i 's|"ballistics-mobile"|"ironsights-mobile"|g' Cargo.toml

# Replace in JSON files
replace_in_files "ironsights" "ironsights" "*.json"
replace_in_files "IronSights" "IronSights" "*.json"

# Replace in Markdown files
replace_in_files "IronSights" "IronSights" "*.md"
replace_in_files "ironsights" "ironsights" "*.md"
replace_in_files "Ballistics-Analyzer" "IronSights" "*.md"
replace_in_files "BALLISTICS ANALYZER" "IRONSIGHTS" "*.md"

# Replace in Shell scripts
replace_in_files "ironsights" "ironsights" "*.sh"
replace_in_files "ironsights" "ironsights" "*.sh"
replace_in_files "IronSights" "IronSights" "*.sh"

# Replace in Makefiles
replace_in_files "ironsights" "ironsights" "Makefile"
replace_in_files "ballistics-desktop" "ironsights-desktop" "Makefile"
replace_in_files "ballistics-mobile" "ironsights-mobile" "Makefile"
replace_in_files "IronSights" "IronSights" "Makefile"

# Replace in PKGBUILD
if [ -f "PKGBUILD" ]; then
    sed -i 's|pkgname=ironsights|pkgname=ironsights|g' PKGBUILD
    sed -i 's|pkgdesc=".*"|pkgdesc="Professional ballistics calculator for precision shooting"|g' PKGBUILD
    sed -i 's|ironsights|ironsights|g' PKGBUILD
    sed -i 's|IronSights|IronSights|g' PKGBUILD
fi

# Replace in YAML files (GitHub Actions)
replace_in_files "IronSights" "IronSights" "*.yml"
replace_in_files "ironsights" "ironsights" "*.yml"
replace_in_files "ballistics-desktop" "ironsights-desktop" "*.yml"
replace_in_files "ballistics-mobile" "ironsights-mobile" "*.yml"

# Replace in HTML files
replace_in_files "IronSights" "IronSights" "*.html"
replace_in_files "ironsights" "ironsights" "*.html"

# Replace in manifest files
replace_in_files "IronSights" "IronSights" "*.webmanifest"
replace_in_files "ironsights" "ironsights" "*.webmanifest"
replace_in_files "IronSights" "IronSights" "manifest.json"
replace_in_files "ironsights" "ironsights" "manifest.json"

# Replace in Android files
replace_in_files "ironsights" "ironsights" "*.xml"
replace_in_files "BallisticsAnalyzer" "IronSights" "*.xml"
replace_in_files "ballistics.analyzer" "ironsights.app" "*.xml"
replace_in_files "ironsights" "ironsights" "*.kt"
replace_in_files "ironsights" "ironsights" "*.java"
replace_in_files "BallisticsAnalyzer" "IronSights" "*.kt"
replace_in_files "BallisticsAnalyzer" "IronSights" "*.java"

# Replace in Gradle files
replace_in_files "ironsights" "ironsights" "*.gradle"
replace_in_files "ironsights" "ironsights" "*.gradle"
replace_in_files "com.ballistics.analyzer" "com.ironsights.app" "*.gradle"

echo -e "${GREEN}✓ Text replacements complete${NC}"

echo -e "${YELLOW}Step 2: Renaming directories...${NC}"

# Rename main module directories
if [ -d "ballistics_core" ]; then
    mv ballistics_core ironsights_core
    echo -e "${GREEN}  ✓ Renamed ballistics_core → ironsights_core${NC}"
fi

if [ -d "ballistics-desktop" ]; then
    mv ballistics-desktop ironsights-desktop
    echo -e "${GREEN}  ✓ Renamed ballistics-desktop → ironsights-desktop${NC}"
fi

if [ -d "ballistics-mobile" ]; then
    mv ballistics-mobile ironsights-mobile
    echo -e "${GREEN}  ✓ Renamed ballistics-mobile → ironsights-mobile${NC}"
fi

# Rename any other directories with old naming
rename_dirs "ironsights" "ironsights"
rename_dirs "ironsights" "ironsights"

echo -e "${GREEN}✓ Directory renaming complete${NC}"

echo -e "${YELLOW}Step 3: Renaming files...${NC}"

# Rename executable and binary names
rename_files "ironsights" "ironsights"
rename_files "ironsights" "ironsights"

echo -e "${GREEN}✓ File renaming complete${NC}"

echo -e "${YELLOW}Step 4: Updating Git repository settings...${NC}"

# Update git remote URL if it contains the old name
if git remote -v | grep -q "ironsights"; then
    OLD_URL=$(git remote get-url origin)
    NEW_URL="${OLD_URL//ironsights/ironsights}"
    NEW_URL="${NEW_URL//Ballistics-Analyzer/IronSights}"
    
    echo -e "${BLUE}  Current origin: $OLD_URL${NC}"
    echo -e "${BLUE}  New origin: $NEW_URL${NC}"
    echo -e "${YELLOW}  Note: You'll need to rename the repository on GitHub/GitLab${NC}"
    
    read -p "  Update git remote URL? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git remote set-url origin "$NEW_URL"
        echo -e "${GREEN}  ✓ Git remote updated${NC}"
    fi
fi

echo -e "${GREEN}✓ Git configuration complete${NC}"

echo -e "${YELLOW}Step 5: Updating package names in dependency paths...${NC}"

# Fix internal dependency paths in Cargo.toml files
for toml_file in $(find . -name "Cargo.toml" -not -path "./target/*" -not -path "./$BACKUP_DIR/*"); do
    sed -i 's|path = "../ballistics_core"|path = "../ironsights_core"|g' "$toml_file"
    sed -i 's|path = "../ballistics-desktop"|path = "../ironsights-desktop"|g' "$toml_file"
    sed -i 's|path = "../ballistics-mobile"|path = "../ironsights-mobile"|g' "$toml_file"
done

echo -e "${GREEN}✓ Dependency paths updated${NC}"

echo -e "${YELLOW}Step 6: Creating new README header...${NC}"

# Update README.md with new branding
if [ -f "README.md" ]; then
    cat > README_new.md << 'EOF'
# 🎯 IronSights

<div align="center">

![IronSights Logo](assets/logo.png)

**Professional Ballistics Calculator for Precision Shooting**

[![Build Status](https://github.com/yourusername/ironsights/workflows/Build/badge.svg)](https://github.com/yourusername/ironsights/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Arch%20Linux%20%7C%20Android-blue)]()

</div>

---

EOF
    
    # Append the rest of the original README (skip old header)
    tail -n +20 README.md >> README_new.md
    mv README_new.md README.md
    
    echo -e "${GREEN}✓ README updated${NC}"
fi

echo -e "${YELLOW}Step 7: Cleaning up and finalizing...${NC}"

# Clean Cargo.lock to prevent conflicts
if [ -f "Cargo.lock" ]; then
    rm Cargo.lock
    echo -e "${BLUE}  Removed Cargo.lock (will regenerate on next build)${NC}"
fi

# Create a migration summary
cat > MIGRATION_SUMMARY.md << EOF
# Migration Summary: IronSights → IronSights

## Changes Made

### Directories Renamed:
- \`ballistics_core\` → \`ironsights_core\`
- \`ballistics-desktop\` → \`ironsights-desktop\`
- \`ballistics-mobile\` → \`ironsights-mobile\`

### Text Replacements:
- "IronSights" → "IronSights"
- "ironsights" → "ironsights"
- "ironsights" → "ironsights"
- Package names and dependencies updated

### Next Steps:
1. Rename GitHub/GitLab repository to "ironsights"
2. Update CI/CD pipelines if needed
3. Run \`cargo build\` to verify everything works
4. Update any external documentation or links
5. Update app store listings (if published)

### Backup Location:
- Original files backed up to: \`$BACKUP_DIR\`

---
*Migration completed on $(date)*
EOF

echo -e "${GREEN}✓ Migration summary created${NC}"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  ✨ Migration Complete!               ${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Run 'cargo clean' to clear old build artifacts"
echo "2. Run 'cargo build' to verify the renamed project builds"
echo "3. Rename your GitHub/GitLab repository to 'ironsights'"
echo "4. Update any CI/CD webhooks and secrets"
echo "5. Commit these changes: git add -A && git commit -m 'Rename project to IronSights'"
echo ""
echo -e "${BLUE}Backup saved to: $BACKUP_DIR${NC}"
echo -e "${BLUE}See MIGRATION_SUMMARY.md for details${NC}"
echo ""
echo -e "${GREEN}Your project is now IronSights! 🎯${NC}"