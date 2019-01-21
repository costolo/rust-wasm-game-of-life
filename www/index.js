import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import TypeIt from "typeit";

const RULES = [
  "- any live cell with fewer than two live neighbors dies",
  "- any live cell with two or three live neighbors lives on to the next generation",
  "- any live cell with more than three live neighbors dies",
  "- any dead cell with exactly three live neighbors becomes a live cell"
];

const typeIt = new TypeIt("#rules", {
  strings: RULES,
  cursorChar: "_",
  breakLines: true,
  speed: 25,
});

typeIt.go();

const CELL_SIZE = 10;
const GRID_COLOR = "#666";
const DEAD_COLOR = "#000000";
const ALIVE_COLOR = "#32CD32";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext("2d");

let animationId = null;

const isPaused = () => animationId === null;

const renderLoop = () => {
  universe.tick();

  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => row * width + column;

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "pause";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "play";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const clearButton = document.getElementById("clear");

clearButton.addEventListener("click", () => {
  pause();
  universe.kill_all_cells();
  drawCells();
});

const randomButton = document.getElementById("random");

randomButton.addEventListener("click", () => {
  pause();
  universe.generate_random_cells();
  drawCells();
});

const gliderInput = () =>
  document.querySelectorAll('input[name="glider"]:checked')[0].value;

canvas.addEventListener("click", e => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  const input = gliderInput();

  if (input === "cell") {
    universe.toggle_cell(row, col);
  } else {
    universe.draw_glider(row, col, input);
  }

  drawCells();
});

drawGrid();
drawCells();
