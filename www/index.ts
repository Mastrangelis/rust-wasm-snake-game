import init, { GameStatus, InitOutput, World } from "snake_game";
import { randomRangeIdx } from "./utils/rand";

const CELL_SIZE = 40; // px
const WORLD_WIDTH = 16; // cells
const ARROW_KEY_OPTIONS = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

const drawReward = (ctx: CanvasRenderingContext2D, world: World): void => {
    const rewardCellIdx = world.reward_cell();
    const col = rewardCellIdx % WORLD_WIDTH;

    const row = Math.floor(rewardCellIdx / WORLD_WIDTH);

    ctx.beginPath();
    ctx.fillStyle = "red";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
};

const drawWorld = (ctx: CanvasRenderingContext2D): void => {
    ctx.beginPath();
    for (let i = 0; i < WORLD_WIDTH + 1; i++) {
        ctx.moveTo(i * CELL_SIZE, 0);
        ctx.lineTo(CELL_SIZE * i, WORLD_WIDTH * CELL_SIZE);
    }

    for (let j = 0; j < WORLD_WIDTH + 1; j++) {
        ctx.moveTo(0, CELL_SIZE * j);
        ctx.lineTo(WORLD_WIDTH * CELL_SIZE, CELL_SIZE * j);
    }
    ctx.stroke();
};

const drawSnake = (
    world: World,
    wasmMemory: WebAssembly.Memory,
    ctx: CanvasRenderingContext2D
): void => {
    const snakeCells = new Uint32Array(
        wasmMemory.buffer,
        world.snake_cells(),
        world.snake_length()
    );

    const renderSnakeCell = (cellIdx: number, index: number) => {
        const col = cellIdx % WORLD_WIDTH;
        const row = Math.floor(cellIdx / WORLD_WIDTH);

        // Snake color
        ctx.fillStyle = index === snakeCells.length - 1 ? "green" : "black";

        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    };

    snakeCells.slice().reverse().forEach(renderSnakeCell);

    ctx.stroke();
};

function drawGameStatus(
    world: World,
    gameStatus: HTMLElement,
    points: HTMLElement
) {
    gameStatus.textContent = world.game_status_text();
    points.textContent = world.points().toString();
}

function paint(
    world: World,
    wasmMemory: WebAssembly.Memory,
    ctx: CanvasRenderingContext2D,
    gameStatus: HTMLElement,
    points: HTMLElement,
    gameControlBtn: HTMLElement
) {
    drawWorld(ctx);
    drawSnake(world, wasmMemory, ctx);
    drawReward(ctx, world);
    drawGameStatus(world, gameStatus, points);
}

function play(
    world: World,
    wasmMemory: WebAssembly.Memory,
    ctx: CanvasRenderingContext2D,
    canvas: HTMLCanvasElement,
    gameStatus: HTMLElement,
    points: HTMLElement,
    gameControlBtn: HTMLElement
) {
    const status = world.game_status();

    if (status == GameStatus.Won || status == GameStatus.Lost) {
        gameControlBtn.textContent = "Re-Play";
        return;
    }

    setTimeout(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        world.step();
        paint(world, wasmMemory, ctx, gameStatus, points, gameControlBtn);
        requestAnimationFrame(() =>
            play(
                world,
                wasmMemory,
                ctx,
                canvas,
                gameStatus,
                points,
                gameControlBtn
            )
        );
    }, 150);
}

const registerEvents = (
    world: World,
    canvas: HTMLCanvasElement,
    wasmMemory: WebAssembly.Memory,
    ctx: CanvasRenderingContext2D,
    gameStatus: HTMLElement,
    points: HTMLElement,
    gameControlBtn: HTMLElement
): void => {
    gameControlBtn.addEventListener("click", (_) => {
        const status = world.game_status();

        if (status === undefined) {
            world.start_game();
            play(
                world,
                wasmMemory,
                ctx,
                canvas,
                gameStatus,
                points,
                gameControlBtn
            );
        } else {
            location.reload();
        }
    });

    document.addEventListener("keydown", (event) => {
        if (!ARROW_KEY_OPTIONS.includes(event.key)) return;
        world.set_snake_direction(event.key);
    });
};

init().then((wasm: InitOutput) => {
    const world = World.new(
        WORLD_WIDTH,
        randomRangeIdx(0, Math.pow(WORLD_WIDTH, 2) - 1)
    );

    const canvas = <HTMLCanvasElement>(
        document.getElementById("snake-game-canvas")
    );
    const ctx = canvas.getContext("2d");
    const gameStatus = document.getElementById("game-status");
    const points = document.getElementById("points");
    const gameControlBtn = document.getElementById("game-control-btn");

    canvas.height = CELL_SIZE * WORLD_WIDTH;
    canvas.width = CELL_SIZE * WORLD_WIDTH;

    registerEvents(
        world,
        canvas,
        wasm.memory,
        ctx,
        gameStatus,
        points,
        gameControlBtn
    );

    paint(world, wasm.memory, ctx, gameStatus, points, gameControlBtn);
});
