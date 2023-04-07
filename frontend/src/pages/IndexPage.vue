<template>
  <q-page class="inner-scroll-page">
    <q-scroll-area>
      <q-pull-to-refresh @refresh="onRefresh">
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
      </q-pull-to-refresh>
    </q-scroll-area>
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
    async onRefresh(done: () => void) {
      await this.loadData();
      done();
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
