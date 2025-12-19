# Rigel

## 参考文档地址

[tauri](https://v2.tauri.app/zh-cn/start/create-project/)

## 基本命令

```shell
# 创建项目
cargo create-tauri-app

# 桌面端
npm run tauri dev

# android端
npm run tauri android dev
```

## Eslint

安装eslint

```shell
npm i --save-dev eslint vue-tsc @antfu/eslint-config jiti
```

[配置文件](./eslint.config.ts)

## StyleLint

安装stylelint

```shell
npm i --save-dev less postcss-html
npm i --save-dev stylelint stylelint-config-recommended-vue stylelint-config-standard stylelint-config-standard-less stylelint-config-standard-vue stylelint-order @stylistic/stylelint-plugin
```

[配置文件](./.stylelintrc.mjs)
