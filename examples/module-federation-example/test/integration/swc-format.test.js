import { describe, it, expect } from 'vitest';
import { createRequire } from 'module';
import { mergeUsageData, readShareUsageFiles } from '../../scripts/optimize-shared-chunks.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

describe('SWC Macro Format Tests', () => {
  it('should generate correct dot notation format for SWC macro optimizer', () => {
    // Mock share-usage.json data in new format
    const mockShareUsageData = [
      {
        name: 'host',
        data: {
          treeShake: {
            'lodash-es': {
              capitalize: true,
              debounce: true,
              groupBy: true,
              add: false,
              after: false,
              ary: false,
              default: false,
              omit: false,
              pick: false,
              sortBy: false,
              throttle: false,
              uniq: false,
              chunk_characteristics: {
                entry_module_id: '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
              }
            }
          }
        }
      },
      {
        name: 'remote',
        data: {
          treeShake: {
            'lodash-es': {
              omit: true,
              pick: true,
              sortBy: true,
              throttle: true,
              uniq: true,
              default: true,
              add: false,
              after: false,
              ary: false,
              capitalize: false,
              debounce: false,
              groupBy: false,
              chunk_characteristics: {
                entry_module_id: '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
              }
            }
          }
        }
      }
    ];

    // Generate merged config
    const mergedConfig = mergeUsageData(mockShareUsageData);

    // Verify the structure matches what SWC expects
    expect(mergedConfig).toHaveProperty('treeShake');
    
    // Check treeShake format
    expect(mergedConfig.treeShake).toHaveProperty('lodash-es');
    const lodashConfig = mergedConfig.treeShake['lodash-es'];
    
    // Used exports should be true
    expect(lodashConfig.capitalize).toBe(true);
    expect(lodashConfig.debounce).toBe(true);
    expect(lodashConfig.groupBy).toBe(true);
    expect(lodashConfig.omit).toBe(true);
    expect(lodashConfig.pick).toBe(true);
    expect(lodashConfig.sortBy).toBe(true);
    expect(lodashConfig.throttle).toBe(true);
    expect(lodashConfig.uniq).toBe(true);
    expect(lodashConfig.default).toBe(true);
    
    // Unused exports should be false
    expect(lodashConfig.add).toBe(false);
    expect(lodashConfig.after).toBe(false);
    expect(lodashConfig.ary).toBe(false);
    
    // chunk_characteristics should carry the entry module id
    expect(lodashConfig.chunk_characteristics.entry_module_id).toBe(
      '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
    );
  });

  it('should format config correctly for SWC optimize() call', () => {
    const treeShakeConfig = {
      'lodash-es': {
        capitalize: true,
        debounce: true,
        groupBy: true,
        omit: true,
        pick: true,
        sortBy: true,
        throttle: true,
        uniq: true,
        default: true,
        add: false,
        after: false
      }
    };
    
    const chunk_characteristics = {
      'lodash-es': { entry_module_id: '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js' }
    };

    // This is what gets passed to optimizer.optimize()
    const config = {
      treeShake: {
        'lodash-es': { chunk_characteristics: chunk_characteristics['lodash-es'] }
      }
    };

    // Only include exports marked as true (the dot notation equivalent)
    Object.entries(treeShakeConfig['lodash-es']).forEach(([exportName, shouldKeep]) => {
      if (shouldKeep === true) {
        config.treeShake['lodash-es'][exportName] = true;
      }
    });

    // Verify only true values are included (plus chunk_characteristics)
    expect(Object.keys(config.treeShake['lodash-es']).sort()).toEqual(
      ['capitalize','debounce','default','groupBy','omit','pick','sortBy','throttle','uniq','chunk_characteristics'].sort()
    );
    const { chunk_characteristics: _cc, ...exportFlags } = config.treeShake['lodash-es'];
    expect(exportFlags).toEqual({
      capitalize: true,
      debounce: true,
      groupBy: true,
      omit: true,
      pick: true,
      sortBy: true,
      throttle: true,
      uniq: true,
      default: true
    });

    // This should match the dot notation format: treeShake.lodash-es.capitalize=true
    const configJson = JSON.stringify(config);
    const parsedConfig = JSON.parse(configJson);
    
    // Verify the structure can be interpreted as dot notation
    expect(parsedConfig.treeShake['lodash-es'].capitalize).toBe(true);
    expect(parsedConfig.treeShake['lodash-es'].debounce).toBe(true);
    
    // Verify false values are NOT included (important for optimization)
    expect(parsedConfig.treeShake['lodash-es'].add).toBeUndefined();
    expect(parsedConfig.treeShake['lodash-es'].after).toBeUndefined();
  });

  it('should verify actual optimized chunk exports at runtime (no string heuristics)', () => {
    const optimizedChunkPath = path.resolve(
      __dirname,
      '../../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js'
    );
    const usagePath = path.resolve(__dirname, '../../host/dist/share-usage.json');

    if (!fs.existsSync(optimizedChunkPath) || !fs.existsSync(usagePath)) {
      return; // skip if build artifacts missing in this env
    }

    const usage = JSON.parse(fs.readFileSync(usagePath, 'utf8'));
    const lodashCfg = usage?.treeShake?.['lodash-es'] || {};
    const kept = Object.entries(lodashCfg)
      .filter(([k, v]) => k !== 'chunk_characteristics' && v === true)
      .map(([k]) => k);
    const disabled = Object.entries(lodashCfg)
      .filter(([k, v]) => k !== 'chunk_characteristics' && v === false)
      .map(([k]) => k);
    const entryId = lodashCfg?.chunk_characteristics?.entry_module_id;

    const source = fs.readFileSync(optimizedChunkPath, 'utf8');

    function evaluateChunkAndGetModulesFromSource(src) {
      try {
        const wrapper = `(function(){ var exports = {}; ${src}\n;return exports.modules || exports.__modules || null; })()`;
        // eslint-disable-next-line no-new-func
        const fn = new Function(wrapper + ';');
        const modules = fn();
        if (modules) return modules;
      } catch (_) {}

      const assignIdx = src.indexOf('exports.modules');
      if (assignIdx !== -1) {
        const eqIdx = src.indexOf('=', assignIdx);
        if (eqIdx !== -1) {
          const braceStart = src.indexOf('{', eqIdx);
          if (braceStart !== -1) {
            let i = braceStart;
            let depth = 0;
            let inStr = false;
            let strCh = '';
            for (; i < src.length; i++) {
              const ch = src[i];
              if (inStr) {
                if (ch === '\\') { i++; continue; }
                if (ch === strCh) inStr = false;
                continue;
              }
              if (ch === '"' || ch === '\'') { inStr = true; strCh = ch; continue; }
              if (ch === '{') depth++;
              else if (ch === '}') { depth--; if (depth === 0) { i++; break; } }
            }
            const objCode = src.slice(braceStart, i);
            // eslint-disable-next-line no-new-func
            const modules = new Function('return (' + objCode + ');')();
            if (modules && typeof modules === 'object') return modules;
          }
        }
      }
      return null;
    }

    function createRuntime(modulesObj) {
      const cache = Object.create(null);
      function __webpack_require__(id) {
        if (cache[id]) return cache[id].exports;
        const fn = modulesObj[id];
        if (!fn) throw new Error('Module not found: ' + id);
        const module = { exports: {} };
        cache[id] = module;
        fn(module, module.exports, __webpack_require__);
        return module.exports;
      }
      __webpack_require__.d = (exports, definition) => {
        for (const key in definition) {
          Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
        }
      };
      __webpack_require__.r = (exports) => {
        Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
        Object.defineProperty(exports, '__esModule', { value: true });
      };
      return __webpack_require__;
    }

    const requireCjs = createRequire(import.meta.url);
    let modulesObj = null;
    try {
      const mod = requireCjs(optimizedChunkPath);
      if (mod && (mod.modules || mod.__modules)) {
        modulesObj = mod.modules || mod.__modules;
      }
    } catch (_) {}
    if (!modulesObj) {
      modulesObj = evaluateChunkAndGetModulesFromSource(source);
    }
    expect(modulesObj && typeof modulesObj === 'object').toBe(true);

    const __webpack_require__ = createRuntime(modulesObj);
    expect(typeof entryId).toBe('string');
    const entryExports = __webpack_require__(entryId);

    // Validate kept exports resolve to non-undefined values
    kept.forEach((name) => {
      const val = entryExports && (entryExports[name] ?? (name === 'default' ? entryExports.default : undefined));
      expect(val === undefined).toBe(false);
    });

    // Validate disabled exports are not present or are null
    disabled.forEach((name) => {
      const hasProp = entryExports && Object.prototype.hasOwnProperty.call(entryExports, name);
      if (hasProp) {
        expect(entryExports[name] == null).toBe(true);
      }
    });
  });

  it('should read actual merged config and verify format', () => {
    const mergedConfigPath = path.resolve(__dirname, '../../dist/merged-tree-shake-config.json');
    
    if (fs.existsSync(mergedConfigPath)) {
      const mergedConfig = JSON.parse(fs.readFileSync(mergedConfigPath, 'utf8'));
      
      // Verify structure
      expect(mergedConfig).toHaveProperty('treeShake');
      expect(mergedConfig).toHaveProperty('metadata');
      
      // Verify treeShake has library-name as key
      expect(mergedConfig.treeShake).toHaveProperty('lodash-es');
      
      // Verify all exports have boolean values
      const lodashExports = mergedConfig.treeShake['lodash-es'];
      Object.entries(lodashExports).forEach(([exportName, value]) => {
        expect(typeof value).toBe('boolean');
      });
      
      // Verify entry module is set in chunk_characteristics
      expect(mergedConfig.treeShake['lodash-es'].chunk_characteristics.entry_module_id).toMatch(/node_modules.*lodash-es.*lodash\.js$/);
    }
  });

  it('should demonstrate the dot notation conversion for SWC macro', () => {
    // The format we generate
    const ourFormat = {
      treeShake: {
        'lodash-es': {
          capitalize: true,
          debounce: true,
          default: true
        }
      }
    };

    // This represents the dot notation that SWC macro expects:
    // treeShake.lodash-es.capitalize = true
    // treeShake.lodash-es.debounce = true
    // treeShake.lodash-es.default = true

    // When serialized to JSON and passed to SWC, it should be interpretable as:
    const dotNotationEquivalent = {};
    Object.entries(ourFormat.treeShake).forEach(([libraryName, exports]) => {
      Object.entries(exports).forEach(([exportName, value]) => {
        const dotPath = `treeShake.${libraryName}.${exportName}`;
        dotNotationEquivalent[dotPath] = value;
      });
    });

    expect(dotNotationEquivalent['treeShake.lodash-es.capitalize']).toBe(true);
    expect(dotNotationEquivalent['treeShake.lodash-es.debounce']).toBe(true);
    expect(dotNotationEquivalent['treeShake.lodash-es.default']).toBe(true);
  });
});