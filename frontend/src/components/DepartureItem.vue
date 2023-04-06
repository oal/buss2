<template>
  <q-item clickable v-ripple @click="onClick">
    <q-item-section avatar>
      <q-avatar :style="busColorStyle(route.short_name)">
        {{ route.short_name }}
      </q-avatar>
    </q-item-section>

    <q-item-section>
      <q-item-label lines="1"> Om {{ timeUntilDeparture }} </q-item-label>
      <q-item-label caption lines="2">
        <template v-if="lateBy"> {{ lateBy }} forsinket, </template>
        <template v-else-if="showOnTime"> I rute, </template>
        {{ formattedDepartureTime }}
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import formatDistanceToNow from 'date-fns/formatDistanceToNow';
import formatDistance from 'date-fns/formatDistance';
import differenceInHours from 'date-fns/differenceInHours';
import format from 'date-fns/format';
import { busColorStyle } from '../helpers';
import { EstimatedCall } from 'types/EstimatedCall';
import { Route } from 'types/Route';

const parseTimeOrNull = (time: string | null) => (time ? new Date(time) : null);

export default defineComponent({
  name: 'DepartureItem',
  props: {
    journeyId: {
      type: Number,
    },
    quayId: {
      type: Number,
    },
    estimatedCall: {
      type: Object as PropType<EstimatedCall>,
      required: true,
    },
    route: {
      type: Object as PropType<Route>,
      required: true,
    },
  },
  setup() {
    return {
      busColorStyle,
    };
  },

  methods: {
    onClick() {
      if (!this.quayId || !this.journeyId) return;
      this.$router.push({
        name: 'Journey',
        params: { id: this.journeyId },
        query: { quay: this.quayId },
      });
    },
  },

  computed: {
    expectedDepartureTime() {
      return parseTimeOrNull(this.estimatedCall.expected_departure_time);
    },

    targetDepartureTime() {
      return parseTimeOrNull(this.estimatedCall.expected_departure_time);
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

    formattedDepartureTime() {
      return this.expectedDepartureTime
        ? format(this.expectedDepartureTime, 'HH:mm')
        : '';
    },
  },
});
</script>

<style lang="scss" scoped></style>
