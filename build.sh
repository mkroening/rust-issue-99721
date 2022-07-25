#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

rustc --crate-type staticlib staticlib.rs
rustc -L native=. -l static=staticlib bin.rs

readelf -Wn bin
