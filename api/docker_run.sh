#!/bin/sh

# Note: This script needs to run from inside /api dir
export METRICS_PORT=8001

if [ -z ${SERVICE_VERSION} ]; then
  echo "SERVICE_VERSION env var undefined"
  exit 1
fi

cargo run --release
