<template>
  <div class="help-modal" v-if="visible">
    <h3>Help</h3>
    <button @click="close">Close</button>
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
}

button {
  color: darkgreen;
  font-weight: bold;
  background-color: lightpink;
  }

</style>
