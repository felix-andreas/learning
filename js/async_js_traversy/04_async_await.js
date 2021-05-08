const humans = [
    { name: "emanuel", age: 26 },
    { name: "leo", age: 27 }
]

function getHumans() {
    console.log("getHumans request")
    setTimeout(() => {
        console.log("getHumans: response")
        console.log(Array.from(humans, h => h.name).join(" - "))
    }, 1000)
}

function createHuman(human) {
    return new Promise((resolve, reject) => {
        console.log("createHuman request")
        setTimeout(() => {
            console.log("createHuman response")
            humans.push(human)
            const success = true
            if (success) {
                resolve()
            } else {
                reject("error!!!")
            }
            resolve()
        }, 1000)
    })
}

async function init() {
    await createHuman({ name: "felix", age: 25 })
    getHumans()
}

init()
