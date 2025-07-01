#!/bin/bash

set -e

cargo run -- \
  artifacts/cw20_base.wasm \
  creator \
  data/instantiate.json \
  data/exec.json \
  data/query.json
