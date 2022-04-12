
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

    function reactive<T extends object>(target: T) {
        return new Proxy(
            target,
            {
                get(target: T, key: string, receiver) {
                    console.log("Get was called", key)
                    const result = Reflect.get(target, key, receiver)
                    track(target, key)
                    return result
                },
                set(target: T, key: string, value: any, receiver) {
                    console.log("Set was called", key, value)
                    const oldValue = target[key]
                    const result = Reflect.set(target, key, value, receiver)
                    if (result && oldValue != value) {
                        trigger(target, key)
                    }
                    return result
                }
            }
        )
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

    const product = reactive({ price: 5, quantity: 2 })
    const salePrice = ref(0)
    let total = 0

    effect(() => { salePrice.value = 0.9 * product.price })
    effect(() => { total = salePrice.value * product.quantity })

    console.log({ total, salePrice })

    product.quantity = 3
    console.log({ total, salePrice })

    product.price = 10
    console.log({ total, salePrice })
})()
