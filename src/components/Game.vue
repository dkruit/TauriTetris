<template>
  <h3>Game</h3>

  <div>
    <button v-on:click="drawBlock()">Draw block</button>
  </div>

  <div class="boardrow" v-for="row of gameBoard.board">
      <p class="square" v-for="val of row"
         :style="{backgroundColor: color_from_value(val)}"></p>
  </div>


</template>

<script setup lang="ts">
import {ref} from "vue"
import { Board, Block } from "../game"

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
    case "":
      return "firebrick"

  }
}

const board_rows = 7
const board_columns = 7

const gameBoard = ref(new Board(board_rows, board_columns))

const block = new Block([[0, 0], [0, 1], [1, 0], [1, 1]], "S")
gameBoard.value.drawBlock(block)

function drawBlock() {
  gameBoard.value.drawBlock(new Block([[6,0], [6,1]], "O"))
  console.log("DRAWING BLOCK")
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
</style>
