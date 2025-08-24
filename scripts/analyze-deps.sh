#!/bin/bash
# analyze-deps.sh

echo "Analyzing actual dependency requirements..."

# Check what each crate actually uses
for crate in ironsights_core ironsights-desktop ironsights-mobile; do
    if [ -f "$crate/Cargo.toml" ]; then
        echo ""
        echo "=== $crate dependencies ==="
        grep -A 50 "\[dependencies\]" "$crate/Cargo.toml" | grep "workspace = true" | sed 's/.*\[\(.*\)\].*/\1/' | sed 's/ .*//'
    fi
done

echo ""
echo "Now check if any are missing from the workspace..."