global.fetch = require("node-fetch")

async function fetchUsers() {
    const response = await fetch("https://jsonplaceholder.typicode.com/users")
    const data = await response.json()
    console.log(data)
}

fetchUsers()
