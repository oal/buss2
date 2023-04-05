<template>
  <q-item clickable v-ripple>
    <q-item-section avatar>
      <q-avatar color="primary" text-color="white">
        {{ value.route.short_name }}
      </q-avatar>
    </q-item-section>

    <q-item-section>
      <q-item-label lines="1"
        >{{ timeUntilDeparture }}
        {{ value.estimated_call.expected_departure_time }}</q-item-label
      >
      <q-item-label caption lines="2">
        <template v-if="lateBy"> {{ lateBy }} forsinket </template>
        <template v-else-if="showOnTime"> I rute </template>
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Journey } from 'types/Journey';
import formatDistanceToNow from 'date-fns/formatDistanceToNow';
import formatDistance from 'date-fns/formatDistance';
import differenceInHours from 'date-fns/differenceInHours';

const parseTimeOrNull = (time: string | null) => (time ? new Date(time) : null);

export default defineComponent({
  name: 'DepartureItem',
  props: {
    value: {
      type: Object as PropType<Journey>,
      required: true,
    },
  },

  computed: {
    expectedDepartureTime() {
      return parseTimeOrNull(this.value.estimated_call.expected_departure_time);
    },

    targetDepartureTime() {
      return parseTimeOrNull(this.value.estimated_call.expected_departure_time);
    },

    timeUntilDeparture() {
      return this.expectedDepartureTime
        ? formatDistanceToNow(this.expectedDepartureTime)
        : '';
    },

    showOnTime() {
      return (
        this.targetDepartureTime &&
        differenceInHours(this.targetDepartureTime, new Date()) < 1
      );
    },

    lateBy() {
      let expectedDepartureTime =
        this.value.estimated_call.expected_departure_time;
      let targetDepartureTime = this.value.estimated_call.target_departure_time;
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
