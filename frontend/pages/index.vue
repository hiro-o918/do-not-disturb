<template>
  <section class="container">
    <div>
      <h1 class="title">Do Not Disturb</h1>
      <div class="buttons">
        <div class="button">
          <v-btn elevation="2" large v-on:click="doNotDisturb"
            >Do Not Disturb</v-btn
          >
        </div>
        <div class="button">
          <v-btn elevation="2" large v-on:click="release">Release</v-btn>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({
  methods: {
    doNotDisturb() {
      this.$backendClient.post(`/do-not-disturb`);
    },
    release() {
      this.$backendClient.post(`/release`);
    },
  },
});
</script>

<style>
.container {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.title {
  font-family: "Quicksand", "Source Sans Pro", -apple-system, BlinkMacSystemFont,
    "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif; /* 1 */
  display: block;
  font-weight: 300;
  font-size: 100px;
  color: #35495e;
  letter-spacing: 1px;
}

.buttons {
  display: flex;
  margin: 20px;
  justify-content: center;
  align-items: center;
  text-align: center;
}

.button {
  margin: 0 20px;
}
</style>

