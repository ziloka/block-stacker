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
const board = Array.from({length: HEIGHT}, ()=> (Array.from({length: WIDTH}, () => "N")));
const active_piece_initial = [];
const active_piece_destination = [];
$("div img").each((i, e) => {
    const name = $(e).attr("alt");
    const y = HEIGHT - Math.floor(i / WIDTH) - 1;
    const x = i % WIDTH;
    if (/(L|J|T|S|Z|I)Tet\.png/.test(name)) { // initial tetromino position
        active_piece_initial.push(` vec2(${x}., ${y}.)`)
    } else {
        switch (name) {
            case "GTet.png": // garbage block
                board[x][y] = "G";
                break;
            case "-Tet.png": // dest tetromino position
                active_piece_destination.push(` vec2(${x}., ${y}.)`)
                break;
            case "Tet.png": // empty block
                break;
            default:
                console.log(`missing case for: ${name}`);
                break;
        }
    }
});
// board.reverse().map((a) => a.reverse())

const format = (arr) => JSON.stringify(arr).replace(/"/g, '').replace(/\[/g, "vec![");

console.log(`board: ${format(board)}`);
console.log(`initial: ${format(active_piece_initial)}`);
console.log(`dest: ${format(active_piece_destination)}`);
