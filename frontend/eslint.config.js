import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import prettierPlugin from "eslint-plugin-prettier";
import typescriptPlugin from "@typescript-eslint/eslint-plugin"; // Import direct du plugin TypeScript
import typescriptParser from "@typescript-eslint/parser"; // Utilisation du parser TypeScript

export default [
  {
    ignores: ["dist"],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
      parser: typescriptParser // Ajout du parser TypeScript
    },
    plugins: {
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
      prettier: prettierPlugin
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true }
      ],
      "prettier/prettier": "error"
    }
  },
  {
    plugins: { "@typescript-eslint": typescriptPlugin }, // Ajout du plugin TypeScript
    rules: typescriptPlugin.configs.recommended.rules // Utilisation des règles recommandées
  }
];
