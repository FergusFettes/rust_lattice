import { memory } from "wasm-lattice-models/wasm_lattice_models_bg";
import { Universe, Cell } from "wasm-lattice-models";
// import * as wasm from "wasm-lattice-models";

const CELL_SIZE = 3; // px
const UNIVERSE_SIZE = 150;
const INITIAL_DENSITY = 0.48;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const LIVE_IF = [3, 4, 6, 7, 8];
const BORN_IF = [3, 6, 7, 8];

// Construct the universe, and get its width and height.
const universe = Universe.new();
universe.set_width(UNIVERSE_SIZE);
universe.set_height(UNIVERSE_SIZE);
universe.initialise_cells(INITIAL_DENSITY);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("lattice-model-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) == mask;
};

const renderLoop = () => {
    debugger;
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const makeRule = (rule) => {
    var out = 0;
    for (let i = 0; i < rule.length; i++) {
        out = out | (1 << rule[i]);
    }
    return out;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

// wasm.greet()
universe.set_rules(makeRule(LIVE_IF), makeRule(BORN_IF));
drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
