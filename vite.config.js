import { defineConfig } from 'vite'
import path from 'path'

export default defineConfig({
  base: '/chess/', // Laisser avec github actions
  resolve : {
    alias: {
        '@': path.resolve(__dirname, './common'),
    }
  },
  build: {
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, "index.html"),
        chess: path.resolve(__dirname, "chess/index.html"),
        tit_tac_toe: path.resolve(__dirname, "tic-tac-toe/index.html"),
        heads_tails: path.resolve(__dirname, "heads-tails/index.html"),
      }
    }
  }
});