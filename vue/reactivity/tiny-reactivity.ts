
(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>
    type TargetMap = WeakMap<any, DepsMap>

    const targetMap: TargetMap = new WeakMap()
    let activeEffect: Effect | null = null

    function effect(eff: Effect) {
        activeEffect = eff
        activeEffect()
        activeEffect = null
    }

    function track(target: object, key: string) {
        if (activeEffect) {
            let depsMap = targetMap.get(target)
            if (!depsMap) {
                depsMap = new Map()
                targetMap.set(target, depsMap)
            }
            let dep = depsMap.get(key)
            if (!dep) {
                dep = new Set()
                depsMap.set(key, dep)
            }
            dep.add(activeEffect)
        }
    }

    function trigger(target: object, key: string) {
        const depsMap = targetMap.get(target)
        if (!depsMap) {
            return
        }
        const dep = depsMap.get(key)
        if (dep) {
            dep.forEach(effect => effect())
        }
    }

    function ref(raw: any) {
        const r = {
            get value() {
                track(r, 'value')
                return raw
            },
            set value(newValue) {
                raw = newValue
                trigger(r, 'value')
            }
        }
        return r
    }

    function computed(getter: () => any) {
        const r = ref(undefined)
        effect(() => r.value = getter())
        return r
    }

    const foo = ref(1)
    const bar = computed(() => {console.log("update bar"); return 2 * foo.value})

    console.log("before", foo.value)

    foo.value = 2 // does trigger update of bar right away, but it shouldn't :/
    foo.value = 3
    foo.value = 4

    console.log("after", foo.value)
    console.log("after", bar.value)
})()
