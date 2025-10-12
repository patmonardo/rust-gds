#!/bin/bash
# Safe test runner with limited parallelism to prevent memory spikes
# Use this instead of bare `cargo test` when running full test suite
#
# Usage:
#   ./test-safe.sh              # Run with 4 threads (default)
#   ./test-safe.sh --serial     # Run serially (1 thread) for debugging
#   ./test-safe.sh <filter>     # Run specific tests with 4 threads

set -e

# Check for serial flag
THREADS=4
if [[ "$1" == "--serial" ]]; then
    THREADS=1
    shift  # Remove --serial from args
    echo "🐌 Running rust-gds tests SERIALLY (1 thread at a time)..."
    echo "   This is slow but good for isolating flaky tests"
else
    echo "🧪 Running rust-gds tests with memory-safe configuration..."
    echo "   Parallelism: 4 threads (prevents memory spikes)"
fi

echo "   Total tests: ~1454"
echo ""

# Run with configured parallelism
cargo test --lib --features core -- --test-threads=$THREADS "$@"

echo ""
echo "✅ Tests complete!"
