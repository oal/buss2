<template>
  <div>
    <q-list v-if="journey">
      <DepartureItem
        :estimated-call="departure"
        :route="journey.route"
        v-for="departure in journey.estimated_calls"
        :key="departure.id"
      />
    </q-list>
  </div>
</template>

<script lang="ts">
import DepartureItem from '../components/DepartureItem.vue';
import { Journey } from 'types/Journey';
import { defineComponent } from 'vue';
import { useAppStore } from '../stores/app-store';

export default defineComponent({
  name: 'JourneyPage',
  components: { DepartureItem },
  setup() {
    return {
      store: useAppStore(),
    };
  },
  async created() {
    await this.loadData();
    this.store.setAppTitle(this.journey?.route.name ?? 'Reise');
  },
  data() {
    return {
      journey: null as Journey | null,
    };
  },
  methods: {
    loadData() {
      return this.$axios
        .get<Journey>(`/api/journeys/${this.$route.params.id}`)
        .then((response) => {
          this.journey = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },
  },
});
</script>

<style lang="scss" scoped></style>
