import { memory } from 'gol/gol_bg';
import { Universe, Cell } from "gol";

const CELL_SIZE = 5;
const GRID_COLOUR = "#CCCCCC";
const DEAD_COLOUR = "#FFFFFF";
const ALIVE_COLOUR = "#000000";

const u = Universe.new();
const w = u.width();
const h = u.height();

let animationId = null;

const playPausebutton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
}

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const getIndex = (row, column) => {
    return row * w+ column;
}

const canvas = document.getElementById("canvas");
canvas.height = (CELL_SIZE + 1) * h + 1;
canvas.width = (CELL_SIZE + 1) * w + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    u.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
}

const isPaused = () => {
    return animationId === null;
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOUR;

    // Vertical lines
    for (let i = 0; i <= w; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * h + 1);
    }

    // Horizontal lines
    for (let j = 0; j <= h; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * w+ 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = u.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, w* h);

    ctx.beginPath();

    for (let row = 0; row < h; row++) {
        for (let col = 0; col < w; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOUR
                : ALIVE_COLOUR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

drawGrid();
drawCells();

play();
