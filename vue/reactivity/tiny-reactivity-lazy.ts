
(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>
    type TargetMap = WeakMap<any, DepsMap>

    const targetMap: TargetMap = new WeakMap()
    let activeEffect: Effect | null = null

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

    function computed<T>(getter: () => T) {
        let value: T
        const c = {
            _dirty: true,
            hasEffect: false,
            get value() {
                track(c, 'value')
                if (!c.hasEffect) {
                    activeEffect = () => {
                        if (!c._dirty) {
                            c._dirty = true
                            trigger(c, 'value')
                        }
                    }
                    value = getter()
                    activeEffect = null
                    c.hasEffect = true
                    c._dirty = false
                }
                else if (c._dirty) {
                    c._dirty = false
                    value = getter()
                }
                return value
            },
        }
        return c
    }

    (() => {
        const a = ref(1)
        const b = computed(() => { console.log("update b"); return a.value * 2 }) // only creates computed value
        const c = computed(() => { console.log("update c"); return b.value * 2 })
        console.log("a", a.value)
        console.log("b", b.value) // b getter is executed here
        console.log("c", c.value) // c getter is executed here
        a.value = 2 // only updates a and sets dirty flag for b and c
        console.log("a", a.value)
        console.log("b", b.value)
        console.log("c", c.value)
    })()

})()
