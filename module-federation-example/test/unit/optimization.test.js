import { describe, it, expect, beforeAll } from 'vitest';
import fs from 'fs';
import path from 'path';
import { optimize } from 'swc_macro_wasm';

describe('SWC Macro Optimization', () => {
  const fixturesPath = path.resolve(__dirname, '../fixtures');
  
  describe('Tree Shaking', () => {
    it('should remove unused lodash exports', () => {
      const chunk = fs.readFileSync(
        path.join(fixturesPath, 'lodash-chunk.js'), 
        'utf-8'
      );
      
      const config = {
        treeShake: {
          'lodash-es': {
            sortBy: true,
            uniq: true,
            map: false,
            filter: false,
            reduce: false
          }
        },
        entryModules: {
          'lodash-es': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
        }
      };
      
      const optimized = optimize(chunk, JSON.stringify(config));
      
      // Check that enabled exports are preserved
      expect(optimized).toContain('sortBy');
      expect(optimized).toContain('uniq');
      
      // Check that disabled exports are removed or nullified
      expect(optimized).not.toContain('function map');
      expect(optimized).not.toContain('function filter');
      expect(optimized).not.toContain('function reduce');
      
      // Check size reduction
      const reduction = ((chunk.length - optimized.length) / chunk.length) * 100;
      expect(reduction).toBeGreaterThan(20);
    });
    
    it('should handle Module Federation share-usage.json format', () => {
      const shareUsage = {
        consume_shared_modules: {
          'lodash-es': {
            used_exports: ['sortBy', 'uniq', 'default'],
            unused_exports: ['map', 'filter', 'reduce'],
            entry_module_id: '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
          }
        }
      };
      
      // Convert share usage to tree shake config
      const usedExports = shareUsage.consume_shared_modules['lodash-es'].used_exports;
      const unusedExports = shareUsage.consume_shared_modules['lodash-es'].unused_exports;
      const entryModuleId = shareUsage.consume_shared_modules['lodash-es'].entry_module_id;
      
      const treeShakeConfig = {};
      usedExports.forEach(exp => { treeShakeConfig[exp] = true; });
      unusedExports.forEach(exp => { treeShakeConfig[exp] = false; });
      
      const config = {
        treeShake: { 'lodash-es': treeShakeConfig },
        entryModules: { 'lodash-es': entryModuleId }
      };
      
      expect(config.entryModules['lodash-es']).toBe(entryModuleId);
      expect(config.treeShake['lodash-es'].sortBy).toBe(true);
      expect(config.treeShake['lodash-es'].map).toBe(false);
    });
  });
  
  describe('CommonJS Split Chunks', () => {
    it('should preserve module structure in exports.modules format', () => {
      const chunk = `
        "use strict";
        exports.ids = ["vendors-lodash"];
        exports.modules = {
          "lodash/sortBy.js": function(module, exports, __webpack_require__) {
            exports.sortBy = function() { return "sortBy"; };
          },
          "lodash/map.js": function(module, exports, __webpack_require__) {
            exports.map = function() { return "map"; };
          }
        };
      `;
      
      const config = {
        treeShake: {
          'lodash-es': { sortBy: true, map: false }
        },
        entryModules: {
          'lodash-es': 'lodash/lodash.js'
        }
      };
      
      const optimized = optimize(chunk, JSON.stringify(config));
      
      // Should maintain CommonJS structure
      expect(optimized).toContain('exports.ids');
      expect(optimized).toContain('exports.modules');
    });
  });
  
  describe('Macro Processing', () => {
    it('should process @common:if conditions correctly', () => {
      const chunk = `
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        exports.sortBy = __webpack_require__("sortBy.js").default;
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.map"] */
        exports.map = __webpack_require__("map.js").default;
        /* @common:endif */
      `;
      
      const config = {
        treeShake: {
          'lodash-es': { sortBy: true, map: false }
        }
      };
      
      const optimized = optimize(chunk, JSON.stringify(config));
      
      expect(optimized).toContain('sortBy');
      expect(optimized).not.toContain('exports.map');
    });
  });
});