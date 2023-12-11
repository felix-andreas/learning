import { ref, computed, effect } from "https://unpkg.com/@vue/reactivity@3.3.11/dist/reactivity.esm-browser.prod.js"

(() => {
    // it's possible to chain reactive signals
    console.log("Example 1: Chained signals")
    {
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
    }

    // but this reactive system is susceptible to the diamond problem
    // (this means effects are run more often than necessary)
    // see: https://book.leptos.dev/appendix_reactive_graph.html#solving-the-diamond-problem
    console.log("\nExample 2: Diamond Problem")
    {
        const a = ref("Alice")
        const b = computed(() => { console.log("update b"); return a.value.length }) // immedialty computes b
        const c = computed(() => { console.log("update c"); return a.value.toUpperCase() }) // immedialty computes c
        effect(() => console.log(`run effect: b: ${b.value}, c: ${c.value}`))
        a.value = "Bob"
        a.value = "Tim"
    }
})()
