<template>
  <q-item class="departure-item" clickable v-ripple @click="$emit('click')">
    <q-item-section class="departure-item__route">
      <q-chip
        class="departure-item__route-tag"
        :style="busColorStyle(route.short_name)"
      >
        {{ route.short_name }}
      </q-chip>
    </q-item-section>

    <q-item-section>
      <q-item-label lines="1">
        <template v-if="quay">
          {{ quay.name }}
        </template>
        om {{ timeUntilDeparture }}
      </q-item-label>
      <q-item-label caption lines="2">
        <FormattedDelay :estimated-call="estimatedCall" />
        {{ formattedDepartureTime }}
      </q-item-label>
    </q-item-section>

    <q-item-section side v-if="typeof isFavorite !== 'undefined'">
      <q-btn
        round
        flat
        :loading="favoriteSaving"
        :icon="isFavorite ? 'favorite' : 'favorite_border'"
        :color="isFavorite ? 'red-10' : 'grey'"
        @click.stop="onToggleFavorite"
      ></q-btn>
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
import { Quay } from 'types/Quay';

export default defineComponent({
  name: 'DepartureItem',
  components: { FormattedDelay },
  props: {
    isFavorite: {
      type: Boolean,
      default: undefined,
    },
    quay: {
      type: Object as PropType<Quay>,
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

  data() {
    return {
      favoriteSaving: false,
    };
  },

  methods: {
    onToggleFavorite() {
      this.favoriteSaving = true;
      setTimeout(() => {
        this.favoriteSaving = false;
      }, 1000);
      this.$emit('toggle:favorite');
    },
  },
  watch: {
    isFavorite() {
      this.favoriteSaving = false;
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

<style lang="scss">
.departure-item {
}

.departure-item__route {
  flex-grow: 0;
  flex-basis: 3.5rem;
  flex-shrink: 1;
  font-weight: bold;
}

.departure-item__route-tag {
  margin: 0 0.5rem 0 0;
  .q-chip__content {
    justify-content: center;
  }
}
</style>
