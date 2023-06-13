import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers';
import * as path from 'path';
import { createHtmlPlugin } from 'vite-plugin-html';
import VueI18n from '@intlify/unplugin-vue-i18n/vite';
import Unocss from 'unocss/vite';

// https://vitejs.dev/config/
export default defineConfig({
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
        },
    },
    css: {
        preprocessorOptions: {
            scss: {
                additionalData: `@use "@/assets/styles/element/index.scss" as *;`,
            },
        },
    },
    plugins: [
        vue(),
        createHtmlPlugin({
            minify: true,
            /**
             * Data that needs to be injected into the index.html ejs template
             */
            inject: {
                data: {
                    title: 'vue-ts-starter',
                },
            },
        }),
        AutoImport({
            imports: [
                'vue',
                'vue-router',
                'vue-i18n',
                'vue/macros',
                '@vueuse/head',
                '@vueuse/core',
            ],
            dts: 'src/auto-imports.d.ts',
            resolvers: [ElementPlusResolver()],
            vueTemplate: true,
        }),
        Components({
            resolvers: [ElementPlusResolver()],
            dts: 'src/components.d.ts',
        }),

        // https://github.com/antfu/unocss
        // see unocss.config.ts for config
        Unocss(),

        // https://github.com/intlify/bundle-tools/tree/main/packages/unplugin-vue-i18n
        VueI18n({
            runtimeOnly: true,
            compositionOnly: true,
            /* eslint-disable-next-line @typescript-eslint/ban-ts-comment */
            // @ts-ignore
            strictMessage: false,
            fullInstall: true,
            include: [path.resolve(__dirname, 'locales/**')],
        }),
    ],
    server: {
        port: 8080,
        hmr: {
            host: '127.0.0.1',
            port: 8080,
        },
    },
});
