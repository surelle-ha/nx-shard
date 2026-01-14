// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ui: {
    theme: {
      colors: [
        'primary',
        'secondary',
        'tertiary',
        'info',
        'success',
        'warning',
        'error'
      ]
    }
  },

  compatibilityDate: '2025-01-15',

  devtools: { enabled: true },

  ssr: false,

  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },

  css: ['~/assets/css/main.css'],

  routeRules: {
    '/': { prerender: true }
  },

  srcDir: "src-client",

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    }
  },

  modules: ['@pinia/nuxt', '@vueuse/nuxt', '@nuxt/ui', 'nuxt-charts']
})