(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>
    type TargetMap = WeakMap<object, DepsMap>

    function track(target: object, key: string) {
        let depsMap = targetMap.get(target)
        if (!depsMap) {
            depsMap = new Map()
            targetMap.set(target, depsMap)
        }
        const dep = depsMap.get(key)
        if (!dep) {
            depsMap.set(key, new Set([effect]))
        } else {
            dep.add(effect)
        }
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

    const product = { price: 5, quantity: 2 }
    let total = 0

    let effect: Effect = () => { total = product.price * product.quantity }

    const targetMap: TargetMap = new WeakMap()

    track(product, 'quantity')
    effect()
    console.log(total)

    product.quantity = 3
    trigger(product, 'quantity')
    console.log(total)
})()
