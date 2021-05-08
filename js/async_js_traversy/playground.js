console.log("this code is run before the promise is created")

const promise = new Promise((resolve, reject) => {
    console.log("resolve this promise!!")
    resolve("hihi")
})

console.log("this code is run after the promise is created")

promise.then(value => {
    console.log("promise was resolved with:", value)
})
