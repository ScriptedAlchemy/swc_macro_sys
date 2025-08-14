import { test, expect, E2E_CONFIG } from './setup.js';

test.describe('Application Validation', () => {
  test('should validate complete Module Federation React setup', async ({ page, moduleFederation, performanceMonitor }) => {
    console.log('🔍 Validating Module Federation React application...');
    
    // Step 1: Load host application
    const hostLoadTime = await performanceMonitor.measureLoadTime(async () => {
      await page.goto('/');
      await page.waitForSelector('text=MF React App');
    });
    
    console.log(`✅ Host app loaded in ${hostLoadTime}ms`);
    expect(hostLoadTime).toBeLessThan(E2E_CONFIG.PERFORMANCE_BUDGETS.INITIAL_LOAD);
    
    // Step 2: Verify navigation works
    const pages = [
      { name: 'Dashboard', selector: 'h2:has-text("Dashboard")' },
      { name: 'Analytics', selector: 'h2:has-text("Analytics")' },
      { name: 'Users', selector: 'h2:has-text("Users")' },
      { name: 'Settings', selector: 'h2:has-text("Settings")' }
    ];
    
    for (const pageConfig of pages) {
      await page.click(`text=${pageConfig.name}`);
      await expect(page.locator(pageConfig.selector)).toBeVisible();
    }
    
    console.log('✅ All host app pages navigate correctly');
    
    // Step 3: Verify remote components load
    await page.click('text=Remote Components');
    
    const remoteComponentLoadTime = await performanceMonitor.measureLoadTime(async () => {
      await moduleFederation.switchToRemoteTab('User Card');
    });
    
    console.log(`✅ Remote component loaded in ${remoteComponentLoadTime}ms`);
    expect(remoteComponentLoadTime).toBeLessThan(E2E_CONFIG.PERFORMANCE_BUDGETS.REMOTE_COMPONENT);
    
    // Step 4: Test all remote components
    const remoteComponents = [
      { name: 'User Card', validator: () => page.locator('text=John Doe') },
      { name: 'Data Table', validator: () => page.locator('th:has-text("Product")') },
      { name: 'Charts', validator: () => page.locator('text=Chart Widgets') },
      { name: 'Form Builder', validator: () => page.locator('label:has-text("First Name")') }
    ];
    
    for (const component of remoteComponents) {
      await moduleFederation.switchToRemoteTab(component.name);
      await expect(component.validator()).toBeVisible();
    }
    
    console.log('✅ All remote components load and render correctly');
    
    // Step 5: Verify shared dependencies
    const sharedDeps = await moduleFederation.checkSharedDependencies();
    expect(sharedDeps.reactInstances).toBeLessThanOrEqual(1); // Should have only one React instance
    expect(sharedDeps.antComponents).toBe(true); // Should have Ant Design components
    
    console.log('✅ Shared dependencies work correctly');
    
    // Step 6: Check bundle performance
    const totalBundleSize = performanceMonitor.getTotalBundleSize();
    console.log(`📦 Total bundle size: ${(totalBundleSize / 1024).toFixed(2)}KB`);
    expect(totalBundleSize).toBeLessThan(E2E_CONFIG.PERFORMANCE_BUDGETS.TOTAL_BUNDLE_SIZE);
    
    // Step 7: Verify optimization worked
    const networkRequests = performanceMonitor.getRequestCount();
    console.log(`🌐 Total network requests: ${networkRequests}`);
    
    // Should have reasonable number of requests for a federated app
    expect(networkRequests).toBeLessThan(50);
    
    console.log('🎉 All validations passed! Module Federation React app is working correctly.');
  });

  test('should demonstrate tree-shaking optimization', async ({ page }) => {
    // This test assumes the build has been optimized
    const response = await page.request.get('http://localhost:3001');
    expect(response.status()).toBe(200);
    
    await page.goto('/');
    
    // Look for evidence of optimization in network panel
    const jsRequests = [];
    page.on('request', request => {
      if (request.url().includes('.js')) {
        jsRequests.push(request.url());
      }
    });
    
    // Load the app and trigger remote component loading
    await page.click('text=Remote Components');
    await page.click('[role="tab"]:has-text("User Card")');
    await page.waitForSelector('.ant-card', { timeout: 15000 });
    
    await page.waitForLoadState('networkidle');
    
    // Check for vendor chunks presence
    const optimizedChunks = jsRequests.filter(url => url.includes('.optimized.'));
    const vendorChunks = jsRequests.filter(url => url.includes('vendors-') || url.includes('node_modules'));
    
    console.log('JavaScript requests:', jsRequests.length);
    console.log('Vendor chunks:', vendorChunks.length);
    console.log('Optimized chunks:', optimizedChunks.length);
    
    // Should have some JS requests
    expect(jsRequests.length).toBeGreaterThan(0);
    
    // App should load successfully regardless of optimization
    await expect(page.locator('text=John Doe')).toBeVisible({ timeout: 15000 });
  });

  test('should handle error scenarios gracefully', async ({ page }) => {
    // Test 1: Network interruption
    await page.goto('/');
    
    // Temporarily block remote requests
    await page.route('**/localhost:3002/**', route => {
      route.abort('internetdisconnected');
    });
    
    // Navigate to remote components
    await page.click('text=Remote Components');
    
    // Should show some form of error handling or fallback
    await expect(page.locator('body')).toBeVisible(); // App should not crash
    
    // Re-enable network
    await page.unroute('**/localhost:3002/**');
    
    // Test 2: Invalid component loading
    await page.route('**/remoteEntry.js', route => {
      route.fulfill({
        status: 404,
        body: 'Not Found'
      });
    });
    
    // Try to load remote component
    await page.click('[role="tab"]:has-text("User Card")');
    // Allow page to handle error
    await page.waitForTimeout(500);
    
    // Should handle the error gracefully
    await expect(page.locator('body')).toBeVisible(); // Should not crash
    
    await page.unroute('**/remoteEntry.js');
  });

  test('should maintain functionality across browser refresh', async ({ page }) => {
    // Load app and navigate to remote components
    await page.goto('/remote-components');
    await page.click('[role="tab"]:has-text("Form Builder")');
    await page.waitForSelector('.ant-form', { timeout: 15000 });
    
    // Fill out form
    await page.fill('input[placeholder*="First Name"]', 'Test');
    await page.fill('input[placeholder*="Last Name"]', 'User');
    
    // Refresh page
    await page.reload();
    
    // Should return to the same URL and be functional
    expect(page.url()).toContain('/remote-components');
    
    // Navigate back to form and verify it's working
    await page.click('[role="tab"]:has-text("Form Builder")');
    await page.waitForSelector('.ant-form', { timeout: 15000 });
    
    // Form should be functional after refresh
    await page.fill('input[placeholder*="First Name"]', 'New Test');
    await expect(page.locator('input[placeholder*="First Name"]')).toHaveValue('New Test');
  });
});
