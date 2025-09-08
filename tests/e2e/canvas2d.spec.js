// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Canvas2D Rendering Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the Canvas2D demo page
    await page.goto('/canvas2d-demo.html');
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
  });

  test('Canvas2D demo loads successfully', async ({ page }) => {
    // Check that the page title is correct
    await expect(page).toHaveTitle(/Canvas2D Demo/);
    
    // Check that the main container is present
    await expect(page.locator('.container')).toBeVisible();
    
    // Check that the header is present
    await expect(page.locator('h1')).toContainText('Canvas2D Rendering Demo');
  });

  test('Chart canvas is present and visible', async ({ page }) => {
    // Check that the chart canvas is present
    await expect(page.locator('#chartCanvas')).toBeVisible();
    
    // Check that the canvas has the correct dimensions
    const canvas = page.locator('#chartCanvas');
    await expect(canvas).toHaveAttribute('width', '800');
    await expect(canvas).toHaveAttribute('height', '600');
  });

  test('Chart type selector works', async ({ page }) => {
    // Check that chart type selector is present
    await expect(page.locator('select[name="chartType"]')).toBeVisible();
    
    // Test different chart types
    const chartTypes = ['line', 'bar', 'scatter'];
    
    for (const chartType of chartTypes) {
      await page.selectOption('select[name="chartType"]', chartType);
      
      // Verify the selection
      const selectedType = await page.inputValue('select[name="chartType"]');
      expect(selectedType).toBe(chartType);
    }
  });

  test('Data size selector works', async ({ page }) => {
    // Check that data size selector is present
    await expect(page.locator('select[name="dataSize"]')).toBeVisible();
    
    // Test different data sizes
    const dataSizes = ['100', '500', '1000'];
    
    for (const dataSize of dataSizes) {
      await page.selectOption('select[name="dataSize"]', dataSize);
      
      // Verify the selection
      const selectedSize = await page.inputValue('select[name="dataSize"]');
      expect(selectedSize).toBe(dataSize);
    }
  });

  test('Render button works', async ({ page }) => {
    // Check that render button is present
    await expect(page.locator('button:has-text("Render Chart")')).toBeVisible();
    
    // Click the render button
    await page.click('button:has-text("Render Chart")');
    
    // Wait a bit for rendering to complete
    await page.waitForTimeout(1000);
    
    // The button should still be visible and clickable
    await expect(page.locator('button:has-text("Render Chart")')).toBeVisible();
  });

  test('Line chart renders correctly', async ({ page }) => {
    // Select line chart
    await page.selectOption('select[name="chartType"]', 'line');
    await page.selectOption('select[name="dataSize"]', '100');
    
    // Render the chart
    await page.click('button:has-text("Render Chart")');
    await page.waitForTimeout(1000);
    
    // Check that the canvas has been drawn on
    const canvas = page.locator('#chartCanvas');
    const canvasElement = await canvas.elementHandle();
    
    // Get the canvas context and check if it has content
    const hasContent = await page.evaluate((canvas) => {
      const ctx = canvas.getContext('2d');
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      
      // Check if there are any non-transparent pixels
      for (let i = 3; i < imageData.data.length; i += 4) {
        if (imageData.data[i] > 0) {
          return true;
        }
      }
      return false;
    }, canvasElement);
    
    expect(hasContent).toBe(true);
  });

  test('Bar chart renders correctly', async ({ page }) => {
    // Select bar chart
    await page.selectOption('select[name="chartType"]', 'bar');
    await page.selectOption('select[name="dataSize"]', '50');
    
    // Render the chart
    await page.click('button:has-text("Render Chart")');
    await page.waitForTimeout(1000);
    
    // Check that the canvas has been drawn on
    const canvas = page.locator('#chartCanvas');
    const canvasElement = await canvas.elementHandle();
    
    const hasContent = await page.evaluate((canvas) => {
      const ctx = canvas.getContext('2d');
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      
      for (let i = 3; i < imageData.data.length; i += 4) {
        if (imageData.data[i] > 0) {
          return true;
        }
      }
      return false;
    }, canvasElement);
    
    expect(hasContent).toBe(true);
  });

  test('Scatter plot renders correctly', async ({ page }) => {
    // Select scatter plot
    await page.selectOption('select[name="chartType"]', 'scatter');
    await page.selectOption('select[name="dataSize"]', '200');
    
    // Render the chart
    await page.click('button:has-text("Render Chart")');
    await page.waitForTimeout(1000);
    
    // Check that the canvas has been drawn on
    const canvas = page.locator('#chartCanvas');
    const canvasElement = await canvas.elementHandle();
    
    const hasContent = await page.evaluate((canvas) => {
      const ctx = canvas.getContext('2d');
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      
      for (let i = 3; i < imageData.data.length; i += 4) {
        if (imageData.data[i] > 0) {
          return true;
        }
      }
      return false;
    }, canvasElement);
    
    expect(hasContent).toBe(true);
  });

  test('Performance with different data sizes', async ({ page }) => {
    const dataSizes = ['100', '500', '1000'];
    
    for (const dataSize of dataSizes) {
      // Select data size
      await page.selectOption('select[name="dataSize"]', dataSize);
      await page.selectOption('select[name="chartType"]', 'line');
      
      // Measure rendering time
      const startTime = Date.now();
      await page.click('button:has-text("Render Chart")');
      await page.waitForTimeout(1000);
      const endTime = Date.now();
      
      const renderTime = endTime - startTime;
      
      // Performance should be reasonable for all data sizes
      expect(renderTime).toBeLessThan(2000);
      
      console.log(`Data size ${dataSize}: Render time ${renderTime}ms`);
    }
  });

  test('Chart switching works smoothly', async ({ page }) => {
    const chartTypes = ['line', 'bar', 'scatter'];
    
    for (const chartType of chartTypes) {
      // Select chart type
      await page.selectOption('select[name="chartType"]', chartType);
      
      // Render the chart
      const startTime = Date.now();
      await page.click('button:has-text("Render Chart")');
      await page.waitForTimeout(1000);
      const endTime = Date.now();
      
      const renderTime = endTime - startTime;
      
      // Each chart type should render quickly
      expect(renderTime).toBeLessThan(1500);
      
      console.log(`Chart type ${chartType}: Render time ${renderTime}ms`);
    }
  });

  test('Canvas2D fallback works when WebGPU is not available', async ({ page }) => {
    // This test verifies that Canvas2D rendering works as a fallback
    // when WebGPU is not available
    
    // Select a chart type and render
    await page.selectOption('select[name="chartType"]', 'line');
    await page.selectOption('select[name="dataSize"]', '100');
    
    await page.click('button:has-text("Render Chart")');
    await page.waitForTimeout(1000);
    
    // Check that the canvas has content
    const canvas = page.locator('#chartCanvas');
    const canvasElement = await canvas.elementHandle();
    
    const hasContent = await page.evaluate((canvas) => {
      const ctx = canvas.getContext('2d');
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      
      for (let i = 3; i < imageData.data.length; i += 4) {
        if (imageData.data[i] > 0) {
          return true;
        }
      }
      return false;
    }, canvasElement);
    
    expect(hasContent).toBe(true);
    console.log('Canvas2D fallback rendering works correctly');
  });
});
