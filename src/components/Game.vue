<template>

  <div class="flex-container">
    <div class="game-menu">
      <div>
        <h3>Game</h3>
        <button v-on:click="startGame()">Start Game</button>
        <button v-on:click="stopGame()">Reset Game</button>
        <p>Score: {{ score }}</p>
        <p>Level: {{ level }}</p>
      </div>

    </div>

    <div class="game-board">
        <div>
          <h2 class="gameover"> {{ gameOver }} </h2>

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
import { Board, Block } from "../game"
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/tauri";

function color_from_value(value: string): string {
  switch (value) {
    case "_":
      return "silver"
    case "I":
      return "aqua"
    case "J":
      return "blue"
    case "L":
      return "darkorange"
    case "O":
      return "gold"
    case "S":
      return "green"
    case "T":
      return "darkviolet"
    case "Z":
      return "firebrick"

  }
}

// Declare board references
const board_shape = await invoke("get_board_dimensions")
const gameBoard = ref(new Board(board_shape[0], board_shape[1]))
const gameOver = ref("")
const squareSize = ref(`${90/board_shape[0]}vh`)

const score = ref(0)
const level = ref(0)

// Listen for game updates
listen("tick", (event) => {
  let block = new Block(event.payload.occupied_positions, event.payload.name)
  gameBoard.value.setBlock(block)
})

listen("board", (event) => {
  console.log("Received board update.")
  gameBoard.value.setBoard(event.payload.board)

})

listen("game_over", (event) => {
  console.log("GAME OVER")
  gameOver.value = "GAME OVER"
})

listen("score", (event) => {
  console.log("Updated score")
  score.value = event.payload.value
})

listen("level", (event) => {
  console.log("Updated level")
  level.value = event.payload.value
})

// Set up responding to key presses
document.addEventListener('keydown', (event: KeyboardEvent) => {
  if (event.key === 'ArrowDown' || event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
    arrow_click(event.key);
  }
  if (event.key === 'Z' || event.key === 'z') {
    rotate("counter-clockwise");
  }
  if (event.key === 'X' || event.key === 'x') {
    rotate("clockwise");
  }
  if (event.key === 'Spacebar' || event.key === ' ') {
    event.preventDefault();
    invoke("process_spacebar");
  }
});

async function arrow_click(key: string) {
  console.log(key)
  let result = await invoke("process_arrow_key", {"key": key});
  console.log(result)
}

async function rotate(direction: string) {
  console.log(direction)
  let result = await invoke("process_rotation", {"direction": direction});
  console.log(result)
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
  float: right;
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

.gameover:empty {
    display: none;
}
</style>
