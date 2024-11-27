#!/bin/sh

# Note: This script needs to run from inside /api dir
export METRICS_PORT=8001

cargo run --release
