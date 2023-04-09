<template>
  <div>
    <q-input
      v-model="search"
      filled
      debounce="500"
      :label="$t('searchPlaceholder')"
    ></q-input>
    <q-list bordered separator>
      <q-item
        v-for="stop in stops"
        :key="stop.id"
        v-ripple
        clickable
        :to="{ name: 'Stop', params: { id: stop.id } }"
      >
        <q-item-section>{{ stop.name }}</q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { StopWithQuays } from 'types/StopWithQuays';

export default defineComponent({
  name: 'StopSearch',
  data() {
    return {
      search: '',
      stops: [] as StopWithQuays[],
    };
  },

  watch: {
    search(val: string) {
      if (val.length < 3) {
        this.stops = [];
        return;
      }
      this.$axios
        .get<StopWithQuays[]>('/api/stops', {
          params: {
            search: val,
          },
        })
        .then((response) => {
          this.stops = response.data;
        });
    },
  },
});
</script>

<style lang="scss" scoped></style>
