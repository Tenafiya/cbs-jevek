# ==========================================================
# Project
# ==========================================================

PROJECT_NAME := $(shell grep '^name =' Cargo.toml | head -n1 | sed -E 's/name = "(.*)"/\1/')
VERSION := $(shell grep '^version =' Cargo.toml | head -n1 | sed -E 's/version = "(.*)"/\1/')

DEV_COMPOSE := dev.docker-compose.yaml
PROD_COMPOSE := prod.docker-compose.yaml
COMPOSE := podman compose

GREEN  := \033[0;32m
YELLOW := \033[1;33m
BLUE   := \033[0;34m
RESET  := \033[0m

.DEFAULT_GOAL := help

.PHONY: \
help setup \
dev-run dev-start dev-stop dev-down dev-status dev-logs dev-logs-real \
prod-start prod-stop prod-down \
postgres-ip \
seaorm-entity seaorm-status migrate-init migrate-up migrate-down migrate-refresh \
debug release release-lto run-debug run-release run-lto clean \
install-deps \
version-bump \
git-release

# ==========================================================
# Help
# ==========================================================

help:
	@printf "$(BLUE)\n$(PROJECT_NAME)\n$(RESET)"
	@printf "Current Version : $(GREEN)$(VERSION)$(RESET)\n\n"

	@printf "$(YELLOW)Development$(RESET)\n"
	@printf "  make dev-run          Run application with cargo-watch\n"
	@printf "  make dev-start        Start development containers\n"
	@printf "  make dev-stop         Stop development containers\n"
	@printf "  make dev-down         Destroy development containers\n"
	@printf "  make dev-status       Show container status\n"
	@printf "  make dev-logs         Show logs\n"
	@printf "  make dev-logs-real    Follow logs\n\n"

	@printf "$(YELLOW)Production$(RESET)\n"
	@printf "  make prod-start      Run prod podman containers\n"
	@printf "  make prod-stop       Stop prod podman containers\n"
	@printf "  make prod-down       Drop prod podman containers\n\n"

	@printf "$(YELLOW)Cargo$(RESET)\n"
	@printf "  make debug          Build application in debug mode\n"
	@printf "  make release        Build application in release mode\n"
	@printf "  make release-lto    Build application in release lto mode\n"
	@printf "  make run-debug      Run application in debug mode\n"
	@printf "  make run-release    Run application in release mode\n"
	@printf "  make run-lto        Run application in release lto mode\n"
	@printf "  make clean          Clean application\n\n"

	@printf "$(YELLOW)SeaORM$(RESET)\n"
	@printf "  make seaorm-entity  Generate seaorm entity\n"
	@printf "  make seaorm-status  Check seaorm status\n"
	@printf "  make migrate-init   Initialize seaorm cli\n"
	@printf "  make migrate-up     Run seaorm migration in up mode\n"
	@printf "  make migrate-down   Run seaorm migration in down mode\n"
	@printf "  make migrate-refresh Refresh seaorm migration\n\n"

	@printf "$(YELLOW)Git$(RESET)\n"
	@printf "  make git-release   Git release commands\n"
	@printf "  make git-debug     Git debug commands\n\n"

# ==========================================================
# Setup
# ==========================================================

setup:
	cargo init

# ==========================================================
# Validation
# ==========================================================

check-test-env:
	@if [ ! -f app.config.toml ]; then \
		echo "❌ app.config.toml not found."; \
		exit 1; \
	fi

	@ENV=$$(awk -F'"' '/^environment[[:space:]]*=/{print $$2}' app.config.toml); \
	if [ -z "$$ENV" ]; then \
		echo "❌ Could not determine environment from app.config.toml."; \
		exit 1; \
	fi; \
	if [ "$$ENV" != "TEST" ]; then \
		echo "❌ install-deps is only allowed when environment = \"TEST\"."; \
		echo "Current environment: $$ENV"; \
		exit 1; \
	fi

# ==========================================================
# Cargo
# ==========================================================

dev-run:
	cargo watch -c -w src -x check -x run --env RUST_LOG=debug --env RUST_BACKTRACE=full

debug:
	cargo build

release:
	cargo build --release

release-lto:
	cargo build --profile release-lto

run-debug:
	cargo run

run-release:
	cargo run --release

run-lto:
	cargo run --profile release-lto

clean:
	cargo clean

# ==========================================================
# Containers
# ==========================================================

dev-start:
	$(COMPOSE) -f $(DEV_COMPOSE) up -d

dev-stop:
	$(COMPOSE) -f $(DEV_COMPOSE) stop

dev-down:
	$(COMPOSE) -f $(DEV_COMPOSE) down -v
	podman image prune -af
	podman volume prune -f

dev-status:
	$(COMPOSE) -f $(DEV_COMPOSE) ps

dev-logs:
	$(COMPOSE) -f $(DEV_COMPOSE) logs

dev-logs-real:
	$(COMPOSE) -f $(DEV_COMPOSE) logs -f

prod-start:
	$(COMPOSE) -f $(PROD_COMPOSE) up -d

prod-stop:
	$(COMPOSE) -f $(PROD_COMPOSE) stop

prod-down:
	$(COMPOSE) -f $(PROD_COMPOSE) down -v
	podman image prune -af
	podman volume prune -f

postgres-ip:
	podman inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' postgres

# ==========================================================
# SeaORM
# ==========================================================

seaorm-entity:
	sea-orm-cli generate entity -o entity/src --with-serde both

seaorm-status:
	sea-orm-cli migrate status

migrate-init:
	sea-orm-cli migrate init

migrate-up:
	sea-orm-cli migrate up

migrate-down:
	sea-orm-cli migrate down

migrate-refresh:
	sea-orm-cli migrate refresh

# ==========================================================
# Version
# ==========================================================

version-bump:
	@echo "Bumping Cargo.toml version..."
	@awk '\
	BEGIN{FS=OFS="\""} \
	/^version =/ { \
		split($2,v,"."); \
		v[3]++; \
		$$2=v[1]"."v[2]"."v[3]; \
	} \
	{print}' Cargo.toml > Cargo.toml.tmp
	@mv Cargo.toml.tmp Cargo.toml
	@echo "New Version: $$(grep '^version =' Cargo.toml | cut -d'"' -f2)"

# ==========================================================
# Git
# ==========================================================

git-release: version-bump
	@printf "Commit message: "
	@read MSG; \
	git status && \
	git add . && \
	git commit -m "$$MSG" && \
	git push -u origin main

git-debug:
	@printf "Commit message: "
	@read MSG; \
	git status && \
	git add . && \
	git commit -m "$$MSG" && \
	git push -u origin main

# ==========================================================
# Installer
# ==========================================================

install-deps: check-test-env
	cargo add serde --features "derive"
	cargo add actix-web snowflake_me anyhow actix-rt actix-http chrono thiserror tracing-subscriber chrono-tz futures-util futures actix-cors tokio-util tracing config regex rand env_logger log serde_json serde_with sha2 md5 hex bcrypt base64 dotenvy aes-gcm lettre once_cell
	cargo add uuid --features "v4 fast-rng macro-diagnostics"
	cargo add sea-orm --features "sqlx-postgres runtime-tokio-rustls macros"
	cargo add tokio --features "full"
	cargo add validator --features "derive"
	cargo add reqwest --features json
	cargo add openssl --features "vendored"
	cargo add jsonwebtoken --features "rust_crypto"
	cargo add tracing-subscriber --features "env-filter fmt json"