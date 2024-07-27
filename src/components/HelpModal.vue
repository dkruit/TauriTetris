<template>
  <div class="help-modal" v-if="visible">
    <h3 style="text-align: center">Help</h3>

    <div>
      Click the "Start Game" button to start playing. Use the "Reset Game" to reset the game during or after playing.
      The objective is to fill the rows of the board with the tetrominos which fall down, Clearing rows gives points,
      clearing multiple rows at the same time gives more points!

      <h4>Controls</h4>
      <p><span style="font-weight: bold">&#8592; :</span> Move the tetromino left.</p>
      <p><span style="font-weight: bold">&#8594; :</span> Move the tetromino right.</p>
      <p><span style="font-weight: bold">&#8595; :</span> Move the tetromino down.</p>
      <p><span style="font-weight: bold">z :</span> Rotate the tetromino counter-clockwise.</p>
      <p><span style="font-weight: bold">x :</span> Rotate the tetromino clockwise.</p>
      <p><span style="font-weight: bold">Space:</span> Move the tetromino all the way down.</p>
    </div>

    <div style="text-align: center">
      <button @click="close" >Close</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['update:visible']);

const close = () => {
  emit('update:visible', false);
};
</script>

<style scoped>
.help-modal {
  position: absolute;
  z-index: 5;
  background-color: lightgreen;
  border: 5px solid deeppink;
  border-radius: 8px;
  padding: 20px 20px;
  transform: translate(-50%, -50%);
  top: 50%;
  left: 50%;
  text-align: left;
}

button {
  color: darkgreen;
  font-weight: bold;
  background-color: lightpink;
  }

</style>
