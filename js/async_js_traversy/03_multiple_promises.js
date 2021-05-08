global.fetch = require("node-fetch")
const promise1 = Promise.resolve("Hello World")
const promise2 = 10;
const promise3 = new Promise((resolve, reject) => {
    setTimeout(resolve, 1000, "Goodybye")
})
const promise4 = fetch("https://jsonplaceholder.typicode.com/posts/2")
    .then(response => response.json())
Promise.all([promise1, promise2, promise3, promise4]).then(
    values => console.log(values)
)
