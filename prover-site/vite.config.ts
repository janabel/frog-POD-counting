import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [react(), wasm(), nodePolyfills()],
  server: {
    hmr: true,
  },
  resolve: {
    alias: {
      buffer: "buffer",
    },
  },
  build: {
    target: "esnext",
  },
});
