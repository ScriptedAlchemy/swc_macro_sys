import { describe, it, expect, beforeAll } from 'vitest';
import fs from 'fs';
import path from 'path';
import { performance } from 'perf_hooks';
import { optimizeChunk, analyzeChunk, compareChunks } from '../utils/optimization.js';

describe('Performance Benchmarks', () => {
  // Pre-populate chunks to test
  const hostDist = path.resolve(__dirname, '../../host/dist');
  const remoteDist = path.resolve(__dirname, '../../remote/dist');
  const chunksToTest = [];
  
  [hostDist, remoteDist].forEach(distPath => {
    if (fs.existsSync(distPath)) {
      const lodashChunk = fs.readdirSync(distPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.original'));
      
      if (lodashChunk) {
        chunksToTest.push({
          name: path.basename(distPath) === 'host' ? 'Host' : 'Remote',
          path: path.join(distPath, lodashChunk),
          distPath
        });
      }
    }
  });
  
  describe('Optimization Speed', () => {
    it.each(chunksToTest)('should optimize $name chunk in reasonable time', ({ name, path: chunkPath }) => {
      const config = {
        treeShake: {
          'lodash-es': {
            sortBy: true,
            uniq: true,
            map: false,
            filter: false,
            reduce: false
          }
        ,
        // use chunk_characteristics-only configuration
        // attach entry to the library config when running optimizeChunk
      };
      
      const startTime = performance.now();
      const optimized = optimizeChunk(chunkPath, config);
      const endTime = performance.now();
      
      const duration = endTime - startTime;
      
      console.log(`${name} optimization time: ${duration.toFixed(2)}ms`);
      
      // Should complete within 5 seconds for large chunks
      expect(duration).toBeLessThan(5000);
      
      // Should produce valid output
      expect(optimized.length).toBeGreaterThan(1000);
    });
  });
  
  describe('Size Reduction Metrics', () => {
    it.each(chunksToTest)('should achieve significant reduction for $name', ({ name, path: chunkPath, distPath }) => {
      const optimizedPath = chunkPath.replace('.original', '');
      
      if (!fs.existsSync(optimizedPath)) {
        console.log(`Skipping ${name} - optimized version not found`);
        return;
      }
      
      const comparison = compareChunks(chunkPath, optimizedPath);
      
      console.log(`\n${name} Optimization Results:`);
      console.log(`  Original: ${comparison.original.sizeKB}KB (${comparison.original.moduleCount} modules)`);
      console.log(`  Optimized: ${comparison.optimized.sizeKB}KB (${comparison.optimized.moduleCount} modules)`);
      console.log(`  Size reduction: ${comparison.reduction}%`);
      console.log(`  Module reduction: ${comparison.moduleReduction}%`);
      console.log(`  Saved: ${comparison.sizeSavedKB}KB`);
      
      // Expect at least 30% size reduction
      expect(parseFloat(comparison.reduction)).toBeGreaterThan(30);
    });
  });
  
  describe('Memory Usage', () => {
    it('should handle large chunks without excessive memory', () => {
      const testChunk = chunksToTest[0];
      if (!testChunk) {
        console.log('No chunks available for memory test');
        return;
      }
      
      const config = {
        treeShake: { 'lodash-es': { default: true, chunk_characteristics: { entry_module_id: 'lodash.js' } } }
      };
      
      const memBefore = process.memoryUsage();
      
      // Run optimization multiple times
      for (let i = 0; i < 5; i++) {
        optimizeChunk(testChunk.path, config);
      }
      
      const memAfter = process.memoryUsage();
      
      const heapUsed = (memAfter.heapUsed - memBefore.heapUsed) / 1024 / 1024;
      console.log(`Memory increase after 5 optimizations: ${heapUsed.toFixed(2)}MB`);
      
      // Should not leak excessive memory (< 100MB for 5 runs)
      expect(heapUsed).toBeLessThan(100);
    });
  });
  
  describe('Optimization Quality', () => {
    it('should preserve all used exports', () => {
      const fixture = path.join(__dirname, '../fixtures/lodash-chunk.js');
      const config = {
        treeShake: {
          'lodash-es': {
            sortBy: true,
            uniq: true,
            map: false,
            filter: false,
            reduce: false,
            chunk_characteristics: { entry_module_id: '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js' }
          }
        }
      };
      
      const optimized = optimizeChunk(fixture, config);
      
      // Check preserved functions
      expect(optimized).toContain('sortBy');
      expect(optimized).toContain('uniq');
      
      // Check removed functions
      const removedCorrectly = 
        !optimized.includes('function map') &&
        !optimized.includes('function filter') &&
        !optimized.includes('function reduce');
      
      expect(removedCorrectly).toBe(true);
    });
    
    it('should maintain valid JavaScript syntax', () => {
      const fixture = path.join(__dirname, '../fixtures/lodash-chunk.js');
      const config = {
        treeShake: { 'lodash-es': { sortBy: true, chunk_characteristics: { entry_module_id: 'lodash.js' } } }
      };
      
      const optimized = optimizeChunk(fixture, config);
      
      // Basic syntax checks
      expect(optimized).toContain('exports.modules');
      expect(optimized).toMatch(/function\s*\(/);
      expect(optimized).toMatch(/};\s*$/);
      
      // Check balanced braces
      const openBraces = (optimized.match(/{/g) || []).length;
      const closeBraces = (optimized.match(/}/g) || []).length;
      expect(openBraces).toBe(closeBraces);
    });
  });
});