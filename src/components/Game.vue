<template>
  <h3>Game</h3>

  <div>
    <button v-on:click="startGame()">Start Game</button>
    <button v-on:click="stopGame()">Stop Game</button>
  </div>

  <div>
    <h2 class="gameover"> {{ gameOver }} </h2>

    <div class="boardrow" v-for="row of gameBoard.board">
      <p class="square" v-for="val of row"
         :style="{backgroundColor: color_from_value(val)}"></p>
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

const board_shape = await invoke("get_board_dimensions")

const gameBoard = ref(new Board(board_shape[0], board_shape[1]))
const gameOver = ref("")

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

async function startGame() {
  await invoke("start_game")
}

async function stopGame() {
  await invoke("reset_game")
  gameOver.value = ""
}

</script>

<style scoped>
.boardrow {
  margin: 0;
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.square {
  margin: 1px;
  width: 40px;
  height: 40px;
}

.gameover {
  color: red;
  position: absolute;
  margin-left: auto;
  margin-right: auto;
  left: 0;
  right: 0;
  text-align: center;
}
</style>
