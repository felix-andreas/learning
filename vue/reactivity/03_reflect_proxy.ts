(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>
    type TargetMap = WeakMap<any, DepsMap>

    const targetMap: TargetMap = new WeakMap()

    function track(target: object, key: string) {
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
        dep.add(effect)
    }

    function trigger(target: object, key: string) {
        const depsMap = targetMap.get(target)
        if (!depsMap) {
            return
        }
        let dep = depsMap.get(key)
        if (dep) {
            dep.forEach(effect)
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

    const product = reactive({ price: 5, quantity: 2 })

    let total = 0

    let effect: Effect = () => { total = product.price * product.quantity }

    effect()
    console.log(total)

    product.quantity = 3
    console.log(total)
})()
