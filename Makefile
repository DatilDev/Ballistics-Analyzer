# Makefile for Ballistics Analyzer
# Supports: Linux (Debian/Arch), macOS, Windows

.PHONY: all clean test build install uninstall run desktop mobile core help

# Detect OS
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

# Set OS-specific variables
ifeq ($(UNAME_S),Linux)
    OS := linux
    ifeq ($(shell test -f /etc/debian_version && echo debian),debian)
        DISTRO := debian
    else ifeq ($(shell test -f /etc/arch-release && echo arch),arch)
        DISTRO := arch
    else
        DISTRO := unknown
    endif
    BINARY_EXT :=
    PATH_SEP := /
else ifeq ($(UNAME_S),Darwin)
    OS := macos
    DISTRO := darwin
    BINARY_EXT :=
    PATH_SEP := /
else ifeq ($(OS),Windows_NT)
    OS := windows
    DISTRO := windows
    BINARY_EXT := .exe
    PATH_SEP := \\
else
    OS := unknown
    BINARY_EXT :=
    PATH_SEP := /
endif

# Build configuration
BUILD_TYPE ?= release
CARGO_FLAGS := $(if $(filter release,$(BUILD_TYPE)),--release,)
OUTPUT_DIR := target/$(BUILD_TYPE)

# Targets
DESKTOP_TARGET := ballistics-analyzer$(BINARY_EXT)
CORE_TARGET := libballistics_core.rlib

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(BLUE)Ballistics Analyzer Build System$(NC)"
	@echo "$(GREEN)OS: $(OS) ($(DISTRO)) - Arch: $(UNAME_M)$(NC)"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "Build options:"
	@echo "  BUILD_TYPE=release|debug (default: release)"
	@echo ""
	@echo "Examples:"
	@echo "  make build          # Build everything"
	@echo "  make desktop        # Build desktop app only"
	@echo "  make install        # Install to system"
	@echo "  make clean          # Clean build artifacts"

all: core desktop ## Build everything

clean: ## Clean all build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	@cargo clean
	@rm -rf build/
	@rm -rf pkg/
	@rm -rf AppDir/
	@rm -f *.AppImage
	@echo "$(GREEN)✓ Clean complete$(NC)"

# Core library
core: ## Build core library
	@echo "$(BLUE)Building ballistics_core...$(NC)"
	@cd ballistics_core && cargo build $(CARGO_FLAGS)
	@echo "$(GREEN)✓ Core library built$(NC)"

test-core: ## Test core library
	@echo "$(BLUE)Testing ballistics_core...$(NC)"
	@cd ballistics_core && cargo test $(CARGO_FLAGS)

# Desktop application
desktop: core ## Build desktop application
	@echo "$(BLUE)Building desktop application...$(NC)"
	@cd ballistics-desktop && cargo build $(CARGO_FLAGS)
	@echo "$(GREEN)✓ Desktop application built$(NC)"

test-desktop: ## Test desktop application
	@echo "$(BLUE)Testing desktop application...$(NC)"
	@cd ballistics-desktop && cargo test $(CARGO_FLAGS)

run: desktop ## Run desktop application
	@echo "$(BLUE)Running Ballistics Analyzer...$(NC)"
	@./target/$(BUILD_TYPE)/$(DESKTOP_TARGET)

# Mobile targets
mobile-android: ## Build Android APK
	@echo "$(BLUE)Building Android APK...$(NC)"
	@chmod +x scripts/build-android.sh
	@./scripts/build-android.sh
	@echo "$(GREEN)✓ Android APK built$(NC)"

mobile-ios: ## Build iOS app
	@echo "$(BLUE)Building iOS app...$(NC)"
	@chmod +x scripts/build-ios.sh
	@./scripts/build-ios.sh
	@echo "$(GREEN)✓ iOS app built$(NC)"

