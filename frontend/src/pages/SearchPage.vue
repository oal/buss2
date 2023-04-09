<template>
  <q-page>
    <StopSearch />
  </q-page>
</template>

<script>
import StopSearch from 'components/StopSearch.vue';
import { useAppStore } from 'stores/app-store.ts';

export default {
  name: 'SearchPage',
  components: { StopSearch },
  setup() {
    return {
      store: useAppStore(),
    };
  },
  created() {
    this.store.setAppTitle(this.$t('search'));
  },
};
</script>

<style lang="scss" scoped></style>
