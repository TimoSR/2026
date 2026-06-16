import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  resolve: {
    alias: {
      "@aztro/widget": fileURLToPath(
        new URL("../../packages/widget/src/index.ts", import.meta.url)
      )
    }
  },
  server: {
    host: "127.0.0.1",
    port: 5173
  },
  build: {
    outDir: "dist"
  }
});
