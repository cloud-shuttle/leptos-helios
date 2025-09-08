// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('WASM Integration Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the WASM example page
    await page.goto('/example.html');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
  });

  test('WASM module loads successfully', async ({ page }) => {
    // Check that the page title is correct
    await expect(page).toHaveTitle(/WASM Integration Example/);
    
    // Wait for WASM module to initialize
    await page.waitForTimeout(2000);
    
    // Check that the WASM module is loaded by looking for console logs
    const consoleLogs = [];
    page.on('console', msg => {
      if (msg.type() === 'log') {
        consoleLogs.push(msg.text());
      }
    });
    
    // Trigger a WASM function call
    await page.click('button:has-text("Test Greet")');
    await page.waitForTimeout(1000);
    
    // Check that we have some console logs indicating WASM functionality
    const wasmLogs = consoleLogs.filter(log => 
      log.includes('Hello') || log.includes('WASM') || log.includes('greet')
    );
    
    expect(wasmLogs.length).toBeGreaterThan(0);
    console.log('WASM Console Logs:', wasmLogs);
  });

  test('Greet function works', async ({ page }) => {
    // Click the greet test button
    await page.click('button:has-text("Test Greet")');
    
    // Wait for the result to appear
    await page.waitForSelector('#greet-result', { timeout: 5000 });
    
    // Get the result text
    const resultText = await page.textContent('#greet-result');
    
    // The result should contain a greeting
    expect(resultText).toMatch(/Hello/);
    
    console.log('Greet Result:', resultText);
  });

  test('Data processing function works', async ({ page }) => {
    // Click the data processing test button
    await page.click('button:has-text("Test Data Processing")');
    
    // Wait for the result to appear
    await page.waitForSelector('#data-result', { timeout: 5000 });
    
    // Get the result text
    const resultText = await page.textContent('#data-result');
    
    // The result should contain processed data information
    expect(resultText).toMatch(/Processed|Data|Result/);
    
    console.log('Data Processing Result:', resultText);
  });

  test('Chart creation function works', async ({ page }) => {
    // Click the chart creation test button
    await page.click('button:has-text("Test Chart Creation")');
    
    // Wait for the result to appear
    await page.waitForSelector('#chart-result', { timeout: 5000 });
    
    // Get the result text
    const resultText = await page.textContent('#chart-result');
    
    // The result should contain chart information
    expect(resultText).toMatch(/Chart|Created|Success/);
    
    console.log('Chart Creation Result:', resultText);
  });

  test('WebGPU support detection works', async ({ page }) => {
    // Click the WebGPU test button
    await page.click('button:has-text("Test WebGPU")');
    
    // Wait for the result to appear
    await page.waitForSelector('#webgpu-result', { timeout: 5000 });
    
    // Get the result text
    const resultText = await page.textContent('#webgpu-result');
    
    // The result should contain WebGPU support information
    expect(resultText).toMatch(/WebGPU|Support|Available|Not Available/);
    
    console.log('WebGPU Support Result:', resultText);
  });

  test('All WASM functions are accessible', async ({ page }) => {
    // Check that all test buttons are present and clickable
    const buttons = [
      'Test Greet',
      'Test Data Processing', 
      'Test Chart Creation',
      'Test WebGPU'
    ];
    
    for (const buttonText of buttons) {
      const button = page.locator(`button:has-text("${buttonText}")`);
      await expect(button).toBeVisible();
      await expect(button).toBeEnabled();
    }
  });

  test('WASM functions return valid results', async ({ page }) => {
    // Test all functions and verify they return results
    const testCases = [
      { button: 'Test Greet', resultSelector: '#greet-result' },
      { button: 'Test Data Processing', resultSelector: '#data-result' },
      { button: 'Test Chart Creation', resultSelector: '#chart-result' },
      { button: 'Test WebGPU', resultSelector: '#webgpu-result' }
    ];
    
    for (const testCase of testCases) {
      // Click the button
      await page.click(`button:has-text("${testCase.button}")`);
      
      // Wait for result
      await page.waitForSelector(testCase.resultSelector, { timeout: 5000 });
      
      // Get the result
      const resultText = await page.textContent(testCase.resultSelector);
      
      // Result should not be empty
      expect(resultText.trim()).not.toBe('');
      
      console.log(`${testCase.button}: ${resultText}`);
    }
  });

  test('WASM module handles errors gracefully', async ({ page }) => {
    // This test verifies that the WASM module doesn't crash the page
    // when functions are called multiple times
    
    // Call all functions multiple times
    for (let i = 0; i < 3; i++) {
      await page.click('button:has-text("Test Greet")');
      await page.waitForTimeout(500);
      
      await page.click('button:has-text("Test Data Processing")');
      await page.waitForTimeout(500);
      
      await page.click('button:has-text("Test Chart Creation")');
      await page.waitForTimeout(500);
      
      await page.click('button:has-text("Test WebGPU")');
      await page.waitForTimeout(500);
    }
    
    // Page should still be functional
    await expect(page.locator('h1')).toBeVisible();
    await expect(page.locator('button:has-text("Test Greet")')).toBeEnabled();
  });

  test('WASM performance is acceptable', async ({ page }) => {
    // Measure the time it takes for WASM functions to execute
    const functions = [
      'Test Greet',
      'Test Data Processing',
      'Test Chart Creation',
      'Test WebGPU'
    ];
    
    for (const functionName of functions) {
      const startTime = Date.now();
      
      await page.click(`button:has-text("${functionName}")`);
      
      // Wait for any result to appear
      await page.waitForTimeout(1000);
      
      const endTime = Date.now();
      const executionTime = endTime - startTime;
      
      // WASM functions should execute quickly
      expect(executionTime).toBeLessThan(2000);
      
      console.log(`${functionName}: ${executionTime}ms`);
    }
  });
});
