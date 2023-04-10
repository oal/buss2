<template>
  <q-page class="inner-scroll-page">
    <q-banner
      v-if="journey && !hasStarted"
      class="bg-secondary text-white q-py-md"
    >
      {{ $t('journeyNotStarted') }}
    </q-banner>
    <div v-if="previousAndNextStop">
      <StopProgress
        :previous="previousAndNextStop.previous"
        :next="previousAndNextStop.next"
      />
    </div>
    <q-scroll-area class="journey-timeline">
      <q-pull-to-refresh @refresh="onRefresh">
        <q-timeline ref="timeline" class="journey-timeline__list q-px-md">
          <template v-if="journey">
            <JourneyTimelineEntry
              v-for="call in journey.estimated_calls"
              :key="call.id"
              :estimated-call="call"
            />
          </template>
        </q-timeline>
      </q-pull-to-refresh>
    </q-scroll-area>
    <q-separator />
    <div
      :class="
        $q.dark.isActive ? 'bg-grey-10 text-white' : 'bg-grey-2 text-black'
      "
      class="q-pa-xs"
    >
      <q-toggle v-model="autoScroll" color="accent">
        {{ $t('autoScroll') }}
      </q-toggle>
    </div>
  </q-page>
</template>

<script lang="ts">
import { Journey } from 'types/Journey';
import { defineComponent, nextTick } from 'vue';
import { useAppStore } from '../stores/app-store';
import JourneyTimelineEntry from '../components/JourneyTimelineEntry.vue';
import StopProgress from '../components/StopProgress.vue';

export default defineComponent({
  name: 'JourneyPage',
  components: { StopProgress, JourneyTimelineEntry },
  setup() {
    return {
      store: useAppStore(),
      refreshInterval: null as any,
    };
  },
  data() {
    return {
      journey: null as Journey | null,
      autoScroll: true,
    };
  },
  computed: {
    hasStarted() {
      const firstTime = this.journey?.estimated_calls.find(
        (call) => call.expected_departure_time
      )?.expected_departure_time;
      return firstTime && new Date(firstTime) < new Date();
    },
    previousAndNextStop() {
      let now = new Date();
      const next = this.journey?.estimated_calls.find((call) => {
        let expectedDepartureTime = call.expected_departure_time;
        return expectedDepartureTime && new Date(expectedDepartureTime) > now;
      });

      if (!next) return null;

      const indexOfCurrentEntry = this.journey?.estimated_calls.indexOf(next);
      if (
        typeof indexOfCurrentEntry === 'undefined' ||
        indexOfCurrentEntry <= 0
      )
        return null;

      const previous = this.journey?.estimated_calls[indexOfCurrentEntry - 1];
      if (typeof previous === 'undefined') return null;

      return { previous, next };
    },
  },
  watch: {
    journey() {
      setTimeout(() => {
        if (!this.autoScroll) return;
        this.scrollToCurrent();
      }, 500);
    },
  },
  async created() {
    this.$q.loading.show();
    await this.loadData();
    this.store.setAppTitle(this.journey?.route.name ?? 'Reise');
    this.$q.loading.hide();
    this.refreshInterval = setInterval(this.loadData, 5000);
  },
  beforeUnmount() {
    clearInterval(this.refreshInterval);
    this.refreshInterval = null;
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
    async onRefresh(done: () => void) {
      await this.loadData();
      done();
    },
    async scrollToCurrent() {
      await nextTick();
      let pastStops = this.$refs.timeline?.$el.querySelectorAll(
        '[data-is-past=true]'
      );
      let previousStop = pastStops?.[pastStops.length - 1];
      if (previousStop) {
        previousStop.scrollIntoView({ behavior: 'smooth' });
      }
    },
  },
});
</script>

<style lang="scss"></style>
