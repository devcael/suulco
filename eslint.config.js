import js from '@eslint/js'
import vue from 'eslint-plugin-vue'
import tseslint from 'typescript-eslint'
import vueParser from 'vue-eslint-parser'
import globals from 'globals'

export default [
  // Ignorar arquivos e pastas
  {
    ignores: [
      'dist/**',
      'node_modules/**',
      'src-tauri/target/**',
      '*.min.js'
    ]
  },

  // Regras base do JavaScript
  js.configs.recommended,

  // Configurações recomendadas do TypeScript
  ...tseslint.configs.recommended,

  // Configurações recomendadas do Vue 3
  ...vue.configs['flat/recommended'],

  // Configuração principal
  {
    files: ['**/*.{js,mjs,cjs,ts,vue}'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        ecmaVersion: 'latest',
        sourceType: 'module'
      },
      globals: {
        ...globals.browser,
        ...globals.node
      }
    },
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      // Permite variáveis prefixadas com _
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_'
        }
      ],

      // Desativa validação duplicada do TypeScript
      'no-unused-vars': 'off',

      // Opcional: permitir componentes com nome de uma palavra
      'vue/multi-word-component-names': 'off'
    }
  }
]
