<template>
  <q-item clickable :to="to" :exact="exact">
    <q-item-section v-if="icon" avatar>
      <q-avatar>
        <q-icon size="md" :name="icon" />
      </q-avatar>
    </q-item-section>

    <q-item-section>
      <q-item-label>{{ title }}</q-item-label>
      <q-item-label caption>{{ caption }}</q-item-label>
    </q-item-section>
  </q-item>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'MenuItem',
  props: {
    title: {
      type: String,
      required: true,
    },

    caption: {
      type: String,
      default: '',
    },

    to: {
      type: Object,
      default: undefined,
    },

    icon: {
      type: String,
      default: '',
    },

    exact: {
      type: Boolean,
      default: false,
    },
  },
});
</script>
