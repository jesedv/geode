import { defineConfig } from "vite";
export default defineConfig({
  build: { target: "esnext", outDir: "dist", assetsInlineLimit: 0 },
  optimizeDeps: { exclude: ["./pkg/geode_wasm.js"] },
});
