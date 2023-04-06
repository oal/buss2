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
        <FormattedDelay :estimated-call="estimatedCall" />
        {{ formattedDepartureTime }}
      </q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue';
import formatDistanceToNow from 'date-fns/formatDistanceToNow';
import format from 'date-fns/format';
import { busColorStyle, parseTimeOrNull } from '../helpers';
import { EstimatedCall } from 'types/EstimatedCall';
import { Route } from 'types/Route';
import FormattedDelay from './FormattedDelay.vue';

export default defineComponent({
  name: 'DepartureItem',
  components: { FormattedDelay },
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

    timeUntilDeparture() {
      return this.expectedDepartureTime
        ? formatDistanceToNow(this.expectedDepartureTime)
        : '';
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
