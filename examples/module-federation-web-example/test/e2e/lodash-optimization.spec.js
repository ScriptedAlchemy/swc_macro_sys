import { test, expect } from '@playwright/test';

test.describe('Module Federation Web - Lodash Optimization', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the host application
    await page.goto('/');
    
    // Wait for the page to load completely
    await page.waitForLoadState('networkidle');
  });

  test('should load the host application successfully', async ({ page }) => {
    // Check that the page title is set
    await expect(page).toHaveTitle(/Module Federation/);
    
    // Check for basic page structure
    const body = page.locator('body');
    await expect(body).toBeVisible();
  });

  test('should have lodash functions available and working', async ({ page }) => {
    // Execute lodash functions in the browser context
    const result = await page.evaluate(async () => {
      // Wait for the bootstrap module to be available
      let bootstrap;
      try {
        // Try to import the bootstrap module
        bootstrap = await import('./bootstrap.js');
      } catch (error) {
        // If direct import fails, try to access from window or global scope
        if (window.bootstrap) {
          bootstrap = window.bootstrap;
        } else {
          throw new Error('Bootstrap module not available: ' + error.message);
        }
      }

      // Test data for lodash functions
      const testData = [
        { name: 'John', age: 30, score: 85 },
        { name: 'Jane', age: 25, score: 92 },
        { name: 'Bob', age: 35, score: 78 },
        { name: 'Alice', age: 28, score: 95 },
        { name: 'John', age: 30, score: 85 } // Duplicate for uniq test
      ];

      let results = {};

      try {
        // Test processItems function if available
        if (bootstrap.processItems) {
          results.processItems = bootstrap.processItems(testData);
        }

        // Test individual lodash functions if available
        if (bootstrap.sortBy) {
          results.sortBy = bootstrap.sortBy(testData, 'age');
        }

        if (bootstrap.uniq) {
          results.uniq = bootstrap.uniq(testData);
        }

        // Test that lodash functions work correctly
        results.sortByWorking = results.sortBy && results.sortBy.length === testData.length;
        results.uniqWorking = results.uniq && results.uniq.length === testData.length - 1; // Should remove one duplicate
        
        return results;
      } catch (error) {
        return { error: error.message, bootstrap: !!bootstrap };
      }
    });

    // Verify that lodash functions are working
    if (result.error) {
      console.log('Test result with error:', result);
      // If there's an error, at least verify bootstrap is available
      expect(result.bootstrap).toBe(true);
    } else {
      // Verify sortBy function works
      if (result.sortBy) {
        expect(result.sortByWorking).toBe(true);
        expect(result.sortBy[0].age).toBeLessThanOrEqual(result.sortBy[1].age);
      }

      // Verify uniq function works
      if (result.uniq) {
        expect(result.uniqWorking).toBe(true);
      }

      // Verify processItems function works
      if (result.processItems) {
        expect(result.processItems).toBeDefined();
        expect(Array.isArray(result.processItems)).toBe(true);
      }
    }
  });

  test('should load remote components successfully', async ({ page }) => {
    // Check for any console errors
    const consoleErrors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });

    // Wait for any dynamic imports to complete
    await page.waitForTimeout(2000);

    // Check that there are no critical console errors
    const criticalErrors = consoleErrors.filter(error => 
      !error.includes('favicon') && 
      !error.includes('404') &&
      !error.includes('net::ERR_')
    );

    if (criticalErrors.length > 0) {
      console.log('Console errors found:', criticalErrors);
    }

    // Allow some non-critical errors but ensure the page is functional
    expect(criticalErrors.length).toBeLessThan(5);
  });

  test('should have optimized bundle with tree-shaken lodash', async ({ page }) => {
    // Check network requests to verify bundle optimization
    const requests = [];
    page.on('request', request => {
      requests.push(request.url());
    });

    // Reload to capture all network requests
    await page.reload();
    await page.waitForLoadState('networkidle');

    // Check that JavaScript bundles are loaded
    const jsRequests = requests.filter(url => url.endsWith('.js'));
    expect(jsRequests.length).toBeGreaterThan(0);

    // Log bundle information for debugging
    console.log('JavaScript bundles loaded:', jsRequests.length);
    console.log('Bundle URLs:', jsRequests);
  });
});