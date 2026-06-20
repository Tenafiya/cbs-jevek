#!/usr/bin/env bash

set -Eeuo pipefail

###############################################################################
# Configuration
###############################################################################

SERVICE_NAME="cbs-jevek.service"
PROCESS_NAME="cbs-jevek"

PROJECT_DIR="$HOME/cbs-jevek"

SOURCE_BINARY="$PROJECT_DIR/target/release/cbs-jevek"
TARGET_BINARY="/opt/cbs-api"

DEFAULT_SSH_KEY="$HOME/.ssh/id"

###############################################################################
# Helpers
###############################################################################

timestamp() {
    date '+%Y-%m-%d %H:%M:%S'
}

log() {
    echo "[$(timestamp)] [INFO] $*"
}

warn() {
    echo "[$(timestamp)] [WARN] $*" >&2
}

error() {
    echo "[$(timestamp)] [ERROR] $*" >&2
}

die() {
    error "$*"
    exit 1
}

on_error() {
    local exit_code=$?

    error "Deployment failed."
    error "Line: ${BASH_LINENO[0]}"
    error "Command: ${BASH_COMMAND}"

    exit "$exit_code"
}

trap on_error ERR

require_command() {
    command -v "$1" >/dev/null 2>&1 || die "Missing command: $1"
}

###############################################################################
# Validation
###############################################################################

validate_environment() {
    require_command git
    require_command cargo
    require_command ssh-add
    require_command systemctl
    require_command sudo

    [ -d "$PROJECT_DIR" ] || die "Project directory not found: $PROJECT_DIR"
}

###############################################################################
# Stages
###############################################################################

stage_prepare() {

    log "Starting SSH agent"

    if [ -z "${SSH_AUTH_SOCK:-}" ]; then
        eval "$(ssh-agent -s)" >/dev/null
    fi

    read -rp "SSH key path [$DEFAULT_SSH_KEY]: " SSH_KEY

    SSH_KEY="${SSH_KEY:-$DEFAULT_SSH_KEY}"

    [ -f "$SSH_KEY" ] || die "SSH key not found: $SSH_KEY"

    ssh-add "$SSH_KEY"

    log "SSH key loaded"
}

stage_update() {

    log "Updating source code"

    cd "$PROJECT_DIR"

    git fetch origin

    git pull --ff-only origin main

    log "Repository updated"
}

stage_build() {

    log "Building release binary"

    cd "$PROJECT_DIR"

    cargo build --release

    [ -f "$SOURCE_BINARY" ] || die "Build succeeded but binary not found"

    log "Build completed"
}

stage_deploy() {

    log "Checking release binary"

    [ -f "$SOURCE_BINARY" ] || die "Release binary not found"

    log "Stopping service"

    sudo systemctl stop "$SERVICE_NAME"

    log "Killing remaining processes"

    pkill -f "$PROCESS_NAME" || true

    sleep 2

    log "Installing new binary"

    sudo install \
        -m 755 \
        "$SOURCE_BINARY" \
        "$TARGET_BINARY"

    log "Starting service"

    sudo systemctl start "$SERVICE_NAME"

    sleep 3

    log "Verifying service"

    sudo systemctl is-active --quiet "$SERVICE_NAME" \
        || die "Service failed to start"

    log "Deployment successful"
}

stage_logs() {

    log "Opening logs"

    if command -v cbs-jevek-logs >/dev/null 2>&1; then
        cbs-jevek-logs
        return
    fi

    if command -v ccze >/dev/null 2>&1; then
        journalctl -u "$SERVICE_NAME" -f --output=cat | ccze -A
    else
        journalctl -u "$SERVICE_NAME" -f --output=cat
    fi
}

stage_full() {

    stage_prepare
    stage_update
    stage_build
    stage_deploy
    stage_logs
}

###############################################################################
# Usage
###############################################################################

usage() {
    cat <<EOF

Usage:

  ./deploy.sh --stage <stage>

Available stages:

  prepare
      Start ssh-agent and load deployment key

  update
      Pull latest code

  build
      Build release binary

  deploy
      Stop service, replace binary and restart

  logs
      Open application logs

  full
      Execute complete deployment pipeline

Examples:

  ./deploy.sh --stage full

  ./deploy.sh --stage build

  ./deploy.sh --stage deploy

  ./deploy.sh --stage logs

EOF
}

###############################################################################
# Main
###############################################################################

validate_environment

if [ $# -ne 2 ]; then
    usage
    exit 1
fi

if [ "$1" != "--stage" ]; then
    usage
    exit 1
fi

case "$2" in
    prepare)
        stage_prepare
        ;;
    update)
        stage_update
        ;;
    build)
        stage_build
        ;;
    deploy)
        stage_deploy
        ;;
    logs)
        stage_logs
        ;;
    full)
        stage_full
        ;;
    *)
        usage
        exit 1
        ;;
esac

log "Done"