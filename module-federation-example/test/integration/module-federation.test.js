import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';

describe('Module Federation Integration', () => {
  const hostDistPath = path.resolve(__dirname, '../../host/dist');
  const remoteDistPath = path.resolve(__dirname, '../../remote/dist');
  
  beforeAll(() => {
    // Build apps if needed
    if (!fs.existsSync(hostDistPath) || !fs.existsSync(remoteDistPath)) {
      console.log('Building Module Federation apps...');
      execSync('pnpm run build', { 
        cwd: path.resolve(__dirname, '../..'),
        stdio: 'inherit' 
      });
    }
  });
  
  describe('Build Output', () => {
    it('should generate host bundle files', () => {
      expect(fs.existsSync(path.join(hostDistPath, 'main.js'))).toBe(true);
      expect(fs.existsSync(path.join(hostDistPath, 'share-usage.json'))).toBe(true);
    });
    
    it('should generate remote bundle files', () => {
      expect(fs.existsSync(path.join(remoteDistPath, 'main.js'))).toBe(true);
      expect(fs.existsSync(path.join(remoteDistPath, 'remoteEntry.js'))).toBe(true);
      expect(fs.existsSync(path.join(remoteDistPath, 'share-usage.json'))).toBe(true);
    });
    
    it('should generate lodash vendor chunks', () => {
      const hostLodashChunk = fs.readdirSync(hostDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js'));
      const remoteLodashChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js'));
        
      expect(hostLodashChunk).toBeDefined();
      expect(remoteLodashChunk).toBeDefined();
    });
  });
  
  describe('Share Usage Analysis', () => {
    it('should track host app lodash usage', () => {
      const shareUsage = JSON.parse(
        fs.readFileSync(path.join(hostDistPath, 'share-usage.json'), 'utf-8')
      );
      
      expect(shareUsage.consume_shared_modules).toBeDefined();
      expect(shareUsage.consume_shared_modules['lodash-es']).toBeDefined();
      
      const lodashUsage = shareUsage.consume_shared_modules['lodash-es'];
      expect(lodashUsage.used_exports).toContain('sortBy');
      expect(lodashUsage.used_exports).toContain('uniq');
      expect(lodashUsage.entry_module_id).toContain('lodash-es/lodash.js');
    });
    
    it('should track remote app lodash usage', () => {
      const shareUsage = JSON.parse(
        fs.readFileSync(path.join(remoteDistPath, 'share-usage.json'), 'utf-8')
      );
      
      const lodashUsage = shareUsage.consume_shared_modules['lodash-es'];
      expect(lodashUsage.used_exports).toContain('capitalize');
      expect(lodashUsage.used_exports).toContain('debounce');
      expect(lodashUsage.entry_module_id).toContain('lodash-es/lodash.js');
    });
  });
  
  describe('Optimization', () => {
    let optimizationResult;
    
    beforeAll(() => {
      // Run optimization
      console.log('Running optimization...');
      execSync('pnpm run optimize', { 
        cwd: path.resolve(__dirname, '../..'),
        stdio: 'pipe' 
      });
    });
    
    it('should create optimized lodash chunks', () => {
      const hostLodashChunk = fs.readdirSync(hostDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original'));
      const hostOriginalChunk = hostLodashChunk + '.original';
      
      expect(fs.existsSync(path.join(hostDistPath, hostLodashChunk))).toBe(true);
      expect(fs.existsSync(path.join(hostDistPath, hostOriginalChunk))).toBe(true);
    });
    
    it('should achieve significant size reduction', () => {
      const hostLodashChunk = fs.readdirSync(hostDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original'));
      
      const optimizedSize = fs.statSync(path.join(hostDistPath, hostLodashChunk)).size;
      const originalSize = fs.statSync(path.join(hostDistPath, hostLodashChunk + '.original')).size;
      
      const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
      
      expect(reduction).toBeGreaterThan(30); // At least 30% reduction
      console.log(`Size reduction: ${reduction.toFixed(2)}%`);
    });
    
    it('should preserve used exports in optimized chunks', () => {
      const hostLodashChunk = fs.readdirSync(hostDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original'));
      
      const optimizedContent = fs.readFileSync(
        path.join(hostDistPath, hostLodashChunk), 
        'utf-8'
      );
      
      // Host uses sortBy and uniq
      expect(optimizedContent).toContain('sortBy');
      expect(optimizedContent).toContain('uniq');
      
      // Should have removed many unused functions
      const unusedFunctions = ['groupBy', 'throttle', 'debounce', 'omit', 'pick'];
      let removedCount = 0;
      
      unusedFunctions.forEach(func => {
        if (!optimizedContent.includes(`function ${func}`) && 
            !optimizedContent.includes(`${func}:function`)) {
          removedCount++;
        }
      });
      
      expect(removedCount).toBeGreaterThan(0);
    });
  });
});