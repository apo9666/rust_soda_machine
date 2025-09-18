# Soda Machine Tests

This crate contains integration tests for the Soda Machine project.

## Project Structure

```
soda_test/
├── src/
│   └── lib.rs         # Main test file containing integration tests
└── Cargo.toml         # Project configuration and dependencies
```

## Running Tests

To run all tests in this crate:

```bash
cargo test
```

To run tests with output:

```bash
cargo test -- --nocapture
```

To run a specific test:

```bash
cargo test test_name
```

## Test Categories

- Customer Service Tests: Tests related to customer operations like balance management
- More categories will be added as the test suite grows

## Dependencies

- `mockall`: For creating mock objects in tests
- `assert_cmd`: For testing CLI applications
- `predicates`: For creating test assertions
- `tokio`: For async testing support