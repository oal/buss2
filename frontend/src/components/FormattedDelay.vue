<template>
  <template v-if="lateBy"> {{ lateBy }} forsinket</template>
  <template v-else-if="showOnTime"> I rute</template>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import differenceInHours from 'date-fns/differenceInHours';
import formatDistance from 'date-fns/formatDistance';
import { EstimatedCall } from 'types/EstimatedCall';
import { parseTimeOrNull } from '../helpers';
import { EstimatedCallWithQuay } from 'types/EstimatedCallWithQuay';

export default defineComponent({
  name: 'FormattedDelay',
  props: {
    estimatedCall: {
      type: Object as PropType<EstimatedCall | EstimatedCallWithQuay>,
      required: true,
    },
  },
  computed: {
    targetDepartureTime() {
      return parseTimeOrNull(this.estimatedCall.expected_departure_time);
    },
    showOnTime() {
      return (
        this.targetDepartureTime &&
        differenceInHours(this.targetDepartureTime, new Date()) < 1
      );
    },

    lateBy() {
      let expectedDepartureTime = this.estimatedCall.expected_departure_time;
      let targetDepartureTime = this.estimatedCall.target_departure_time;
      if (
        expectedDepartureTime === targetDepartureTime ||
        !expectedDepartureTime ||
        !targetDepartureTime
      )
        return null;

      return formatDistance(
        new Date(expectedDepartureTime),
        new Date(targetDepartureTime)
      );
    },
  },
});
</script>

<style lang="scss" scoped></style>
