<template>
  <q-timeline-entry
    :data-is-past="isPast"
    :color="isPast ? 'gray' : 'primary'"
    :id="`time-entry-${estimatedCall.id}`"
  >
    <template v-slot:subtitle>
      <template v-if="isPast">Var </template>
      <FormattedDelay :estimated-call="estimatedCall" />
      ved
    </template>

    <template v-slot:title>
      <div class="flex justify-between">
        <span class="flex flex-center q-pr-sm">
          {{ estimatedCall.quay.name }}
        </span>
        <q-chip
          :color="!isPast ? 'primary' : undefined"
          dark
          square
          v-if="expectedDepartureTime"
        >
          {{ formattedExpectedDepartureTime }}
        </q-chip>
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
    expectedDepartureTime() {
      return parseTimeOrNull(this.estimatedCall.expected_departure_time);
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
});
</script>

<style lang="scss" scoped></style>
