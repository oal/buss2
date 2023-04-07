<template>
  <q-page>
    <q-list>
      <DepartureItem
        :estimated-call="favorite.estimated_call"
        :route="favorite.route"
        :quay="favorite.quay"
        v-for="favorite in favorites"
        :key="favorite.id"
        @click="onFavoriteClick(favorite)"
      >
        {{ favorite }}
      </DepartureItem>
    </q-list>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import db from '../db';
import DepartureItem from '../components/DepartureItem.vue';
import { useAppStore } from '../stores/app-store';

export default defineComponent({
  name: 'IndexPage',
  components: { DepartureItem },
  setup() {
    return {
      store: useAppStore(),
    };
  },
  async created() {
    this.$q.loading.show();
    this.store.setAppTitle('Buss');
    await this.loadData();
    this.$q.loading.hide();
  },
  data() {
    return {
      favorites: [],
    };
  },
  methods: {
    async loadData() {
      const favorites = await db.favorites.toArray();
      const response = await this.$axios.get('/api/journeys/favorites', {
        params: {
          ids: favorites
            .map((favorite) => `${favorite.quayId}|${favorite.routeId}`)
            .join(','),
        },
      });
      this.favorites = response.data;
    },
    onFavoriteClick(favorite: any) {
      this.$router.push({
        name: 'Journey',
        params: { id: favorite.journey_id },
        query: { quay: favorite.quay.id },
      });
    },
  },
});
</script>
