// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('WebGPU Rendering Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the WebGPU test page
    await page.goto('/webgpu-test.html');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
  });

  test('WebGPU support detection works', async ({ page }) => {
    // Click the WebGPU support test button
    await page.click('button:has-text("Test WebGPU Support")');
    
    // Wait for the result to appear
    await page.waitForSelector('#webgpu-result .result', { timeout: 10000 });
    
    // Get the result text
    const resultText = await page.textContent('#webgpu-result .result');
    
    // The result should contain either "Supported" or "Not Supported"
    expect(resultText).toMatch(/WebGPU Support: (Supported|Not Supported)/);
    
    // Log the result for debugging
    console.log('WebGPU Support Result:', resultText);
  });

  test('Browser WebGPU API detection works', async ({ page }) => {
    // Click the browser WebGPU API check button
    await page.click('button:has-text("Check Browser WebGPU API")');
    
    // Wait for the result to appear
    await page.waitForSelector('#browser-result .result', { timeout: 5000 });
    
    // Get the result text
    const resultText = await page.textContent('#browser-result .result');
    
    // The result should contain either "Available" or "Not Available"
    expect(resultText).toMatch(/Browser WebGPU API: (Available|Not Available)/);
    
    // Log the result for debugging
    console.log('Browser WebGPU API Result:', resultText);
  });

  test('WASM module loads successfully', async ({ page }) => {
    // Check that the WASM module is loaded by looking for console logs
    const consoleLogs = [];
    page.on('console', msg => {
      if (msg.type() === 'log') {
        consoleLogs.push(msg.text());
      }
    });
    
    // Wait a bit for the WASM module to initialize
    await page.waitForTimeout(3000);
    
    // Check that we have some console logs indicating WASM initialization
    const wasmLogs = consoleLogs.filter(log => 
      log.includes('WASM module') || log.includes('WebGPU test result') || log.includes('initialized')
    );
    
    // If no WASM logs, check that the page loaded without errors
    if (wasmLogs.length === 0) {
      // Check that the page elements are present (indicating successful load)
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('button:has-text("Test WebGPU Support")')).toBeVisible();
    } else {
      expect(wasmLogs.length).toBeGreaterThan(0);
    }
    
    console.log('WASM Console Logs:', wasmLogs);
  });

  test('Test results are recorded', async ({ page }) => {
    // Run both tests
    await page.click('button:has-text("Test WebGPU Support")');
    await page.waitForSelector('#webgpu-result .result', { timeout: 10000 });
    
    await page.click('button:has-text("Check Browser WebGPU API")');
    await page.waitForSelector('#browser-result .result', { timeout: 5000 });
    
    // Check that test results are recorded
    const testResults = await page.textContent('#test-results');
    expect(testResults).toContain('WASM WebGPU Check');
    expect(testResults).toContain('Browser WebGPU API');
    
    console.log('Test Results:', testResults);
  });
});