# Platform-specific installation
install: desktop ## Install application to system
ifeq ($(OS),linux)
	@echo "$(YELLOW)Installing on Linux...$(NC)"
	@sudo install -Dm755 target/$(BUILD_TYPE)/$(DESKTOP_TARGET) /usr/local/bin/ballistics-analyzer
	@sudo install -Dm644 ballistics-desktop/assets/icon.png /usr/share/icons/hicolor/256x256/apps/ballistics-analyzer.png
	@sudo install -Dm644 pkg/ballistics-analyzer.desktop /usr/share/applications/ballistics-analyzer.desktop
	@sudo update-desktop-database /usr/share/applications 2>/dev/null || true
	@sudo gtk-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true
	@echo "$(GREEN)✓ Installed to /usr/local/bin/ballistics-analyzer$(NC)"
else ifeq ($(OS),macos)
	@echo "$(YELLOW)Installing on macOS...$(NC)"
	@chmod +x scripts/create-macos-bundle.sh
	@./scripts/create-macos-bundle.sh target/$(BUILD_TYPE)/$(DESKTOP_TARGET)
	@cp -r "build/macos/Ballistics Analyzer.app" /Applications/
	@echo "$(GREEN)✓ Installed to /Applications/Ballistics Analyzer.app$(NC)"
else ifeq ($(OS),windows)
	@echo "$(YELLOW)Installing on Windows...$(NC)"
	@powershell -ExecutionPolicy Bypass -File scripts/build-desktop.ps1 -CreateInstaller
	@echo "$(GREEN)✓ Run the installer from build/windows/ballistics-analyzer-setup.exe$(NC)"
endif

uninstall: ## Uninstall application from system
ifeq ($(OS),linux)
	@echo "$(YELLOW)Uninstalling from Linux...$(NC)"
	@sudo rm -f /usr/local/bin/ballistics-analyzer
	@sudo rm -f /usr/share/icons/hicolor/256x256/apps/ballistics-analyzer.png
	@sudo rm -f /usr/share/applications/ballistics-analyzer.desktop
	@sudo update-desktop-database /usr/share/applications 2>/dev/null || true
	@sudo gtk-update-icon-cache /usr/share/icons/hicolor 2>/dev/null || true
	@echo "$(GREEN)✓ Uninstalled$(NC)"
else ifeq ($(OS),macos)
	@echo "$(YELLOW)Uninstalling from macOS...$(NC)"
	@rm -rf "/Applications/Ballistics Analyzer.app"
	@echo "$(GREEN)✓ Uninstalled$(NC)"
else ifeq ($(OS),windows)
	@echo "$(YELLOW)Please use Windows Control Panel to uninstall$(NC)"
endif

# Distribution packages
dist: desktop ## Create distribution package
	@echo "$(YELLOW)Creating distribution package...$(NC)"
ifeq ($(OS),linux)
	@chmod +x scripts/create-appimage.sh
	@./scripts/create-appimage.sh
	@chmod +x scripts/create-pkgbuild.sh
	@./scripts/create-pkgbuild.sh
else ifeq ($(OS),macos)
	@chmod +x scripts/create-macos-bundle.sh
	@./scripts/create-macos-bundle.sh target/$(BUILD_TYPE)/$(DESKTOP_TARGET)
else ifeq ($(OS),windows)
	@powershell -ExecutionPolicy Bypass -File scripts/build-desktop.ps1 -BuildType $(BUILD_TYPE) -CreateInstaller
endif
	@echo "$(GREEN)✓ Distribution package created$(NC)"

# Development targets
dev: ## Run in development mode with auto-reload
	@echo "$(BLUE)Running in development mode...$(NC)"
	@cargo watch -x 'run --bin ballistics-analyzer'

fmt: ## Format code
	@echo "$(YELLOW)Formatting code...$(NC)"
	@cargo fmt --all
	@echo "$(GREEN)✓ Code formatted$(NC)"

lint: ## Run clippy linter
	@echo "$(YELLOW)Running clippy...$(NC)"
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✓ Linting complete$(NC)"

audit: ## Security audit
	@echo "$(YELLOW)Running security audit...$(NC)"
	@cargo audit
	@echo "$(GREEN)✓ Security audit complete$(NC)"

doc: ## Generate documentation
	@echo "$(YELLOW)Generating documentation...$(NC)"
	@cargo doc --no-deps --open
	@echo "$(GREEN)✓ Documentation generated$(NC)"

# Testing targets
test: test-core test-desktop ## Run all tests
	@echo "$(GREEN)✓ All tests passed$(NC)"

