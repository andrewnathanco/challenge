import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import basicSsl from "@vitejs/plugin-basic-ssl";

export default defineConfig({
  base: process.env.BASE_PATH || "/",
  plugins: [solidPlugin(), basicSsl()],
  server: {
    port: 3000,
  },
  build: {
    target: "esnext",
  },
});
