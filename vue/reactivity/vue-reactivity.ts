import { ref, computed } from "https://unpkg.com/@vue/reactivity@3.2.32/dist/reactivity.esm-browser.prod.js"

(() => {
    const foo = ref(1)
    const bar = computed(() => {console.log("update bar"); return 2 * foo.value})

    console.log("before", foo.value)

    foo.value = 2 // does not trigger update of bar right away
    foo.value = 3
    foo.value = 4

    console.log("after", foo.value)
    console.log("after", bar.value) // bar get's updated here
})()
