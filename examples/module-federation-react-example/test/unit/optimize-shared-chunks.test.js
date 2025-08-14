import { describe, it, expect } from 'vitest';
import { mergeUsageData } from '../../scripts/optimize-shared-chunks.js';

describe('optimize-shared-chunks aggregation', () => {
  it('OR-merges export flags across apps and does not merge chunk_characteristics', () => {
    const host = {
      name: 'host',
      data: {
        treeShake: {
          'antd': {
            Button: true,
            Modal: false,
            Table: false,
            chunk_characteristics: {
              entry_module_id: 'host/antd/entry.js',
              chunk_files: ['host-antd.js']
            }
          },
          'lodash-es': {
            debounce: false,
            random: true,
            chunk_characteristics: {
              entry_module_id: 'host/lodash/entry.js',
              chunk_files: ['host-lodash.js']
            }
          }
        }
      }
    };

    const remote = {
      name: 'remote',
      data: {
        treeShake: {
          'antd': {
            Button: false,
            Radio: true,
            Table: true,
            chunk_characteristics: {
              entry_module_id: 'remote/antd/entry.js',
              chunk_files: ['remote-antd.js']
            }
          },
          'lodash-es': {
            debounce: true,
            random: false,
            chunk_characteristics: {
              entry_module_id: 'remote/lodash/entry.js',
              chunk_files: ['remote-lodash.js']
            }
          }
        }
      }
    };

    const { treeShake, metadata } = mergeUsageData([host, remote]);

    // Check metadata sanity
    expect(Array.isArray(metadata.apps)).toBe(true);
    expect(metadata.apps.sort()).toEqual(['host', 'remote']);

    // antd should be OR-merged
    expect(treeShake['antd']).toBeDefined();
    expect(treeShake['antd'].Button).toBe(true); // host=true OR remote=false => true
    expect(treeShake['antd'].Radio).toBe(true);  // remote=true
    expect(treeShake['antd'].Table).toBe(true);  // remote=true OR host=false => true
    expect('chunk_characteristics' in treeShake['antd']).toBe(false);

    // lodash-es should be OR-merged
    expect(treeShake['lodash-es']).toBeDefined();
    expect(treeShake['lodash-es'].debounce).toBe(true); // remote=true
    expect(treeShake['lodash-es'].random).toBe(true);   // host=true
    expect('chunk_characteristics' in treeShake['lodash-es']).toBe(false);
  });
});


