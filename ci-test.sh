#!/bin/bash

# CI-friendly test script that doesn't require npm-run-all

echo "Running Rust tests..."
cargo test --workspace

if [ $? -ne 0 ]; then
    echo "Rust tests failed"
    exit 1
fi

echo "Running JavaScript tests..."
pnpm -r test

if [ $? -ne 0 ]; then
    echo "JavaScript tests failed"
    exit 1
fi

echo "All tests passed!"