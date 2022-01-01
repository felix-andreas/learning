(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>
    type TargetMap = WeakMap<any, DepsMap>

    function track(target: any, key: string) {
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

    function trigger(target: any, key: string) {
        const depsMap = targetMap.get(target)
        if (!depsMap) {
            return
        }
        let dep = depsMap.get(key)
        if (dep) {
            dep.forEach(effect => effect())
        }
    }

    const product: any = { price: 5, quantity: 2 }
    let total = 0

    let effect: Effect = () => {
        total = product.price * product.quantity
    }

    const targetMap: TargetMap = new WeakMap()

    track(product, 'quantity')
    effect()
    console.log(total)

    product.quantity = 3
    trigger(product, 'quantity')
    console.log(total)
})()
