/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  build: {
    target: "esnext",
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
  },
});
