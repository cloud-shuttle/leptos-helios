# End-to-End Tests for Leptos Helios

This directory contains Playwright-based end-to-end tests for the Leptos Helios visualization library.

## Test Structure

### WebGPU Tests (`webgpu.spec.js`)
- **WebGPU Support Detection**: Tests WebGPU API availability and support detection
- **WebGPU Charts Demo**: Tests visual WebGPU chart rendering functionality
- **Performance Tests**: Tests WebGPU performance with different data sizes
- **Cross-browser Compatibility**: Tests WebGPU functionality across different browsers

### Canvas2D Tests (`canvas2d.spec.js`)
- **Canvas2D Rendering**: Tests Canvas2D fallback rendering functionality
- **Chart Types**: Tests line charts, bar charts, and scatter plots
- **Performance**: Tests rendering performance with different data sizes
- **Fallback Behavior**: Tests Canvas2D as fallback when WebGPU is not available

### WASM Integration Tests (`wasm-integration.spec.js`)
- **WASM Module Loading**: Tests WebAssembly module initialization
- **Function Calls**: Tests WASM function calls (greet, data processing, chart creation)
- **WebGPU Detection**: Tests WebGPU support detection via WASM
- **Error Handling**: Tests graceful error handling in WASM functions

## Running Tests

### Prerequisites
```bash
# Install dependencies with pnpm
pnpm install

# Install Playwright browsers
pnpm run install:playwright
```

### Running All Tests
```bash
# Run all E2E tests
pnpm run test:e2e

# Run tests with UI
pnpm run test:e2e:ui

# Run tests in headed mode (visible browser)
pnpm run test:e2e:headed

# Debug tests
pnpm run test:e2e:debug
```

### Running Specific Test Suites
```bash
# Run only WebGPU tests
pnpm exec playwright test webgpu

# Run only Canvas2D tests
pnpm exec playwright test canvas2d

# Run only WASM integration tests
pnpm exec playwright test wasm-integration
```

### Running Tests on Specific Browsers
```bash
# Run tests on Chrome only
pnpm exec playwright test --project=chromium

# Run tests on Firefox only
pnpm exec playwright test --project=firefox

# Run tests on Safari only
pnpm exec playwright test --project=webkit
```

## Test Configuration

The tests are configured in `playwright.config.js` with the following features:

- **Multi-browser Testing**: Chrome, Firefox, Safari, Edge
- **Mobile Testing**: Mobile Chrome and Safari
- **Automatic Server**: Starts the demo server automatically
- **Screenshots**: Takes screenshots on test failures
- **Videos**: Records videos of failed tests
- **Traces**: Collects traces for debugging failed tests

## Test Data

Tests use the following demo pages:
- `/webgpu-test.html` - WebGPU support detection
- `/webgpu-charts-demo.html` - Visual WebGPU chart rendering
- `/canvas2d-demo.html` - Canvas2D chart rendering
- `/example.html` - WASM integration example

## Performance Benchmarks

The tests include performance benchmarks for:
- **Rendering Time**: Time to render charts with different data sizes
- **FPS**: Frames per second for interactive charts
- **Memory Usage**: Memory consumption during rendering
- **Cross-browser Performance**: Performance comparison across browsers

## Continuous Integration

These tests are designed to run in CI environments:
- **Headless Mode**: Runs without GUI in CI
- **Retry Logic**: Retries failed tests on CI
- **Parallel Execution**: Runs tests in parallel for speed
- **Artifact Collection**: Collects screenshots, videos, and traces

## Debugging

### Viewing Test Results
```bash
# Open test results in browser
npx playwright show-report
```

### Debugging Failed Tests
```bash
# Run specific test in debug mode
npx playwright test webgpu --debug

# Run with trace viewer
npx playwright test --trace on
```

### Common Issues
1. **WebGPU Not Available**: Some tests may fail if WebGPU is not supported
2. **Server Not Running**: Ensure the demo server is running on port 8080
3. **Browser Compatibility**: Some features may not work in all browsers

## Adding New Tests

To add new E2E tests:

1. Create a new test file in `tests/e2e/`
2. Follow the existing test structure
3. Use the `test.describe()` and `test()` functions
4. Add appropriate assertions and timeouts
5. Update this README with test descriptions

## Test Maintenance

- **Regular Updates**: Update tests when demo pages change
- **Browser Updates**: Test with new browser versions
- **Performance Monitoring**: Monitor test execution times
- **Flaky Test Handling**: Identify and fix flaky tests
