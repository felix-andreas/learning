
(() => {
    type Effect = () => void
    type Getter = () => any
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


    function track(target: any, key: string) {
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

    function trigger(target: any, key: string) {
        const depsMap = targetMap.get(target)
        if (!depsMap) {
            return
        }
        const dep = depsMap.get(key)
        if (dep) {
            dep.forEach(effect => effect())
        }
    }


    function reactive(target: any) {
        const handler: ProxyHandler<any> = {
            get(target: any, key: string, receiver) {
                console.log("Get was called", key)
                const result = Reflect.get(target, key, receiver)
                track(target, key)
                return result
            },
            set(target: any, key: string, value: any, receiver) {
                console.log("Set was called", key, value)
                const oldValue = target[key]
                const result = Reflect.set(target, key, value, receiver)
                if (result && oldValue != value) {
                    trigger(target, key)
                }
                return result
            }
        }
        return new Proxy(target, handler)
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

    function computed(getter: Getter) {
        const result = ref(undefined)
        effect(() => result.value = getter())
        return result
    }

    const product = reactive({ price: 5, quantity: 2 })
    const salePrice = computed(() => 0.9 * product.price)
    const total = computed(() => salePrice.value * product.quantity)

    console.log("total:", total.value, "salePrice:", salePrice.value)

    product.quantity = 3
    console.log("total:", total.value, "salePrice:", salePrice.value)

    product.price = 10
    console.log("total:", total.value, "salePrice:", salePrice.value)
})()
