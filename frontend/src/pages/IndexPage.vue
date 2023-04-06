<template>
  <q-page class="row items-center justify-evenly">
    <ul>
      <li v-for="favorite in favorites" :key="favorite.id">
        {{ favorite }}
      </li>
    </ul>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import db, { Favorite } from '../db';

export default defineComponent({
  name: 'IndexPage',

  async created() {
    this.$q.loading.show();
    await this.loadFavorites();
    await this.loadData();
    this.$q.loading.hide();
  },
  data() {
    return {
      favorites: [] as Favorite[],
    };
  },
  methods: {
    async loadFavorites() {
      this.favorites = await db.favorites.toArray();
    },
    async loadData() {
      const response = await this.$axios.get('/api/journeys/favorites', {
        params: {
          ids: this.favorites
            .map((favorite) => `${favorite.quayId}|${favorite.routeId}`)
            .join(','),
        },
      });
      console.log(response.data);
    },
  },
});
</script>
