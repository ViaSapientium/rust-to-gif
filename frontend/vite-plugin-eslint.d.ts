declare module "vite-plugin-eslint" {
  import { Plugin } from "vite";
  const plugin: () => Plugin;
  export default plugin;
}
