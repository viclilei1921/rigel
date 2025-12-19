/* eslint-env node */
import { antfu } from '@antfu/eslint-config'

export default antfu(
  {
    typescript: true,
    vue: true,
    ignores: ['node_modules', '.gitignore', 'public', 'src-tauri']
  },
  {
    rules: {
      'perfectionist/sort-imports': [
        'warn',
        {
          groups: [
            'vue-type',
            'type',
            ['parent-type', 'sibling-type', 'index-type', 'internal-type'],
            { newlinesBetween: 'always' },
            'builtin',
            'vue',
            'external',
            { newlinesBetween: 'always' },
            '@/hooks',
            { newlinesBetween: 'always' },
            '@/stores',
            { newlinesBetween: 'always' },
            'internal',
            { newlinesBetween: 'always' },
            ['parent', 'sibling', 'index'],
            'side-effect',
            'object',
            'unknown'
          ],
          customGroups: [
            {
              groupName: 'vue-type',
              elementNamePattern: [
                '^vue$',
                '^vue-.+',
                '^@vueuse/.+',
                '^@/store[s]?$',
                '^@/store[s]?/.+',
                '^@/hook[s]?$',
                '^@/hook[s]?/.+'
              ],
              selector: 'type'
            },
            {
              groupName: 'vue',
              elementNamePattern: ['^vue$', '^vue-.+', '^@vueuse/.+']
            },
            {
              groupName: '@/hooks',
              elementNamePattern: ['^@/hook[s]?$', '^@/hook[s]?/.+'],
              selector: 'internal'
            },
            {
              groupName: '@/stores',
              elementNamePattern: ['^@/store[s]?$', '^@/store[s]?/.+'],
              selector: 'internal'
            }
          ],
          newlinesBetween: 'never'
        }
      ],
      // 用于控制对象、数组、函数参数等中的尾随逗号
      'style/comma-dangle': ['warn', 'never'],
      'node/prefer-global/process': 'off',
      'curly': ['error', 'all'],
      'style/brace-style': ['error', '1tbs', { allowSingleLine: false }],
      'style/nonblock-statement-body-position': ['error', 'below'],
      'style/keyword-spacing': [
        'error',
        { overrides: { if: { after: true }, else: { after: true } } }
      ],
      'vue/comma-dangle': ['error', 'never'],
      '@typescript-eslint/consistent-type-definitions': 'off',
      'style/padding-line-between-statements': [
        'error',
        { blankLine: 'always', next: '*', prev: 'import' },
        { blankLine: 'any', next: 'import', prev: 'import' }
      ],
      'style/no-extra-parens': [
        'error',
        'all',
        {
          nestedBinaryExpressions: false,
          conditionalAssign: false,
          ternaryOperandBinaryExpressions: false,
          returnAssign: false,
          nestedConditionalExpressions: false
        }
      ]
    }
  }
)
