// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('leptos-helios Comprehensive Showcase', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/comprehensive-showcase.html');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Page Structure and Loading', () => {
    test('should load the comprehensive showcase page', async ({ page }) => {
      await expect(page).toHaveTitle(/leptos-helios Comprehensive Showcase/);
      await expect(page.locator('h1')).toContainText('🚀 leptos-helios');
      await expect(page.locator('.header p')).toContainText('Comprehensive Showcase of All Features');
    });

    test('should display all feature cards', async ({ page }) => {
      const featureCards = page.locator('.feature-card');
      await expect(featureCards).toHaveCount(6);

      // Check all phase titles are present
      await expect(page.locator('text=Phase 1: Core Interactivity')).toBeVisible();
      await expect(page.locator('text=Phase 2: Advanced Chart Types')).toBeVisible();
      await expect(page.locator('text=Phase 3: Performance Optimizations')).toBeVisible();
      await expect(page.locator('text=Phase 4: Advanced Graph Features')).toBeVisible();
      await expect(page.locator('text=Phase 5: Smooth Animations')).toBeVisible();
      await expect(page.locator('text=Integration Demo')).toBeVisible();
    });

    test('should have responsive design', async ({ page }) => {
      // Test desktop view
      await page.setViewportSize({ width: 1200, height: 800 });
      await expect(page.locator('.feature-grid')).toBeVisible();

      // Test mobile view
      await page.setViewportSize({ width: 375, height: 667 });
      await expect(page.locator('.feature-grid')).toBeVisible();
      await expect(page.locator('h1')).toHaveCSS('font-size', '32px'); // 2rem on mobile
    });
  });

  test.describe('Phase 1: Core Interactivity', () => {
    test('should initialize interactivity demo', async ({ page }) => {
      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toBeVisible();
      await expect(canvas).toHaveAttribute('width', '350');
      await expect(canvas).toHaveAttribute('height', '250');
    });

    test('should demonstrate zoom functionality', async ({ page }) => {
      await page.click('button:has-text("Zoom Demo")');

      // Check that interaction count increases
      await expect(page.locator('#interaction-count')).not.toHaveText('0');

      // Check viewport info updates
      await expect(page.locator('#viewport-info')).toContainText('zoom: 2.0');

      // Check canvas transformation
      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toHaveCSS('transform', /scale\(1\.2\)/);
    });

    test('should demonstrate pan functionality', async ({ page }) => {
      await page.click('button:has-text("Pan Demo")');

      // Check interaction count increases
      await expect(page.locator('#interaction-count')).not.toHaveText('0');

      // Check viewport info updates
      await expect(page.locator('#viewport-info')).toContainText('x: 50, y: 30');

      // Check canvas transformation
      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toHaveCSS('transform', /translate\(10px, 5px\)/);
    });

    test('should demonstrate tooltips', async ({ page }) => {
      await page.click('button:has-text("Tooltips")');

      // Check interaction count increases
      await expect(page.locator('#interaction-count')).not.toHaveText('0');

      // Check tooltip appears
      await expect(page.locator('text=Sample tooltip with rich information')).toBeVisible();
    });

    test('should demonstrate brush selection', async ({ page }) => {
      await page.click('button:has-text("Brush Selection")');

      // Check interaction count increases
      await expect(page.locator('#interaction-count')).not.toHaveText('0');

      // Check brush selection overlay appears
      const brushOverlay = page.locator('div[style*="border: 2px dashed #667eea"]');
      await expect(brushOverlay).toBeVisible();
    });
  });

  test.describe('Phase 2: Advanced Chart Types', () => {
    test('should initialize chart types demo', async ({ page }) => {
      const svg = page.locator('#chart-svg');
      await expect(svg).toBeVisible();
      await expect(svg).toHaveAttribute('width', '350');
      await expect(svg).toHaveAttribute('height', '250');
    });

    test('should display heatmap', async ({ page }) => {
      await page.click('button:has-text("Heatmap")');

      // Check chart type updates
      await expect(page.locator('#current-chart')).toHaveText('Heatmap');
      await expect(page.locator('#chart-elements')).toHaveText('100');

      // Check SVG has heatmap rectangles
      const svg = page.locator('#chart-svg');
      const rectangles = svg.locator('rect');
      await expect(rectangles).toHaveCount(100);
    });

    test('should display treemap', async ({ page }) => {
      await page.click('button:has-text("Treemap")');

      // Check chart type updates
      await expect(page.locator('#current-chart')).toHaveText('Treemap');
      await expect(page.locator('#chart-elements')).toHaveText('8');

      // Check SVG has treemap rectangles
      const svg = page.locator('#chart-svg');
      const rectangles = svg.locator('rect');
      await expect(rectangles).toHaveCount(8);
    });

    test('should display Sankey diagram', async ({ page }) => {
      await page.click('button:has-text("Sankey")');

      // Check chart type updates
      await expect(page.locator('#current-chart')).toHaveText('Sankey');
      await expect(page.locator('#chart-elements')).toHaveText('12');

      // Check SVG has Sankey elements
      const svg = page.locator('#chart-svg');
      const rectangles = svg.locator('rect');
      const paths = svg.locator('path');
      await expect(rectangles).toHaveCount(5); // 5 nodes
      await expect(paths).toHaveCount(4); // 4 flow paths
    });

    test('should clear chart', async ({ page }) => {
      // First show a chart
      await page.click('button:has-text("Heatmap")');
      await expect(page.locator('#current-chart')).toHaveText('Heatmap');

      // Then clear it
      await page.click('button:has-text("Clear")');
      await expect(page.locator('#current-chart')).toHaveText('None');
      await expect(page.locator('#chart-elements')).toHaveText('0');

      // Check SVG is empty
      const svg = page.locator('#chart-svg');
      await expect(svg.locator('*')).toHaveCount(0);
    });
  });

  test.describe('Phase 3: Performance Optimizations', () => {
    test('should initialize performance demo', async ({ page }) => {
      const canvas = page.locator('#performance-canvas');
      await expect(canvas).toBeVisible();
      await expect(canvas).toHaveAttribute('width', '350');
      await expect(canvas).toHaveAttribute('height', '200');
    });

    test('should demonstrate virtual scrolling', async ({ page }) => {
      await page.click('button:has-text("Virtual Scroll")');

      // Check performance metrics update
      await expect(page.locator('#fps-counter')).toHaveText('60');
      await expect(page.locator('#memory-usage')).toContainText('MB');
      await expect(page.locator('#render-time')).toContainText('ms');
    });

    test('should demonstrate data sampling', async ({ page }) => {
      await page.click('button:has-text("Data Sampling")');

      // Check performance metrics
      await expect(page.locator('#fps-counter')).toHaveText('60');
      await expect(page.locator('#memory-usage')).toHaveText('32 MB');
      await expect(page.locator('#render-time')).toHaveText('5ms');
    });

    test('should demonstrate WebGL rendering', async ({ page }) => {
      await page.click('button:has-text("WebGL Render")');

      // Check performance metrics
      await expect(page.locator('#fps-counter')).toHaveText('60');
      await expect(page.locator('#memory-usage')).toHaveText('28 MB');
      await expect(page.locator('#render-time')).toHaveText('3ms');
    });

    test('should reset performance demo', async ({ page }) => {
      // First run a demo
      await page.click('button:has-text("Virtual Scroll")');
      await expect(page.locator('#memory-usage')).toHaveText('45 MB');

      // Then reset
      await page.click('button:has-text("Reset")');
      await expect(page.locator('#memory-usage')).toHaveText('25 MB');
      await expect(page.locator('#render-time')).toHaveText('2ms');
    });
  });

  test.describe('Phase 4: Advanced Graph Features', () => {
    test('should initialize graph demo', async ({ page }) => {
      const container = page.locator('#graph-demo');
      await expect(container).toBeVisible();

      // Check initial nodes are created
      await expect(page.locator('#node-count')).toHaveText('5');
      await expect(page.locator('#edge-count')).toHaveText('4');

      const nodes = container.locator('.node');
      await expect(nodes).toHaveCount(5);
    });

    test('should demonstrate force layout', async ({ page }) => {
      await page.click('button:has-text("Force Layout")');

      // Check metrics update
      await expect(page.locator('#node-count')).toHaveText('8');
      await expect(page.locator('#edge-count')).toHaveText('12');

      // Check nodes have transition animations
      const nodes = page.locator('#graph-demo .node');
      await expect(nodes.first()).toHaveCSS('transition', /all 1s ease/);
    });

    test('should demonstrate clustering', async ({ page }) => {
      await page.click('button:has-text("Clustering")');

      // Check cluster count updates
      await expect(page.locator('#cluster-count')).toHaveText('3');

      // Check nodes change color and position
      const nodes = page.locator('#graph-demo .node');
      await expect(nodes.first()).toHaveCSS('transition', /all 0\.8s ease/);
    });

    test('should demonstrate network analysis', async ({ page }) => {
      await page.click('button:has-text("Analysis")');

      // Check all metrics update
      await expect(page.locator('#node-count')).toHaveText('12');
      await expect(page.locator('#edge-count')).toHaveText('18');
      await expect(page.locator('#cluster-count')).toHaveText('4');

      // Check additional nodes are added
      const nodes = page.locator('#graph-demo .node');
      await expect(nodes).toHaveCount(9); // 5 original + 4 new
    });

    test('should clear graph', async ({ page }) => {
      // First add some content
      await page.click('button:has-text("Force Layout")');
      await expect(page.locator('#node-count')).not.toHaveText('0');

      // Then clear
      await page.click('button:has-text("Clear")');
      await expect(page.locator('#node-count')).toHaveText('0');
      await expect(page.locator('#edge-count')).toHaveText('0');
      await expect(page.locator('#cluster-count')).toHaveText('0');

      // Check container is empty
      const container = page.locator('#graph-demo');
      await expect(container.locator('.node')).toHaveCount(0);
    });
  });

  test.describe('Phase 5: Smooth Animations', () => {
    test('should initialize animation demo', async ({ page }) => {
      const container = page.locator('#animation-demo');
      await expect(container).toBeVisible();

      const element = page.locator('#animated-element');
      await expect(element).toBeVisible();
      await expect(element).toHaveCSS('background-color', 'rgb(102, 126, 234)'); // #667eea
    });

    test('should demonstrate easing animations', async ({ page }) => {
      await page.click('button:has-text("Easing Demo")');

      // Check animation type updates
      await expect(page.locator('#animation-type')).toHaveText('Easing Demo');

      // Check element moves
      const element = page.locator('#animated-element');
      await expect(element).toHaveCSS('transition', /all 2s cubic-bezier/);

      // Check progress bar animates
      const progressFill = page.locator('#progress-fill');
      await expect(progressFill).toHaveCSS('width', /[1-9]/); // Should be > 0%
    });

    test('should demonstrate tween animations', async ({ page }) => {
      await page.click('button:has-text("Tween Demo")');

      // Check animation type updates
      await expect(page.locator('#animation-type')).toHaveText('Tween Demo');

      // Check element transforms
      const element = page.locator('#animated-element');
      await expect(element).toHaveCSS('transition', /all 1\.5s ease-in-out/);
      await expect(element).toHaveCSS('transform', /scale\(1\.5\) rotate\(180deg\)/);
      await expect(element).toHaveCSS('background-color', 'rgb(240, 147, 251)'); // #f093fb
    });

    test('should demonstrate state transitions', async ({ page }) => {
      await page.click('button:has-text("Transitions")');

      // Check animation type updates
      await expect(page.locator('#animation-type')).toHaveText('State Transitions');

      // Check element has transition
      const element = page.locator('#animated-element');
      await expect(element).toHaveCSS('transition', /all 0\.5s ease/);
    });

    test('should stop animations', async ({ page }) => {
      // First start an animation
      await page.click('button:has-text("Easing Demo")');
      await expect(page.locator('#animation-type')).toHaveText('Easing Demo');

      // Then stop it
      await page.click('button:has-text("Stop")');
      await expect(page.locator('#animation-type')).toHaveText('Stopped');
      await expect(page.locator('#animation-progress')).toHaveText('0%');

      // Check element resets
      const element = page.locator('#animated-element');
      await expect(element).toHaveCSS('transition', 'none');
      await expect(element).toHaveCSS('background-color', 'rgb(102, 126, 234)'); // #667eea
    });
  });

  test.describe('Integration Demo', () => {
    test('should initialize integration demo', async ({ page }) => {
      const canvas = page.locator('#integration-canvas');
      await expect(canvas).toBeVisible();
      await expect(canvas).toHaveAttribute('width', '350');
      await expect(canvas).toHaveAttribute('height', '250');

      // Check initial status
      await expect(page.locator('#integration-status')).toHaveText('Ready');
      await expect(page.locator('#active-features')).toHaveText('0');
      await expect(page.locator('#integration-performance')).toHaveText('Excellent');
    });

    test('should run full integration demo', async ({ page }) => {
      await page.click('button:has-text("Full Demo")');

      // Check status updates
      await expect(page.locator('#integration-status')).toHaveText('Running Full Demo');
      await expect(page.locator('#active-features')).toHaveText('5');
      await expect(page.locator('#integration-performance')).toHaveText('Excellent');
    });

    test('should demonstrate cross-feature interactions', async ({ page }) => {
      await page.click('button:has-text("Cross Features")');

      // Check status updates
      await expect(page.locator('#integration-status')).toHaveText('Cross-Feature Demo');
      await expect(page.locator('#active-features')).toHaveText('3');
      await expect(page.locator('#integration-performance')).toHaveText('Good');
    });

    test('should demonstrate performance testing', async ({ page }) => {
      await page.click('button:has-text("Performance")');

      // Check status updates
      await expect(page.locator('#integration-status')).toHaveText('Performance Test');
      await expect(page.locator('#active-features')).toHaveText('4');
      await expect(page.locator('#integration-performance')).toHaveText('Excellent');
    });

    test('should reset integration demo', async ({ page }) => {
      // First run a demo
      await page.click('button:has-text("Full Demo")');
      await expect(page.locator('#integration-status')).toHaveText('Running Full Demo');

      // Then reset
      await page.click('button:has-text("Reset")');
      await expect(page.locator('#integration-status')).toHaveText('Ready');
      await expect(page.locator('#active-features')).toHaveText('0');
      await expect(page.locator('#integration-performance')).toHaveText('Excellent');
    });
  });

  test.describe('Performance Monitoring', () => {
    test('should display performance metrics', async ({ page }) => {
      // Check FPS counter is present and updating
      const fpsCounter = page.locator('#fps-counter');
      await expect(fpsCounter).toBeVisible();
      await expect(fpsCounter).toHaveText('60');

      // Check memory usage is displayed
      const memoryUsage = page.locator('#memory-usage');
      await expect(memoryUsage).toBeVisible();
      await expect(memoryUsage).toContainText('MB');

      // Check render time is displayed
      const renderTime = page.locator('#render-time');
      await expect(renderTime).toBeVisible();
      await expect(renderTime).toContainText('ms');
    });

    test('should update performance metrics over time', async ({ page }) => {
      // Wait for metrics to potentially update
      await page.waitForTimeout(2000);

      // Check that metrics are still visible and reasonable
      const fpsCounter = page.locator('#fps-counter');
      await expect(fpsCounter).toBeVisible();

      const memoryUsage = page.locator('#memory-usage');
      await expect(memoryUsage).toBeVisible();

      const renderTime = page.locator('#render-time');
      await expect(renderTime).toBeVisible();
    });
  });

  test.describe('Accessibility', () => {
    test('should have proper heading structure', async ({ page }) => {
      // Check main heading
      const h1 = page.locator('h1');
      await expect(h1).toBeVisible();

      // Check feature headings
      const h3s = page.locator('h3');
      await expect(h3s).toHaveCount(6);
    });

    test('should have accessible buttons', async ({ page }) => {
      const buttons = page.locator('button');
      const buttonCount = await buttons.count();

      for (let i = 0; i < buttonCount; i++) {
        const button = buttons.nth(i);
        await expect(button).toBeVisible();
        await expect(button).toHaveAttribute('type', 'button');
      }
    });

    test('should have proper color contrast', async ({ page }) => {
      // Check that text is visible against backgrounds
      const header = page.locator('.header h1');
      await expect(header).toHaveCSS('color', 'rgb(255, 255, 255)'); // White text

      const featureCards = page.locator('.feature-card h3');
      await expect(featureCards.first()).toHaveCSS('color', 'rgb(102, 126, 234)'); // #667eea
    });
  });

  test.describe('Cross-Browser Compatibility', () => {
    test('should work in different browsers', async ({ page }) => {
      // Basic functionality should work across browsers
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('.feature-grid')).toBeVisible();

      // Test basic interactions
      await page.click('button:has-text("Zoom Demo")');
      await expect(page.locator('#interaction-count')).not.toHaveText('0');
    });
  });

  test.describe('Mobile Responsiveness', () => {
    test('should adapt to mobile viewport', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Check that content is still visible and accessible
      await expect(page.locator('h1')).toBeVisible();
      await expect(page.locator('.feature-grid')).toBeVisible();

      // Check that buttons are still clickable
      await page.click('button:has-text("Zoom Demo")');
      await expect(page.locator('#interaction-count')).not.toHaveText('0');
    });

    test('should handle touch interactions', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Test touch interactions
      const button = page.locator('button:has-text("Pan Demo")');
      await button.tap();
      await expect(page.locator('#interaction-count')).not.toHaveText('0');
    });
  });
});
