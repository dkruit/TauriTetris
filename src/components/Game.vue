<template>

  <div class="flex-container">

    <HelpModal v-model:visible="showHelp"></HelpModal>

    <div class="game-menu">

      <div class="boardrow" v-for="row of nextTetrominoBoard.board">
        <p class="square" v-for="val of row"
           :style="{backgroundColor: color_from_value(val), height: squareSize, width: squareSize}"></p>
      </div>

      <div>
        <h2>TauriTetris</h2>
        <button v-on:click="startGame()">Start Game</button>
        <button v-on:click="stopGame()">Reset Game</button>
        <button v-on:click="showHelp = true">Help</button>

        <p>Score: {{ score }}</p>
        <p>Level: {{ level }}</p>
      </div>

      <div class="highscores">
        <h3>HIGHSCORES</h3>
        <p v-for="(score, pos) of highScores"> {{ pos+1 }}: {{ score }}</p>
      </div>
    </div>

    <div class="game-board">
        <div>
          <h2 class="gameover"> {{ gameOver }} </h2>
          <h1 class="score-increase"> {{ scoreIncrease }} </h1>

          <div class="boardrow" v-for="row of gameBoard.board">
            <p class="square" v-for="val of row"
               :style="{backgroundColor: color_from_value(val), height: squareSize, width: squareSize}"></p>
          </div>
        </div>
    </div>

  </div>


</template>

<script setup lang="ts">
import {ref} from "vue"
import { Board, Tetromino } from "../game"
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";
import HelpModal from "./HelpModal.vue";

function color_from_value(value: string): string {
  let color: string = ""
  switch (value) {
    case "_":
      color = "silver";
      break;
    case "I":
      color =  "aqua";
      break;
    case "J":
      color =  "blue";
      break;
    case "L":
      color =  "darkorange";
      break;
    case "O":
      color =  "gold";
      break;
    case "S":
      color =  "green";
      break;
    case "T":
      color =  "darkviolet";
      break;
    case "Z":
      color =  "firebrick";
      break;
  }
  return color
}

// Declare board references
const board_shape: [number, number] = await invoke("get_board_dimensions")
const gameBoard = ref<Board>(new Board(board_shape[0], board_shape[1]))
const nextTetrominoBoard = ref<Board>(new Board(4, 4))
const gameOver = ref<string>("")
const squareSize = ref<string>(`${90/board_shape[0]}vh`)

const score = ref<number>(0)
const scoreIncrease = ref<string>("")
const level = ref<number>(0)
const highScores = ref<number[]>([])

const showHelp = ref<boolean>(false)

interface TetrominoPayload {
  occupied_positions: [number, number][],
  name: string
}
// Listen for game updates
listen<TetrominoPayload>("current_tetromino", (event) => {
  let tetromino = new Tetromino(event.payload.occupied_positions, event.payload.name)
  gameBoard.value.setTetromino(tetromino)
})

// Listen for game updates
listen<TetrominoPayload>("next_tetromino", (event) => {
  console.log("Reveived next tetromino")
  let tetromino = new Tetromino(event.payload.occupied_positions, event.payload.name)
  nextTetrominoBoard.value.setTetromino(tetromino)
})

listen<{ board: string[][] }>("board", (event) => {
  console.log("Received board update.")
  gameBoard.value.setBoard(event.payload.board)

})

listen("game_over", () => {
  console.log("GAME OVER")
  gameOver.value = "GAME OVER"
  updateHigScores(score.value)
})

listen<{ value: number }>("score", (event) => {
  console.log("Updated score")
  score.value = event.payload.value
})

listen<{ value: number }>("score_increase", (event) => {
  showScoreIncrease(event.payload.value)
})

listen<{ value: number }>("level", (event) => {
  console.log("Updated level")
  level.value = event.payload.value
})

// Set up responding to key presses
document.addEventListener('keydown', (event: KeyboardEvent) => {
  if (event.key === 'ArrowDown') {
    process_command("down")
  }
  if (event.key === 'ArrowLeft') {
    process_command("left")
  }
  if (event.key === 'ArrowRight') {
    process_command("right");
  }
  if (event.key === 'Z' || event.key === 'z') {
    process_command("counter-clockwise");
  }
  if (event.key === 'X' || event.key === 'x') {
    process_command("clockwise");
  }
  if (event.key === 'Spacebar' || event.key === ' ') {
    event.preventDefault();
    process_command("hard-drop")
  }
});

async function process_command(command: string) {
  console.log(command)
  let result = await invoke("process_command", {"command": command});
  console.log(result)
}

async function showScoreIncrease(value: number) {
  scoreIncrease.value = `+${value}`
  await new Promise(resolve => setTimeout(resolve, 800))
  scoreIncrease.value = ""
}

async function updateHigScores(new_score: number) {
  if (highScores.value.length < 3) {
    highScores.value.push(new_score)
  }
  else if (new_score > highScores.value[2]) {
    highScores.value[2] = (new_score)
  }
  highScores.value.sort().reverse()
}

// Commands to start and stop the game
async function startGame() {
  await invoke("start_game")
}

async function stopGame() {
  await invoke("reset_game")
  gameOver.value = ""
}

</script>

<style scoped>
.flex-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.game-menu {
  width: 30%;
  max-width: 300px;
  padding: 55px;
  float: right;
  color: darkgreen;
  font-weight: bold;
  background-color: lightpink;
  border-radius: 15px;
  display: flex;
  flex-direction: column;
}

.highscores {
  flex-grow: 1;
  margin-top: 5vh;
  padding-top: 10px;
  border-radius: 10pt;
  color: deeppink;
  background-color: lightgreen;
}

button {
  color: deeppink;
  font-weight: bold;
  background-color: lightgreen;
  }
button:active {
    background-color: #0f0f0f69;
  }

.game-board {
  width: 50%;
  max-width: 65vh;
  position: relative;
  float: left;
}

.boardrow {
  margin: 0;
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.square {
  margin: 1px;
}

.gameover {
  color: red;
  background-color: #2f2f2f;
  border-radius: 8px;
  padding: 5px 10px;
  position: absolute;
  transform: translate(-50%, -50%);
  top: 50%;
  left: 50%;
  text-align: center;
}

.score-increase {
  color: deeppink;
  text-shadow: darkblue 3px 3px 2px;
  font-weight: bolder;
  position: absolute;
  transform: translate(-50%, -50%);
  top: 50%;
  left: 50%;
  text-align: center;
}

.gameover:empty {
    display: none;
}
</style>