bench: ## Run benchmarks
	@echo "$(YELLOW)Running benchmarks...$(NC)"
	@cargo bench
	@echo "$(GREEN)✓ Benchmarks complete$(NC)"

coverage: ## Generate test coverage report
	@echo "$(YELLOW)Generating coverage report...$(NC)"
	@cargo tarpaulin --out Html --output-dir coverage
	@echo "$(GREEN)✓ Coverage report generated in coverage/$(NC)"

# CI/CD targets
ci: fmt lint test audit ## Run CI checks
	@echo "$(GREEN)✓ CI checks passed$(NC)"

# Platform-specific dependency installation
deps: ## Install build dependencies
ifeq ($(OS),linux)
ifeq ($(DISTRO),debian)
	@echo "$(YELLOW)Installing Debian/Ubuntu dependencies...$(NC)"
	@sudo apt-get update
	@sudo apt-get install -y build-essential pkg-config libssl-dev libgtk-3-dev \
		libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev libx11-dev \
		libxcb1-dev libxkbcommon-dev libgl1-mesa-dev libwayland-dev
else ifeq ($(DISTRO),arch)
	@echo "$(YELLOW)Installing Arch Linux dependencies...$(NC)"
	@sudo pacman -Sy --needed base-devel pkg-config openssl gtk3 webkit2gtk \
		libappindicator-gtk3 librsvg libx11 libxcb libxkbcommon mesa wayland
endif
else ifeq ($(OS),macos)
	@echo "$(YELLOW)Checking macOS dependencies...$(NC)"
	@command -v brew >/dev/null 2>&1 || (echo "$(RED)Homebrew not found. Install from https://brew.sh$(NC)" && exit 1)
	@echo "$(GREEN)✓ Dependencies satisfied$(NC)"
else ifeq ($(OS),windows)
	@echo "$(YELLOW)Windows dependencies should be installed via Visual Studio$(NC)"
endif

# Docker targets
docker-build: ## Build Docker image
	@echo "$(YELLOW)Building Docker image...$(NC)"
	@docker build -t ballistics-analyzer:latest .
	@echo "$(GREEN)✓ Docker image built$(NC)"

docker-run: ## Run in Docker container
	@echo "$(YELLOW)Running in Docker...$(NC)"
	@docker run --rm -it -e DISPLAY=$DISPLAY -v /tmp/.X11-unix:/tmp/.X11-unix ballistics-analyzer:latest

# Release targets
release: ## Create release build
	@echo "$(YELLOW)Creating release build...$(NC)"
	@$(MAKE) BUILD_TYPE=release clean
	@$(MAKE) BUILD_TYPE=release all
	@$(MAKE) BUILD_TYPE=release dist
	@echo "$(GREEN)✓ Release build complete$(NC)"

# Version management
version: ## Show version
	@grep "^version" ballistics_core/Cargo.toml | head -1 | cut -d '"' -f2

bump-version: ## Bump version (use VERSION=x.y.z)
ifndef VERSION
	@echo "$(RED)Please specify VERSION=x.y.z$(NC)"
	@exit 1
endif
	@echo "$(YELLOW)Bumping version to $(VERSION)...$(NC)"
	@sed -i 's/^version = .*/version = "$(VERSION)"/' ballistics_core/Cargo.toml
	@sed -i 's/^version = .*/version = "$(VERSION)"/' ballistics-desktop/Cargo.toml
	@sed -i 's/^version = .*/version = "$(VERSION)"/' ballistics-mobile/Cargo.toml
	@echo "$(GREEN)✓ Version bumped to $(VERSION)$(NC)"

# Utility targets
check: ## Check if project builds
	@echo "$(YELLOW)Checking build...$(NC)"
	@cargo check --all
	@echo "$(GREEN)✓ Build check passed$(NC)"

size: ## Show binary sizes
	@echo "$(BLUE)Binary sizes:$(NC)"
	@ls -lh target/*/ballistics-analyzer* 2>/dev/null || echo "No binaries built yet"

loc: ## Count lines of code
	@echo "$(BLUE)Lines of code:$(NC)"
	@tokei . --exclude target --exclude node_modules --exclude build

.DEFAULT_GOAL := help