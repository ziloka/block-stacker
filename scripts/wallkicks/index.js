// Run program via node https://deno.com
// node index.js

// given HTML table of tetrominos convert it in array representation
// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration

const fs = require('fs');
const cheerio = require('cheerio');

const $ = cheerio.load(fs.readFileSync("input.txt"));

const HEIGHT = 7;
const WIDTH = 5;

// 35 elements in total
const matrix = Array.from({length: HEIGHT}, ()=> (Array.from({length: WIDTH}, () => 0)));
console.log(`matrix has ${matrix.length} rows`);
console.log(`matrix has ${matrix[0].length} columns`);
$("div img").each((i, e) => {
    const name = $(e).attr("alt");
    const row = Math.floor(i / WIDTH);
    const column = i % WIDTH;
    // console.log(`${i} (${row}, ${column})`);
    switch (name) {
        case "LTet.png": // active tetromino
            matrix[row][column] = 1;
            break;
        case "GTet.png": // garbage block
            matrix[row][column] = 2;
            break;
        case "-Tet.png": // where dest tetromino goes
            matrix[row][column] = 3;
            break;
        case "Tet.png": // empty block
            break;
        default:
            console.log(`missing case for: ${name}`);
            break;
    }
});

console.log(matrix);