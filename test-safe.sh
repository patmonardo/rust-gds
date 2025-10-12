#!/bin/bash
# Safe test runner with limited parallelism to prevent memory spikes
# Use this instead of bare `cargo test` when running full test suite

set -e

echo "🧪 Running rust-gds tests with memory-safe configuration..."
echo "   Total tests: ~1454"
echo "   Parallelism: 4 threads (prevents memory spikes)"
echo ""

# Run with limited parallelism
cargo test --lib --features core -- --test-threads=4 "$@"

echo ""
echo "✅ Tests complete!"
