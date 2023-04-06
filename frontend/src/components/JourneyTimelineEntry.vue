<template>
  <q-timeline-entry
    :data-is-past="isPast"
    :color="isPast ? 'gray' : 'primary'"
    :id="`time-entry-${estimatedCall.id}`"
    v-ripple
    @click="onClick"
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
        <div>
          <q-chip
            disable
            dark
            square
            size="sm"
            v-if="
              targetDepartureTime &&
              formattedTargetDepartureTime !== formattedExpectedDepartureTime
            "
          >
            <s>{{ formattedTargetDepartureTime }}</s>
          </q-chip>
          <q-chip
            :color="!isPast ? 'primary' : undefined"
            dark
            square
            v-if="expectedDepartureTime"
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

  methods: {
    onClick() {
      this.$router.push({
        name: 'Quay',
        params: { id: this.estimatedCall.quay.id },
      });
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
