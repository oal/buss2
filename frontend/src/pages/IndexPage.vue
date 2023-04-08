<template>
  <q-page class="inner-scroll-page">
    <p v-if="favorites.length === 0" class="q-pa-md text-center">
      Ingen favoritter funnet.
    </p>
    <q-scroll-area>
      <q-pull-to-refresh @refresh="onRefresh">
        <q-list>
          <DepartureItem
            v-for="favorite in favorites"
            :key="favorite.journey_id"
            :estimated-call="favorite.estimated_call"
            :route="favorite.route"
            :quay="favorite.quay"
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
import { FavoriteResult } from 'types/FavoriteResult';

export default defineComponent({
  name: 'IndexPage',
  components: { DepartureItem },
  setup() {
    return {
      store: useAppStore(),
      refreshInterval: null as any,
    };
  },
  data() {
    return {
      favorites: [] as FavoriteResult[],
    };
  },
  async created() {
    this.$q.loading.show();
    this.store.setAppTitle('Buss');
    await this.loadData();
    this.$q.loading.hide();

    this.refreshInterval = setInterval(this.loadData, 5000);
  },
  beforeUnmount() {
    clearInterval(this.refreshInterval);
    this.refreshInterval = null;
  },
  methods: {
    async loadData() {
      const favorites = await db.favorites.toArray();
      const response = await this.$axios.get<FavoriteResult[]>(
        '/api/favorites',
        {
          params: {
            ids: favorites
              .map((favorite) => `${favorite.quayId}|${favorite.routeId}`)
              .join(','),
          },
        }
      );
      this.favorites = response.data;
    },
    async onRefresh(done: () => void) {
      await this.loadData();
      done();
    },
    onFavoriteClick(favorite: FavoriteResult) {
      this.$router.push({
        name: 'Journey',
        params: { id: favorite.journey_id },
        query: { quay: favorite.quay.id },
      });
    },
  },
});
</script>
