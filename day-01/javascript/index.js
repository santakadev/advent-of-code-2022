fs = require('fs')
const readline = require('readline')


rl = readline.createInterface({
    input: fs.createReadStream('./input.txt'),
})

const allSums = [0]
let index = 0

rl.on('line', line => {
    if (line === "") {
        index++
        allSums[index] = 0
    } else {
        allSums[index] += parseInt(line)
    }
})

rl.on('close', () => {
    console.log(Math.max(...allSums))
})
