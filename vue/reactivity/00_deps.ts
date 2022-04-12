(() => {
    let price = 5
    let quantity = 2
    let total = 0

    const dep = new Set<() => void>()
    const effect = () => { total = price * quantity }
    const track = () => { dep.add(effect) }
    const trigger = () => { dep.forEach(effect) }

    track()
    trigger()
    console.log(total);

    quantity = 3
    trigger()
    console.log(total);
})()
