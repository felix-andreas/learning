(() => {
    type Effect = () => void
    type Dep = Set<Effect>
    type DepsMap = Map<string, Dep>

    const track = (key: string) => {
        let dep = depsMap.get(key)
        if (dep === undefined) {
            dep = new Set()
            depsMap.set(key, dep)
        }

        dep.add(effect)
    }

    const trigger = (key: string) => {
        let dep = depsMap.get(key)
        if (dep === undefined) {
            return
        }
        dep.forEach(effect)
    }


    const product = { price: 5, quantity: 2 }
    const depsMap: DepsMap = new Map()

    let total = 0

    let effect = () => { total = product.price * product.quantity }

    track('quantity')
    effect()
    console.log("total", total)

    product.quantity = 3
    trigger('quantity')
    console.log("total", total)
})()