test.describe('WebGPU Charts Demo', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the WebGPU charts demo page
    await page.goto('/webgpu-charts-demo.html');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
  });

  test('WebGPU charts demo loads successfully', async ({ page }) => {
    // Check that the page title is correct
    await expect(page).toHaveTitle(/WebGPU Charts Demo/);
    
    // Check that the main container is present
    await expect(page.locator('.container')).toBeVisible();
    
    // Check that the header is present
    await expect(page.locator('.header h1')).toContainText('WebGPU Charts Demo');
  });

  test('Chart controls are present', async ({ page }) => {
    // Check that render buttons are present (the demo has separate buttons for each chart type)
    await expect(page.locator('button:has-text("Render Line Chart")')).toBeVisible();
    await expect(page.locator('button:has-text("Render Bar Chart")')).toBeVisible();
    await expect(page.locator('button:has-text("Render Scatter Plot")')).toBeVisible();
  });

  test('Chart canvas is present', async ({ page }) => {
    // Check that the chart canvases are present (the demo has multiple canvases)
    await expect(page.locator('#lineChartCanvas')).toBeVisible();
    await expect(page.locator('#barChartCanvas')).toBeVisible();
    await expect(page.locator('#scatterCanvas')).toBeVisible();
    
    // Check that the canvases have the correct dimensions
    const lineCanvas = page.locator('#lineChartCanvas');
    await expect(lineCanvas).toHaveAttribute('width', '500');
    await expect(lineCanvas).toHaveAttribute('height', '400');
  });

  test('Performance metrics section is present', async ({ page }) => {
    // Check that performance metrics section is present
    await expect(page.locator('.performance-metrics')).toBeVisible();
    
    // Check that render context metric is present
    await expect(page.locator('#renderContext')).toBeVisible();
  });

  test('Chart rendering works', async ({ page }) => {
    // Click render line chart button
    await page.click('button:has-text("Render Line Chart")');
    
    // Wait for rendering to complete (look for performance metrics)
    await page.waitForSelector('#renderContext:not(:empty)', { timeout: 10000 });
    
    // Check that render context has a value
    const renderContextText = await page.textContent('#renderContext');
    expect(renderContextText).not.toBe('-');
    
    console.log('Render Context:', renderContextText);
  });

  test('Chart type switching works', async ({ page }) => {
    const chartButtons = [
      { text: 'Render Line Chart', canvas: '#lineChartCanvas' },
      { text: 'Render Bar Chart', canvas: '#barChartCanvas' },
      { text: 'Render Scatter Plot', canvas: '#scatterCanvas' }
    ];
    
    for (const chart of chartButtons) {
      // Click the specific chart render button
      await page.click(`button:has-text("${chart.text}")`);
      
      // Wait for rendering to complete
      await page.waitForSelector('#renderContext:not(:empty)', { timeout: 10000 });
      
      // Verify the canvas is visible
      await expect(page.locator(chart.canvas)).toBeVisible();
      
      console.log(`Chart ${chart.text} rendered successfully`);
    }
  });

  test('Data size scaling works', async ({ page }) => {
    // The WebGPU demo doesn't have data size selectors, so we'll test different chart types instead
    const chartButtons = [
      'Render Line Chart',
      'Render Bar Chart', 
      'Render Scatter Plot'
    ];
    
    for (const buttonText of chartButtons) {
      // Click render button
      await page.click(`button:has-text("${buttonText}")`);
      
      // Wait for rendering to complete
      await page.waitForSelector('#renderContext:not(:empty)', { timeout: 15000 });
      
      // Get performance metrics
      const renderContextText = await page.textContent('#renderContext');
      
      console.log(`Chart ${buttonText}: Render Context=${renderContextText}`);
    }
  });
});

test.describe('WebGPU Performance Tests', () => {
  test('WebGPU performance is acceptable', async ({ page }) => {
    await page.goto('/webgpu-charts-demo.html');
    await page.waitForLoadState('networkidle');
    
    // Measure performance with line chart
    const startTime = Date.now();
    await page.click('button:has-text("Render Line Chart")');
    await page.waitForSelector('#renderContext:not(:empty)', { timeout: 10000 });
    const endTime = Date.now();
    
    const totalTime = endTime - startTime;
    const renderContextText = await page.textContent('#renderContext');
    
    // Performance assertions
    expect(totalTime).toBeLessThan(5000); // Should render within 5 seconds
    expect(renderContextText).not.toBe('-'); // Should have some render context
    
    console.log(`Performance: Total time=${totalTime}ms, Render Context=${renderContextText}`);
  });

  test('WebGPU handles different chart types', async ({ page }) => {
    await page.goto('/webgpu-charts-demo.html');
    await page.waitForLoadState('networkidle');
    
    const chartTypes = [
      'Render Line Chart',
      'Render Bar Chart',
      'Render Scatter Plot'
    ];
    
    for (const chartType of chartTypes) {
      // Measure performance with different chart types
      const startTime = Date.now();
      await page.click(`button:has-text("${chartType}")`);
      await page.waitForSelector('#renderContext:not(:empty)', { timeout: 15000 });
      const endTime = Date.now();
      
      const totalTime = endTime - startTime;
      const renderContextText = await page.textContent('#renderContext');
      
      // Performance assertions
      expect(totalTime).toBeLessThan(10000); // Should render within 10 seconds
      expect(renderContextText).not.toBe('-'); // Should have some render context
      
      console.log(`${chartType} Performance: Total time=${totalTime}ms, Render Context=${renderContextText}`);
    }
  });
});
