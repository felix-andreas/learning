import { ref, computed } from "https://unpkg.com/@vue/reactivity@3.2.32/dist/reactivity.esm-browser.prod.js"

(() => {
    const a = ref(1)
    const b = computed(() => { console.log("update b"); return a.value * 2 })
    const c = computed(() => { console.log("update c"); return b.value * 2 })
    console.log("a", a.value)
    console.log("b", b.value)
    console.log("c", c.value)
    a.value = 2
    console.log("a", a.value)
    console.log("b", b.value)
    console.log("c", c.value)
})()
