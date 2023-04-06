<template>
  <div class="stop-progress">
    <div class="stop-progress__stops">
      <div class="stop-progress__stop">
        <q-icon name="room" size="md" color="primary" />
        <strong>
          <small>Forrige stopp:</small>
          {{ previous.quay.name }}
        </strong>
      </div>
      <div class="stop-progress__stop">
        <strong>
          <small>Neste stopp:</small>
          {{ next.quay.name }}
        </strong>
        <q-icon name="room" size="md" color="primary" />
      </div>
    </div>
    <q-linear-progress v-if="progress > 0" :value="progress" />
    <div v-else>
      <q-linear-progress indeterminate color="dark" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { EstimatedCallWithQuay } from 'types/EstimatedCallWithQuay';
import { parseTimeOrNull } from '../helpers';

export default defineComponent({
  name: 'StopProgress',
  props: {
    previous: {
      type: Object as PropType<EstimatedCallWithQuay>,
      required: true,
    },
    next: {
      type: Object as PropType<EstimatedCallWithQuay>,
      required: true,
    },
  },

  setup() {
    return {
      updateInterval: null as any,
    };
  },

  data() {
    return {
      progress: 0,
    };
  },

  mounted() {
    this.updateDiffFromNow();
    this.updateInterval = setInterval(this.updateDiffFromNow, 500);
  },

  beforeUnmount() {
    clearInterval(this.updateInterval);
    this.updateInterval = null;
  },

  methods: {
    updateDiffFromNow() {
      if (!this.startTime) return;

      let now = new Date();
      let diff =
        this.diffInSeconds - (now.getTime() - this.startTime.getTime()) / 1000;
      this.progress = 1.0 - diff / this.diffInSeconds;
      if (this.progress > 1.0) this.progress = 0;
    },
  },

  computed: {
    startTime() {
      return parseTimeOrNull(this.previous.expected_departure_time);
    },
    endTime() {
      return parseTimeOrNull(this.next.expected_departure_time);
    },
    diffInSeconds() {
      if (!this.startTime || !this.endTime) return 0;
      return (this.endTime.getTime() - this.startTime.getTime()) / 1000;
    },
  },
});
</script>

<style lang="scss" scoped>
.stop-progress {
}

.stop-progress__stops {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.stop-progress__stop {
  padding: 0.5rem;
  display: flex;

  small {
    display: block;
    line-height: 0.8;
    font-weight: normal;
    padding-top: 0.3rem;
    font-size: 0.6rem;
  }
}
</style>
