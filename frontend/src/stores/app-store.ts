import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', {
  state: () => ({
    counter: 0,
    appTitle: 'Buss',
  }),
  getters: {
    doubleCount: (state) => state.counter * 2,
  },
  actions: {
    increment() {
      this.counter++;
    },
    setAppTitle(title: string) {
      this.appTitle = title;
    },
  },
});
