PROJECT_NAME := $(shell grep '^name =' Cargo.toml | head -n 1 | sed -E 's/name = "(.*)"/\1/')

setup:
	cargo init

dev-run:
	cargo watch -c -w src -x check -x test -x run --env RUST_LOG=debug --env RUST_BACKTRACE=full


prod-build:
	cargo build --release

dev-start:
	podman-compose -f dev.docker-compose.yaml up -d

dev-down:
	podman-compose -f dev.docker-compose.yaml down -v && \
	podman rmi -a && \
	podman volume prune -f

dev-stop:
	podman-compose -f dev.docker-compose.yaml stop

dev-logs:
	podman-compose -f dev.docker-compose.yaml logs

dev-logs-real:
	podman-compose -f dev.docker-compose.yaml logs -f

dev-status:
	podman-compose -f dev.docker-compose.yaml ps

prod-start:
	podman-compose -f prod.docker-compose.yaml up -d

prod-stop:
	podman-compose -f prod.docker-compose.yaml down

prod-down:
	podman-compose -f prod.docker-compose.yaml down -v && \
	podman rm -all && \
	podman volume prune -f

postgres-ip:
	podman inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' postgres

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

install-deps:
	cargo add actix-web actix-rt actix-http chrono thiserror tracing-subscriber chrono-tz futures-util futures actix-cors tokio-util tracing config regex rand env_logger log serde_json sha2 md5 hex bcrypt base64 dotenvy aes-gcm lettre once_cell && \
	cargo add uuid --features "v4 fast-rng macro-diagnostics" && \
	cargo add serde --features "derive" && \
	cargo add sea-orm --features "sqlx-postgres runtime-tokio-rustls macros" && \
	cargo add tokio --features "full" && \
	cargo add validator --features "derive" && \
	cargo add reqwest --features json && \
	cargo add openssl --features "vendored" && \
	cargo add jsonwebtoken --features "rust_crypto" && \
	cargo add tracing-subscriber --features "env-filter fmt json"
