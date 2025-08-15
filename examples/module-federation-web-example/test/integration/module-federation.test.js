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
      
      // New structure has treeShake at top level
      expect(shareUsage.treeShake).toBeDefined();
      expect(shareUsage.treeShake['lodash-es']).toBeDefined();
      
      const lodashUsage = shareUsage.treeShake['lodash-es'];
      // In the new format, used exports are marked as true
      expect(lodashUsage.sortBy).toBe(true);
      expect(lodashUsage.uniq).toBe(true);
      expect(lodashUsage.default).toBe(true);
    });
    
    it('should track remote app lodash usage', () => {
      const shareUsage = JSON.parse(
        fs.readFileSync(path.join(remoteDistPath, 'share-usage.json'), 'utf-8')
      );
      
      // New structure has treeShake at top level
      expect(shareUsage.treeShake).toBeDefined();
      expect(shareUsage.treeShake['lodash-es']).toBeDefined();
      
      const lodashUsage = shareUsage.treeShake['lodash-es'];
      // In the new format, used exports are marked as true
      expect(lodashUsage.capitalize).toBe(true);
      expect(lodashUsage.debounce).toBe(true);
      expect(lodashUsage.groupBy).toBe(true);
      expect(lodashUsage.throttle).toBe(true);
      expect(lodashUsage.pick).toBe(true);
      expect(lodashUsage.omit).toBe(true);
      expect(lodashUsage.default).toBe(true);
    });
    
    it('should track remote app ramda usage', () => {
      const shareUsage = JSON.parse(
        fs.readFileSync(path.join(remoteDistPath, 'share-usage.json'), 'utf-8')
      );
      
      expect(shareUsage.treeShake).toBeDefined();
      expect(shareUsage.treeShake['ramda']).toBeDefined();
      
      const ramdaUsage = shareUsage.treeShake['ramda'];
      // In the new format, used exports are marked as true
      expect(ramdaUsage.compose).toBe(true);
      expect(ramdaUsage.curry).toBe(true);
      expect(ramdaUsage.map).toBe(true);
      expect(ramdaUsage.filter).toBe(true);
      expect(ramdaUsage.reduce).toBe(true);
      expect(ramdaUsage.pipe).toBe(true);
      expect(ramdaUsage.prop).toBe(true);
      expect(ramdaUsage.path).toBe(true);
    });
    
    it('should track remote app date-fns usage', () => {
      const shareUsage = JSON.parse(
        fs.readFileSync(path.join(remoteDistPath, 'share-usage.json'), 'utf-8')
      );
      
      expect(shareUsage.treeShake).toBeDefined();
      expect(shareUsage.treeShake['date-fns']).toBeDefined();
      
      const dateFnsUsage = shareUsage.treeShake['date-fns'];
      // In the new format, used exports are marked as true
      expect(dateFnsUsage.format).toBe(true);
      expect(dateFnsUsage.parseISO).toBe(true);
      expect(dateFnsUsage.addDays).toBe(true);
      expect(dateFnsUsage.subDays).toBe(true);
      expect(dateFnsUsage.isAfter).toBe(true);
      expect(dateFnsUsage.isBefore).toBe(true);
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
    
    it('should create optimized shared library chunks', () => {
      // Check lodash-es
      const hostLodashChunk = fs.readdirSync(hostDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original'));
      const hostOriginalChunk = hostLodashChunk + '.original';
      
      expect(fs.existsSync(path.join(hostDistPath, hostLodashChunk))).toBe(true);
      expect(fs.existsSync(path.join(hostDistPath, hostOriginalChunk))).toBe(true);
      
      // Check ramda
      const remoteRamdaChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('ramda') && file.endsWith('.js') && !file.endsWith('.original'));
      if (remoteRamdaChunk) {
        const ramdaOriginalChunk = remoteRamdaChunk + '.original';
        expect(fs.existsSync(path.join(remoteDistPath, remoteRamdaChunk))).toBe(true);
        expect(fs.existsSync(path.join(remoteDistPath, ramdaOriginalChunk))).toBe(true);
      }
      
      // Check date-fns
      const remoteDateFnsChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('date-fns') && file.endsWith('.js') && !file.endsWith('.original'));
      if (remoteDateFnsChunk) {
        const dateFnsOriginalChunk = remoteDateFnsChunk + '.original';
        expect(fs.existsSync(path.join(remoteDistPath, remoteDateFnsChunk))).toBe(true);
        expect(fs.existsSync(path.join(remoteDistPath, dateFnsOriginalChunk))).toBe(true);
      }
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
    
    it('should execute host app with optimized chunks', async () => {
      // Create a test that verifies lodash functions actually work after optimization
      const testScript = path.join(hostDistPath, 'test-lodash-functions.js');
      const vendorChunk = fs.readdirSync(hostDistPath)
        .find(file => {
          if (!file.endsWith('.js') || file.endsWith('.original')) return false;
          try {
            const content = fs.readFileSync(path.join(hostDistPath, file), 'utf8');
            return content.includes('lodash') && content.includes('sortBy');
          } catch {
            return false;
          }
        });
      if (!vendorChunk) {
        throw new Error('Could not find lodash-es vendor chunk in host/dist');
      }
      fs.writeFileSync(testScript, `
        (async () => {
          try {
            const fs = require('fs');
            // Provide browser-like globals for JSONP
            global.self = globalThis;
            self["webpackChunkapp"] = [];
            let capturedModules = null;
            self["webpackChunkapp"].push = function(data) {
              // JSONP push([chunkIds, modules])
              if (Array.isArray(data) && data.length >= 2) {
                capturedModules = data[1];
              }
            };

            // Intercept Array.prototype.push to capture JSONP module table
            const __origPush = Array.prototype.push;
            Array.prototype.push = function(...args) {
              if (
                args.length > 0 &&
                Array.isArray(args[0]) &&
                args[0].length >= 2 &&
                args[0][1] && typeof args[0][1] === 'object'
              ) {
                capturedModules = args[0][1];
              }
              return __origPush.apply(this, args);
            };

            // Load the vendor chunk to populate capturedModules
            require('./${vendorChunk}');
            Array.prototype.push = __origPush;

            if (!capturedModules) {
              throw new Error('Failed to capture modules from JSONP vendor chunk');
            }

            // Set up webpack modules environment
            global.__webpack_modules__ = capturedModules;
            global.__webpack_require__ = function(moduleId) {
              if (!__webpack_modules__[moduleId]) {
                throw new Error('Module ' + moduleId + ' not found');
              }
              const module = { exports: {} };
              __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
              return module.exports;
            };
            __webpack_require__.r = (exports) => {
              if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
              }
              Object.defineProperty(exports, '__esModule', { value: true });
            };
            __webpack_require__.d = (exports, definition) => {
              for (const key in definition) {
                if (__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
                  Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
                }
              }
            };
            __webpack_require__.o = (obj, prop) => Object.prototype.hasOwnProperty.call(obj, prop);

            // Resolve lodash aggregator module id dynamically
            const aggregatorId = Object.keys(__webpack_modules__).find(id => id.endsWith('/lodash-es/lodash.js') || id.endsWith('/lodash.js'))
              || Object.keys(__webpack_modules__).find(id => id.includes('lodash-es/lodash.js'));
            if (!aggregatorId) throw new Error('Could not resolve lodash aggregator module id');

            const lodashExports = __webpack_require__(aggregatorId);
            
            console.log('Host App');
            console.log('Testing lodash functions from optimized chunk:');
            
            // Test sortBy - MUST NOT BE NULL
            if (typeof lodashExports.sortBy !== 'function') {
              throw new Error('sortBy is not a function! It was replaced with null!');
            }
            const items = [
              { name: 'Banana', category: 'fruit', price: 0.8 },
              { name: 'Apple', category: 'fruit', price: 1.5 },
              { name: 'Carrot', category: 'vegetable', price: 1.2 }
            ];
            const sorted = lodashExports.sortBy(items, 'price');
            console.log('Sorted data:', sorted.map(i => i.name).join(', '));
            
            // Test uniq - MUST NOT BE NULL
            if (typeof lodashExports.uniq !== 'function') {
              throw new Error('uniq is not a function! It was replaced with null!');
            }
            const numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            const unique = lodashExports.uniq(numbers);
            console.log('Unique numbers:', unique.sort((a, b) => a - b).join(', '));
            
            // Test capitalize - MUST NOT BE NULL
            if (typeof lodashExports.capitalize !== 'function') {
              throw new Error('capitalize is not a function! It was replaced with null!');
            }
            const capitalized = lodashExports.capitalize('hello world');
            console.log('Capitalized text:', capitalized);
            
            // Verify that unused functions ARE null
            if (lodashExports.zip !== null) {
              throw new Error('zip should be null but it is not!');
            }
            if (lodashExports.zipObject !== null) {
              throw new Error('zipObject should be null but it is not!');
            }
            console.log('Unused functions correctly nullified');
            
            console.log('All tests completed successfully');
          } catch (error) {
            console.error('Test failed:', error.message);
            console.error(error.stack);
            process.exit(1);
          }
        })();
      `);
      
      const result = execSync(`node ${testScript}`, {
        cwd: hostDistPath,
        encoding: 'utf-8'
      });
      
      // Check that the functions actually work
      expect(result).toContain('Host App');
      expect(result).toContain('Testing lodash functions from optimized chunk:');
      expect(result).toContain('Sorted data: Banana, Carrot, Apple');
      expect(result).toContain('Unique numbers: 1, 2, 3, 4, 5, 6, 9');
      expect(result).toContain('Capitalized text: Hello world');
      expect(result).toContain('Unused functions correctly nullified');
      expect(result).toContain('All tests completed successfully');
      
      // Make sure no errors about null functions
      expect(result).not.toContain('is not a function');
      expect(result).not.toContain('It was replaced with null');
      
      // Clean up
      fs.unlinkSync(testScript);
    });
    
    it('should execute remote app with optimized chunks', async () => {
      // Create a test that verifies lodash functions actually work after optimization
      const testScript = path.join(remoteDistPath, 'test-lodash-functions.js');
      const vendorChunk = fs.readdirSync(remoteDistPath)
        .find(file => {
          if (!file.endsWith('.js') || file.endsWith('.original')) return false;
          try {
            const content = fs.readFileSync(path.join(remoteDistPath, file), 'utf8');
            return content.includes('lodash') && content.includes('capitalize');
          } catch {
            return false;
          }
        });
      if (!vendorChunk) {
        throw new Error('Could not find lodash-es vendor chunk in remote/dist');
      }
      fs.writeFileSync(testScript, `
        (async () => {
          try {
            const fs = require('fs');
            // Provide browser-like globals for JSONP
            global.self = globalThis;
            self["webpackChunkapp"] = [];
            let capturedModules = null;
            self["webpackChunkapp"].push = function(data) {
              if (Array.isArray(data) && data.length >= 2) {
                capturedModules = data[1];
              }
            };

            // Intercept Array.prototype.push to capture JSONP module table
            const __origPush = Array.prototype.push;
            Array.prototype.push = function(...args) {
              if (
                args.length > 0 &&
                Array.isArray(args[0]) &&
                args[0].length >= 2 &&
                args[0][1] && typeof args[0][1] === 'object'
              ) {
                capturedModules = args[0][1];
              }
              return __origPush.apply(this, args);
            };

            // Load the vendor chunk to populate capturedModules
            require('./${vendorChunk}');
            Array.prototype.push = __origPush;

            if (!capturedModules) {
              throw new Error('Failed to capture modules from JSONP vendor chunk');
            }

            // Set up webpack modules environment
            global.__webpack_modules__ = capturedModules;
            global.__webpack_require__ = function(moduleId) {
              if (!__webpack_modules__[moduleId]) {
                throw new Error('Module ' + moduleId + ' not found');
              }
              const module = { exports: {} };
              __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
              return module.exports;
            };
            __webpack_require__.r = (exports) => {
              if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
              }
              Object.defineProperty(exports, '__esModule', { value: true });
            };
            __webpack_require__.d = (exports, definition) => {
              for (const key in definition) {
                if (__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
                  Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
                }
              }
            };
            __webpack_require__.o = (obj, prop) => Object.prototype.hasOwnProperty.call(obj, prop);

            // Resolve lodash aggregator module id dynamically
            const aggregatorId = Object.keys(__webpack_modules__).find(id => id.endsWith('/lodash-es/lodash.js') || id.endsWith('/lodash.js'))
              || Object.keys(__webpack_modules__).find(id => id.includes('lodash-es/lodash.js'));
            if (!aggregatorId) throw new Error('Could not resolve lodash aggregator module id');

            const lodashExports = __webpack_require__(aggregatorId);

            console.log('Remote App');
            console.log('Testing lodash functions from optimized chunk:');

            // Remote-specific checks (reuse a subset)
            if (typeof lodashExports.capitalize !== 'function') {
              throw new Error('capitalize is not a function! It was replaced with null!');
            }
            const capitalized = lodashExports.capitalize('hello world');
            console.log('Capitalized text:', capitalized);

            if (lodashExports.zip !== null) {
              throw new Error('zip should be null but it is not!');
            }
            if (lodashExports.zipObject !== null) {
              throw new Error('zipObject should be null but it is not!');
            }
            console.log('Unused functions correctly nullified');

            console.log('All tests completed successfully');
          } catch (error) {
            console.error('Test failed:', error.message);
            console.error(error.stack);
            process.exit(1);
          }
        })();
      `);

      const result = execSync(`node ${testScript}`, {
        cwd: remoteDistPath,
        encoding: 'utf-8'
      });

      expect(result).toContain('Remote App');
      expect(result).toContain('Testing lodash functions from optimized chunk:');
      expect(result).toContain('Capitalized text: Hello world');
      expect(result).toContain('Unused functions correctly nullified');
      expect(result).toContain('All tests completed successfully');

      // Make sure no errors about null functions
      expect(result).not.toContain('is not a function');
      expect(result).not.toContain('It was replaced with null');

      // Clean up
      fs.unlinkSync(testScript);
    });
    
    it('should remove unused modules from all shared library chunks', () => {
      // Test lodash-es chunk
      const lodashChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (lodashChunk) {
        const lodashContent = fs.readFileSync(path.join(remoteDistPath, lodashChunk), 'utf-8');
        
        // Check that the optimization has taken effect by verifying size reduction
        const originalChunk = lodashChunk + '.original';
        if (fs.existsSync(path.join(remoteDistPath, originalChunk))) {
          const originalSize = fs.statSync(path.join(remoteDistPath, originalChunk)).size;
          const optimizedSize = fs.statSync(path.join(remoteDistPath, lodashChunk)).size;
          expect(optimizedSize).toBeLessThan(originalSize);
          
          // Significant reduction expected
          const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
          expect(reduction).toBeGreaterThan(25);
        }
        
        // These should be kept (check for full module paths)
        const keptLodashModules = ['groupBy.js', 'debounce.js', 'throttle.js', 'pick.js', 'omit.js'];
        keptLodashModules.forEach(module => {
          expect(lodashContent).toContain(`/lodash-es/${module}"`);
        });
      }
      
      // Test ramda chunk
      const ramdaChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('ramda') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (ramdaChunk) {
        const ramdaContent = fs.readFileSync(path.join(remoteDistPath, ramdaChunk), 'utf-8');
        
        // Check that the optimization has taken effect by verifying size reduction
        const originalChunk = ramdaChunk + '.original';
        if (fs.existsSync(path.join(remoteDistPath, originalChunk))) {
          const originalSize = fs.statSync(path.join(remoteDistPath, originalChunk)).size;
          const optimizedSize = fs.statSync(path.join(remoteDistPath, ramdaChunk)).size;
          expect(optimizedSize).toBeLessThan(originalSize);
          
          // Significant reduction expected
          const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
          expect(reduction).toBeGreaterThan(40);
        }
        
        // These should be kept (check for full module paths)
        const keptRamdaModules = ['compose.js', 'curry.js', 'map.js', 'filter.js', 'reduce.js'];
        keptRamdaModules.forEach(module => {
          expect(ramdaContent).toContain(`/ramda/es/${module}"`);
        });
      }
      
      // Test date-fns chunk  
      const dateFnsChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('date-fns') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (dateFnsChunk) {
        const dateFnsContent = fs.readFileSync(path.join(remoteDistPath, dateFnsChunk), 'utf-8');
        
        // Check that the optimization has taken effect by verifying size reduction
        const originalChunk = dateFnsChunk + '.original';
        if (fs.existsSync(path.join(remoteDistPath, originalChunk))) {
          const originalSize = fs.statSync(path.join(remoteDistPath, originalChunk)).size;
          const optimizedSize = fs.statSync(path.join(remoteDistPath, dateFnsChunk)).size;
          expect(optimizedSize).toBeLessThan(originalSize);
          
          // Significant reduction expected
          const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
          expect(reduction).toBeGreaterThan(40);
        }
        
        // These functions should be kept (check for module paths)
        const keptDateFns = ['format', 'parseISO', 'addDays', 'subDays', 'isAfter', 'isBefore'];
        keptDateFns.forEach(func => {
          const hasFunction = 
            dateFnsContent.includes(`/date-fns/${func}.js"`) ||
            dateFnsContent.includes(`/date-fns/${func}/index.js"`);
          expect(hasFunction).toBe(true);
        });
      }
    });
    
    it('should achieve significant size reduction for all shared libraries', () => {
      const results = [];
      
      // Check lodash-es reduction
      const lodashChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (lodashChunk) {
        const optimizedSize = fs.statSync(path.join(remoteDistPath, lodashChunk)).size;
        const originalSize = fs.statSync(path.join(remoteDistPath, lodashChunk + '.original')).size;
        const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
        results.push({ library: 'lodash-es', reduction });
        expect(reduction).toBeGreaterThan(30);
      }
      
      // Check ramda reduction
      const ramdaChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('ramda') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (ramdaChunk) {
        const optimizedSize = fs.statSync(path.join(remoteDistPath, ramdaChunk)).size;
        const originalSize = fs.statSync(path.join(remoteDistPath, ramdaChunk + '.original')).size;
        const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
        results.push({ library: 'ramda', reduction });
        expect(reduction).toBeGreaterThan(40);
      }
      
      // Check date-fns reduction
      const dateFnsChunk = fs.readdirSync(remoteDistPath)
        .find(file => file.includes('date-fns') && file.endsWith('.js') && !file.endsWith('.original') && !file.endsWith('.map'));
      if (dateFnsChunk) {
        const optimizedSize = fs.statSync(path.join(remoteDistPath, dateFnsChunk)).size;
        const originalSize = fs.statSync(path.join(remoteDistPath, dateFnsChunk + '.original')).size;
        const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
        results.push({ library: 'date-fns', reduction });
        expect(reduction).toBeGreaterThan(40);
      }
      
      console.log('Size reduction results:');
      results.forEach(result => {
        console.log(`  ${result.library}: ${result.reduction.toFixed(2)}%`);
      });
    });
  });
});