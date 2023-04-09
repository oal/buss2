<template>
  <q-timeline-entry
    :id="`time-entry-${estimatedCall.id}`"
    v-ripple
    :data-is-past="isPast"
    :color="isPast ? 'gray' : 'primary'"
    class="timeline-entry"
    @click="onClick"
  >
    <template #title>
      <div class="flex justify-between">
        <div>
          {{ estimatedCall.quay.name }}

          <div class="timeline-entry__delay">
            <template v-if="isPast">Var </template>
            <FormattedDelay :estimated-call="estimatedCall" />
          </div>
        </div>

        <div>
          <q-chip
            v-if="
              targetDepartureTime &&
              formattedTargetDepartureTime !== formattedExpectedDepartureTime
            "
            disable
            dark
            square
            size="sm"
            class="q-my-none"
          >
            <s>{{ formattedTargetDepartureTime }}</s>
          </q-chip>
          <q-chip
            v-if="expectedDepartureTime"
            :color="!isPast ? 'primary' : undefined"
            dark
            square
            class="q-my-none q-mr-none"
          >
            {{ formattedExpectedDepartureTime }}
          </q-chip>
        </div>
      </div>
    </template>
  </q-timeline-entry>
</template>

<script lang="ts">
import FormattedDelay from './FormattedDelay.vue';
import { defineComponent, PropType } from 'vue';
import { EstimatedCallWithQuay } from 'types/EstimatedCallWithQuay';
import { parseTimeOrNull } from '../helpers';
import format from 'date-fns/format';

export default defineComponent({
  name: 'JourneyTimelineEntry',
  components: { FormattedDelay },
  props: {
    estimatedCall: {
      type: Object as PropType<EstimatedCallWithQuay>,
      required: true,
    },
  },
  computed: {
    targetDepartureTime() {
      return parseTimeOrNull(this.estimatedCall.target_departure_time);
    },
    formattedTargetDepartureTime() {
      return this.targetDepartureTime
        ? format(this.targetDepartureTime, 'HH:mm')
        : null;
    },
    expectedDepartureTime() {
      return parseTimeOrNull(
        this.estimatedCall.expected_departure_time ??
          this.estimatedCall.expected_arrival_time
      );
    },
    formattedExpectedDepartureTime() {
      return this.expectedDepartureTime
        ? format(this.expectedDepartureTime, 'HH:mm')
        : null;
    },
    isPast() {
      const expected = this.expectedDepartureTime;
      if (!expected) return false;
      return new Date(expected) < new Date();
    },
  },

  methods: {
    onClick() {
      this.$router.push({
        name: 'Quay',
        params: { id: this.estimatedCall.quay.id },
      });
    },
  },
});
</script>

<style lang="scss">
.timeline-entry {
}
.timeline-entry__delay {
  font-size: 0.8rem;
  font-weight: normal;

  &:first-letter {
    text-transform: uppercase;
  }
}
</style>
