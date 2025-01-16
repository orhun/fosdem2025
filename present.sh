#!/usr/bin/env bash

if ! command -v wezterm &>/dev/null; then
  echo "wezterm is not installed. Please install it first."
  exit 1
fi

if ! command -v presenterm &>/dev/null; then
  echo "installing presenterm..."
  cargo install --git https://github.com/mfontanini/presenterm
  exit 1
fi

[ -z "$PRESENT_MODE" ] && args="" || args="-p"
wezterm --config-file config/wezterm.lua \
  start presenterm \
  "$PWD"/presentation.md \
  --config-file "$PWD"/config/presenterm.yml \
  -X $args &
