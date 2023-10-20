// Run program via deno https://deno.com
// deno run --allow-read index.js

// given HTML table of tetrominos convert it in array representation
// https://harddrop.com/wiki/SRS#Wall_Kick_Illustration
import * as cheerio from "https://esm.sh/cheerio@0.22.0";
const $ = cheerio.load(await Deno.readTextFile("input.txt"));

// {
//     const $ = cheerio.load(`<body>
// <table>
// <tr><td>This is first row.</td></tr>
// <tr><td>This is second row.</td></tr>
// <tr><td>This is third row.</td></tr>
// <tr><td>This is fourth row.</td></tr>
// </table>
// </body>`);
//     $("tr:nth-last-child(2n)").toArray().map((e) => $(e).find("tr td").each((_, e) => {
//         console.log($(e).text());
//     }));
// }


function generate_test_case(table, init_rot_indx, dest_rot_index) {
    const HEIGHT = 7;
    const WIDTH = 5;

    // 35 elements in total
    const board = Array.from({length: HEIGHT}, () => (Array.from({length: WIDTH}, () => "N")));
    const active_piece_initial = [];
    const active_piece_destination = [];
    // parse as if bottom left is (0, 0)
    // increments left-right, bottom-up
    $(table).find("div img").each((i, e) => {
        const name = $(e).attr("alt");
        // "relative position"
        const y = Math.floor(i / WIDTH);
        const x = i % WIDTH;
        
        // position in array
        const row = HEIGHT - y - 1;
        const column = x;

        // (6, 0), (6, 1) (6, 2), ...
        // (5, 7), (5, 6) (5, 5), ...
        // console.log(`x: ${x}, y: ${y} | column: ${column} row: ${row}`);
        
        if (/(L|J|T|S|Z|I)Tet\.png/.test(name)) { // initial tetromino position
            // 1, 3
            active_piece_initial.push(` vec2(${column}., ${row}.)`)
        } else {
            switch (name) {
                case "GTet.png": // garbage block
                    // console.log(`${x} ${y}`);
                    board[row][column] = "G";
                    break;
                case "-Tet.png": // dest tetromino position
                    active_piece_destination.push(` vec2(${column}., ${row}.)`)
                    break;
                case "Tet.png": // empty block
                    break;
                default:
                    console.log(`missing case for: ${name}`);
                    break;
            }
        }
    });

    const format = (arr) => JSON.stringify(arr).replace(/"/g, '').replace(/\[/g, "vec![");

    console.log(`// ${init_rot_indx}->${dest_rot_index}`);
    console.log(`generate(${format(board)}, TETROMINO_TYPE, ${format(active_piece_initial)}, ${init_rot_indx}, ${format(active_piece_destination)})\n`);
}

// document.querySelector("table[style=\"text-align:center;\"]").querySelector("tr[align=\"center\"]").querySelector("th").textContent 
// document.querySelector("table[style=\"text-align:center;\"]").querySelector("tr[align=\"center\"]").querySelectorAll(":nth-last-child(-n+2 of td[width=\"74\"])");
$("table[style=\"text-align:center;\"]").each((i, tetrominoTests) => {
    if ([1, 3].includes(i)) return;
    $(tetrominoTests).find("tr[align=\"center\"]").each((_, row) => {
        const [init_rot_indx, dest_rot_indx] = $(row).find("th").text().trim().split("⇒");
        $(row).find("td[width=\"74\"]:nth-last-child(-n+2)").each((_, table) => {
            generate_test_case(table, init_rot_indx, dest_rot_indx);
        });
    });
});