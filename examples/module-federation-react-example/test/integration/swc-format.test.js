import { describe, it, expect } from 'vitest';
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
    
    // chunk_characteristics should carry the entry module id if present
    if (lodashConfig.chunk_characteristics) {
      expect(lodashConfig.chunk_characteristics.entry_module_id).toBe(
        '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js'
      );
    }
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
    
    const entryModuleId = '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js';
    
    // This is what gets passed to optimizer.optimize()
    const config = {
      treeShake: {
        'lodash-es': { chunk_characteristics: { entry_module_id: entryModuleId } }
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

  it('should verify actual optimized chunk has correct macro format', () => {
    const optimizedChunkPath = path.resolve(__dirname, '../../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
    
    if (fs.existsSync(optimizedChunkPath)) {
      const content = fs.readFileSync(optimizedChunkPath, 'utf8');
      
      // Check that preserved exports reference their modules correctly
      const preservedExports = [
        'capitalize: ()=>_capitalize_js__WEBPACK_IMPORTED_MODULE_',
        'debounce: ()=>_debounce_js__WEBPACK_IMPORTED_MODULE_',
        'groupBy: ()=>_groupBy_js__WEBPACK_IMPORTED_MODULE_',
        'omit: ()=>_omit_js__WEBPACK_IMPORTED_MODULE_',
        'pick: ()=>_pick_js__WEBPACK_IMPORTED_MODULE_',
        'sortBy: ()=>_sortBy_js__WEBPACK_IMPORTED_MODULE_',
        'throttle: ()=>_throttle_js__WEBPACK_IMPORTED_MODULE_',
        'uniq: ()=>_uniq_js__WEBPACK_IMPORTED_MODULE_',
        '"default": ()=>_lodash_default_js__WEBPACK_IMPORTED_MODULE_'
      ];
      
      preservedExports.forEach(exportPattern => {
        expect(content).toContain(exportPattern);
      });
      
      // Check that nulled exports are set to null
      const nulledExports = [
        'add: ()=>null',
        'after: ()=>null',
        'ary: ()=>null'
      ];
      
      nulledExports.forEach(exportPattern => {
        expect(content).toContain(exportPattern);
      });
    }
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
      
      // Verify entry module is set in chunk_characteristics if present
      if (mergedConfig.treeShake['lodash-es'].chunk_characteristics) {
        expect(mergedConfig.treeShake['lodash-es'].chunk_characteristics.entry_module_id).toMatch(/node_modules.*lodash-es.*lodash\.js$/);
      }
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