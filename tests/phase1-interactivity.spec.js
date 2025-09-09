// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Phase 1: Core Interactivity E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/comprehensive-showcase.html');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Viewport Management', () => {
    test('should initialize viewport with correct default values', async ({ page }) => {
      await expect(page.locator('#viewport-info')).toHaveText('x: 0, y: 0, zoom: 1.0');
    });

    test('should update viewport info on zoom', async ({ page }) => {
      await page.click('button:has-text("Zoom Demo")');
      await expect(page.locator('#viewport-info')).toContainText('zoom: 2.0');
    });

    test('should update viewport info on pan', async ({ page }) => {
      await page.click('button:has-text("Pan Demo")');
      await expect(page.locator('#viewport-info')).toContainText('x: 50, y: 30');
    });

    test('should apply zoom transformation to canvas', async ({ page }) => {
      await page.click('button:has-text("Zoom Demo")');

      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toHaveCSS('transform', /scale\(1\.2\)/);

      // Wait for animation to complete
      await page.waitForTimeout(1100);
      await expect(canvas).toHaveCSS('transform', /scale\(1\.0\)/);
    });

    test('should apply pan transformation to canvas', async ({ page }) => {
      await page.click('button:has-text("Pan Demo")');

      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toHaveCSS('transform', /translate\(10px, 5px\)/);

      // Wait for animation to complete
      await page.waitForTimeout(1100);
      await expect(canvas).toHaveCSS('transform', /translate\(0, 0\)/);
    });
  });

  test.describe('Interaction Tracking', () => {
    test('should track interaction count', async ({ page }) => {
      const initialCount = await page.locator('#interaction-count').textContent();
      expect(initialCount).toBe('0');

      await page.click('button:has-text("Zoom Demo")');
      await expect(page.locator('#interaction-count')).toHaveText('1');

      await page.click('button:has-text("Pan Demo")');
      await expect(page.locator('#interaction-count')).toHaveText('2');
    });

    test('should increment interaction count for all demo buttons', async ({ page }) => {
      const buttons = [
        'button:has-text("Zoom Demo")',
        'button:has-text("Pan Demo")',
        'button:has-text("Tooltips")',
        'button:has-text("Brush Selection")'
      ];

      for (let i = 0; i < buttons.length; i++) {
        await page.click(buttons[i]);
        await expect(page.locator('#interaction-count')).toHaveText((i + 1).toString());
      }
    });
  });

  test.describe('Tooltip System', () => {
    test('should display tooltip on demand', async ({ page }) => {
      await page.click('button:has-text("Tooltips")');

      const tooltip = page.locator('text=Sample tooltip with rich information');
      await expect(tooltip).toBeVisible();

      // Check tooltip styling
      await expect(tooltip).toHaveCSS('position', 'absolute');
      await expect(tooltip).toHaveCSS('background-color', 'rgba(0, 0, 0, 0.8)');
      await expect(tooltip).toHaveCSS('color', 'rgb(255, 255, 255)');
    });

    test('should auto-hide tooltip after timeout', async ({ page }) => {
      await page.click('button:has-text("Tooltips")');

      const tooltip = page.locator('text=Sample tooltip with rich information');
      await expect(tooltip).toBeVisible();

      // Wait for tooltip to disappear
      await page.waitForTimeout(2500);
      await expect(tooltip).not.toBeVisible();
    });

    test('should position tooltip correctly', async ({ page }) => {
      await page.click('button:has-text("Tooltips")');

      const tooltip = page.locator('text=Sample tooltip with rich information');
      await expect(tooltip).toHaveCSS('position', 'absolute');
      await expect(tooltip).toHaveCSS('z-index', '1000');
      await expect(tooltip).toHaveCSS('pointer-events', 'none');
    });
  });

  test.describe('Brush Selection', () => {
    test('should create brush selection overlay', async ({ page }) => {
      await page.click('button:has-text("Brush Selection")');

      const brushOverlay = page.locator('div[style*="border: 2px dashed #667eea"]');
      await expect(brushOverlay).toBeVisible();

      // Check brush styling
      await expect(brushOverlay).toHaveCSS('border', '2px dashed rgb(102, 126, 234)');
      await expect(brushOverlay).toHaveCSS('background-color', 'rgba(102, 126, 234, 0.1)');
    });

    test('should position brush selection correctly', async ({ page }) => {
      await page.click('button:has-text("Brush Selection")');

      const brushOverlay = page.locator('div[style*="border: 2px dashed #667eea"]');
      await expect(brushOverlay).toHaveCSS('position', 'absolute');
      await expect(brushOverlay).toHaveCSS('width', '100px');
      await expect(brushOverlay).toHaveCSS('height', '80px');
    });

    test('should auto-remove brush selection', async ({ page }) => {
      await page.click('button:has-text("Brush Selection")');

      const brushOverlay = page.locator('div[style*="border: 2px dashed #667eea"]');
      await expect(brushOverlay).toBeVisible();

      // Wait for brush to disappear
      await page.waitForTimeout(2000);
      await expect(brushOverlay).not.toBeVisible();
    });
  });

  test.describe('Canvas Rendering', () => {
    test('should render sample chart on canvas', async ({ page }) => {
      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toBeVisible();

      // Check canvas dimensions
      await expect(canvas).toHaveAttribute('width', '350');
      await expect(canvas).toHaveAttribute('height', '250');
    });

    test('should maintain canvas content during interactions', async ({ page }) => {
      const canvas = page.locator('#interactivity-canvas');

      // Perform multiple interactions
      await page.click('button:has-text("Zoom Demo")');
      await page.waitForTimeout(500);
      await page.click('button:has-text("Pan Demo")');
      await page.waitForTimeout(500);

      // Canvas should still be visible and functional
      await expect(canvas).toBeVisible();
      await expect(canvas).toHaveAttribute('width', '350');
    });
  });

  test.describe('Performance', () => {
    test('should handle rapid interactions without errors', async ({ page }) => {
      // Rapidly click multiple buttons
      const buttons = [
        'button:has-text("Zoom Demo")',
        'button:has-text("Pan Demo")',
        'button:has-text("Tooltips")',
        'button:has-text("Brush Selection")'
      ];

      for (const button of buttons) {
        await page.click(button);
        await page.waitForTimeout(100); // Small delay between clicks
      }

      // Should not have any console errors
      const logs = [];
      page.on('console', msg => {
        if (msg.type() === 'error') {
          logs.push(msg.text());
        }
      });

      await page.waitForTimeout(1000);
      expect(logs).toHaveLength(0);
    });

    test('should maintain smooth animations', async ({ page }) => {
      await page.click('button:has-text("Zoom Demo")');

      const canvas = page.locator('#interactivity-canvas');

      // Check that transformation is smooth
      await expect(canvas).toHaveCSS('transform', /scale\(1\.2\)/);

      // Wait for animation to complete
      await page.waitForTimeout(1100);
      await expect(canvas).toHaveCSS('transform', /scale\(1\.0\)/);
    });
  });

  test.describe('Accessibility', () => {
    test('should have accessible button labels', async ({ page }) => {
      const buttons = [
        'button:has-text("Zoom Demo")',
        'button:has-text("Pan Demo")',
        'button:has-text("Tooltips")',
        'button:has-text("Brush Selection")'
      ];

      for (const button of buttons) {
        const element = page.locator(button);
        await expect(element).toBeVisible();
        await expect(element).toHaveAttribute('type', 'button');
      }
    });

    test('should support keyboard navigation', async ({ page }) => {
      // Tab to first button
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');
      await page.keyboard.press('Tab');

      // Press Enter to activate
      await page.keyboard.press('Enter');

      // Should trigger interaction
      await expect(page.locator('#interaction-count')).not.toHaveText('0');
    });
  });

  test.describe('Cross-Browser Compatibility', () => {
    test('should work consistently across browsers', async ({ page }) => {
      // Test basic functionality
      await page.click('button:has-text("Zoom Demo")');
      await expect(page.locator('#interaction-count')).toHaveText('1');
      await expect(page.locator('#viewport-info')).toContainText('zoom: 2.0');

      // Test canvas transformations
      const canvas = page.locator('#interactivity-canvas');
      await expect(canvas).toHaveCSS('transform', /scale\(1\.2\)/);
    });
  });

  test.describe('Mobile Touch Support', () => {
    test('should handle touch interactions on mobile', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Test touch interactions
      const button = page.locator('button:has-text("Zoom Demo")');
      await button.tap();

      await expect(page.locator('#interaction-count')).toHaveText('1');
      await expect(page.locator('#viewport-info')).toContainText('zoom: 2.0');
    });

    test('should maintain touch responsiveness', async ({ page }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Test multiple touch interactions
      await page.tap('button:has-text("Pan Demo")');
      await page.tap('button:has-text("Tooltips")');

      await expect(page.locator('#interaction-count')).toHaveText('2');
    });
  });
});
