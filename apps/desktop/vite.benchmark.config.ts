import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  build: {
    outDir: "benchmark-dist",
    emptyOutDir: true,
    target: "es2022",
    rollupOptions: {
      input: "benchmark.html",
    },
  },
});
