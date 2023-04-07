<template>
  <q-page>
    <q-list>
      <DepartureItem
        :estimated-call="favorite.estimated_call"
        :quay-id="favorite.quay.id"
        :route="favorite.route"
        v-for="favorite in favorites"
        :key="favorite.id"
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

export default defineComponent({
  name: 'IndexPage',
  components: { DepartureItem },

  async created() {
    this.$q.loading.show();
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
  },
});
</script>
