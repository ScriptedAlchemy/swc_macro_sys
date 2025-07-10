#!/usr/bin/env node

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { optimize } from "./crates/swc_macro_wasm/pkg/swc_macro_wasm.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const LODASH_CHUNK = path.join(__dirname, "test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
const SHARE_USAGE = path.join(__dirname, "test-cases/rspack-annotated-output/share-usage.json");

async function testLodashOptimization() {
    try {
        console.log("🚀 Testing Lodash Chunk Optimization with SWC Macro Transformer");
        console.log("=" . repeat(60));

        // Read files
        const lodashChunk = fs.readFileSync(LODASH_CHUNK, "utf8");
        const shareUsage = JSON.parse(fs.readFileSync(SHARE_USAGE, "utf8"));
        
        const originalSize = Buffer.byteLength(lodashChunk, "utf8");
        console.log(`\n📖 Original lodash chunk: ${(originalSize / 1024).toFixed(2)} KB`);

        // Get lodash usage data
        const lodashUsage = shareUsage.consume_shared_modules["lodash-es"];
        console.log(`\n📊 Lodash usage from share-usage.json:`);
        console.log(`   Used exports: [${lodashUsage.used_exports.join(", ")}]`);
        console.log(`   Unused exports: ${lodashUsage.unused_exports.length} exports`);

        // Build tree-shaking config
        const lodashConfig = {};
        lodashUsage.used_exports.forEach(exp => lodashConfig[exp] = true);
        lodashUsage.unused_exports.forEach(exp => lodashConfig[exp] = false);

        const config = {
            treeShake: {
                "lodash-es": lodashConfig
            }
        };

        console.log(`\n🌳 Tree-shaking configuration:`);
        console.log(`   Total exports: ${Object.keys(lodashConfig).length}`);
        console.log(`   Enabled: ${lodashUsage.used_exports.length}`);
        console.log(`   Disabled: ${lodashUsage.unused_exports.length}`);

        // Optimize
        console.log(`\n⚡ Running optimization...`);
        const startTime = Date.now();
        const optimized = optimize(lodashChunk, JSON.stringify(config));
        const duration = Date.now() - startTime;

        const optimizedSize = Buffer.byteLength(optimized, "utf8");
        const reduction = originalSize - optimizedSize;
        const reductionPercent = (reduction / originalSize * 100).toFixed(2);

        console.log(`\n✅ Optimization Results:`);
        console.log(`   Duration: ${duration}ms`);
        console.log(`   Original:  ${originalSize.toLocaleString()} bytes (${(originalSize / 1024).toFixed(2)} KB)`);
        console.log(`   Optimized: ${optimizedSize.toLocaleString()} bytes (${(optimizedSize / 1024).toFixed(2)} KB)`);
        console.log(`   Reduction: ${reduction.toLocaleString()} bytes (${(reduction / 1024).toFixed(2)} KB)`);
        console.log(`   **${reductionPercent}% smaller**`);

        // Count modules
        const originalModules = (lodashChunk.match(/node_modules\/lodash-es\//g) || []).length;
        const optimizedModules = (optimized.match(/node_modules\/lodash-es\//g) || []).length;
        
        console.log(`\n📦 Module count:`);
        console.log(`   Original: ${originalModules} modules`);
        console.log(`   Optimized: ${optimizedModules} modules`);
        console.log(`   Removed: ${originalModules - optimizedModules} modules`);

        // Write output
        const outputPath = path.join(__dirname, "test-cases/rspack-annotated-output/lodash-optimized.js");
        fs.writeFileSync(outputPath, optimized);
        console.log(`\n💾 Optimized file saved to: ${outputPath}`);

    } catch (error) {
        console.error("❌ Error:", error);
        process.exit(1);
    }
}

testLodashOptimization();