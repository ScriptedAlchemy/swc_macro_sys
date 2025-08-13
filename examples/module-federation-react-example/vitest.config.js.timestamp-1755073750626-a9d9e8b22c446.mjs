// vitest.config.js
import { defineConfig } from "file:///Users/bytedance/dev/swc_macro_sys/node_modules/.pnpm/vitest@1.6.1_@types+node@20.19.7_@vitest+ui@1.6.1_jsdom@24.1.3_less@4.4.0/node_modules/vitest/dist/config.js";
import path from "path";
import wasm from "file:///Users/bytedance/dev/swc_macro_sys/node_modules/.pnpm/vite-plugin-wasm@3.5.0_vite@5.4.19/node_modules/vite-plugin-wasm/exports/import.mjs";
import topLevelAwait from "file:///Users/bytedance/dev/swc_macro_sys/node_modules/.pnpm/vite-plugin-top-level-await@1.5.0_vite@5.4.19/node_modules/vite-plugin-top-level-await/exports/import.mjs";
var __vite_injected_original_dirname = "/Users/bytedance/dev/swc_macro_sys/examples/module-federation-react-example";
var vitest_config_default = defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: "./test/setup.js",
    include: ["test/**/*.test.js"],
    exclude: ["test/e2e/**"],
    watch: false,
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      exclude: [
        "node_modules/",
        "test/",
        "**/*.config.js",
        "dist/",
        "scripts/"
      ]
    }
  },
  resolve: {
    alias: {
      "@host": path.resolve(__vite_injected_original_dirname, "./host/src"),
      "@remote": path.resolve(__vite_injected_original_dirname, "./remote/src"),
      "@test": path.resolve(__vite_injected_original_dirname, "./test")
    }
  }
});
export {
  vitest_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZXN0LmNvbmZpZy5qcyJdLAogICJzb3VyY2VzQ29udGVudCI6IFsiY29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2Rpcm5hbWUgPSBcIi9Vc2Vycy9ieXRlZGFuY2UvZGV2L3N3Y19tYWNyb19zeXMvZXhhbXBsZXMvbW9kdWxlLWZlZGVyYXRpb24tcmVhY3QtZXhhbXBsZVwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL1VzZXJzL2J5dGVkYW5jZS9kZXYvc3djX21hY3JvX3N5cy9leGFtcGxlcy9tb2R1bGUtZmVkZXJhdGlvbi1yZWFjdC1leGFtcGxlL3ZpdGVzdC5jb25maWcuanNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL1VzZXJzL2J5dGVkYW5jZS9kZXYvc3djX21hY3JvX3N5cy9leGFtcGxlcy9tb2R1bGUtZmVkZXJhdGlvbi1yZWFjdC1leGFtcGxlL3ZpdGVzdC5jb25maWcuanNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlc3QvY29uZmlnJztcbmltcG9ydCBwYXRoIGZyb20gJ3BhdGgnO1xuaW1wb3J0IHdhc20gZnJvbSAndml0ZS1wbHVnaW4td2FzbSc7XG5pbXBvcnQgdG9wTGV2ZWxBd2FpdCBmcm9tICd2aXRlLXBsdWdpbi10b3AtbGV2ZWwtYXdhaXQnO1xuXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xuICBwbHVnaW5zOiBbXG4gICAgd2FzbSgpLFxuICAgIHRvcExldmVsQXdhaXQoKVxuICBdLFxuICB0ZXN0OiB7XG4gICAgZ2xvYmFsczogdHJ1ZSxcbiAgICBlbnZpcm9ubWVudDogJ2pzZG9tJyxcbiAgICBzZXR1cEZpbGVzOiAnLi90ZXN0L3NldHVwLmpzJyxcbiAgICBpbmNsdWRlOiBbJ3Rlc3QvKiovKi50ZXN0LmpzJ10sXG4gICAgZXhjbHVkZTogWyd0ZXN0L2UyZS8qKiddLFxuICAgIHdhdGNoOiBmYWxzZSxcbiAgICBjb3ZlcmFnZToge1xuICAgICAgcHJvdmlkZXI6ICd2OCcsXG4gICAgICByZXBvcnRlcjogWyd0ZXh0JywgJ2pzb24nLCAnaHRtbCddLFxuICAgICAgZXhjbHVkZTogW1xuICAgICAgICAnbm9kZV9tb2R1bGVzLycsXG4gICAgICAgICd0ZXN0LycsXG4gICAgICAgICcqKi8qLmNvbmZpZy5qcycsXG4gICAgICAgICdkaXN0LycsXG4gICAgICAgICdzY3JpcHRzLydcbiAgICAgIF1cbiAgICB9XG4gIH0sXG4gIHJlc29sdmU6IHtcbiAgICBhbGlhczoge1xuICAgICAgJ0Bob3N0JzogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgJy4vaG9zdC9zcmMnKSxcbiAgICAgICdAcmVtb3RlJzogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgJy4vcmVtb3RlL3NyYycpLFxuICAgICAgJ0B0ZXN0JzogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgJy4vdGVzdCcpXG4gICAgfVxuICB9XG59KTsiXSwKICAibWFwcGluZ3MiOiAiO0FBQXVaLFNBQVMsb0JBQW9CO0FBQ3BiLE9BQU8sVUFBVTtBQUNqQixPQUFPLFVBQVU7QUFDakIsT0FBTyxtQkFBbUI7QUFIMUIsSUFBTSxtQ0FBbUM7QUFLekMsSUFBTyx3QkFBUSxhQUFhO0FBQUEsRUFDMUIsU0FBUztBQUFBLElBQ1AsS0FBSztBQUFBLElBQ0wsY0FBYztBQUFBLEVBQ2hCO0FBQUEsRUFDQSxNQUFNO0FBQUEsSUFDSixTQUFTO0FBQUEsSUFDVCxhQUFhO0FBQUEsSUFDYixZQUFZO0FBQUEsSUFDWixTQUFTLENBQUMsbUJBQW1CO0FBQUEsSUFDN0IsU0FBUyxDQUFDLGFBQWE7QUFBQSxJQUN2QixPQUFPO0FBQUEsSUFDUCxVQUFVO0FBQUEsTUFDUixVQUFVO0FBQUEsTUFDVixVQUFVLENBQUMsUUFBUSxRQUFRLE1BQU07QUFBQSxNQUNqQyxTQUFTO0FBQUEsUUFDUDtBQUFBLFFBQ0E7QUFBQSxRQUNBO0FBQUEsUUFDQTtBQUFBLFFBQ0E7QUFBQSxNQUNGO0FBQUEsSUFDRjtBQUFBLEVBQ0Y7QUFBQSxFQUNBLFNBQVM7QUFBQSxJQUNQLE9BQU87QUFBQSxNQUNMLFNBQVMsS0FBSyxRQUFRLGtDQUFXLFlBQVk7QUFBQSxNQUM3QyxXQUFXLEtBQUssUUFBUSxrQ0FBVyxjQUFjO0FBQUEsTUFDakQsU0FBUyxLQUFLLFFBQVEsa0NBQVcsUUFBUTtBQUFBLElBQzNDO0FBQUEsRUFDRjtBQUNGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
