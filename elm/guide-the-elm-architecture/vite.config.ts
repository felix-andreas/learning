import { defineConfig } from 'vite'
import Elm from 'vite-plugin-elm'
import UnoCSS from 'unocss/vite'

export default defineConfig({
    plugins: [
        Elm(),
        UnoCSS()
    ]
})
