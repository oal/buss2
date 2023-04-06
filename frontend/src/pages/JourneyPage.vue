<template>
  <q-page>
    <q-timeline ref="timeline" class="q-px-md">
      <template v-if="journey">
        <JourneyTimelineEntry
          :estimated-call="call"
          v-for="call in journey.estimated_calls"
          :key="call.id"
        />
      </template>
    </q-timeline>
  </q-page>
</template>

<script lang="ts">
import { Journey } from 'types/Journey';
import { defineComponent, nextTick } from 'vue';
import { useAppStore } from '../stores/app-store';
import JourneyTimelineEntry from '../components/JourneyTimelineEntry.vue';

export default defineComponent({
  name: 'JourneyPage',
  components: { JourneyTimelineEntry },
  setup() {
    return {
      store: useAppStore(),
      refreshInterval: null as any,
    };
  },
  async created() {
    await this.loadData();
    this.store.setAppTitle(this.journey?.route.name ?? 'Reise');
    this.refreshInterval = setInterval(this.loadData, 5000);
  },
  beforeUnmount() {
    clearInterval(this.refreshInterval);
    this.refreshInterval = null;
  },
  data() {
    return {
      journey: null as Journey | null,
    };
  },
  watch: {
    journey: {
      handler: 'scrollToCurrent',
    },
  },
  methods: {
    loadData() {
      return this.$axios
        .get<Journey>(`/api/journeys/${this.$route.params.id}`)
        .then((response) => {
          this.journey = response.data;
          this.scrollToCurrent();
        })
        .catch((error) => {
          console.log(error);
        });
    },
    async scrollToCurrent() {
      await nextTick();
      let pastStops = this.$refs.timeline?.$el.querySelectorAll(
        '[data-is-past=true]'
      );
      let previousStop = pastStops?.[pastStops.length - 2];
      if (previousStop) {
        previousStop.scrollIntoView({ behavior: 'smooth' });
      }
    },
  },
});
</script>

<style lang="scss" scoped></style>
