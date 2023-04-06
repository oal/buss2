<template>
  <q-page>
    <q-list v-if="quay">
      <q-item-label header>Neste busser</q-item-label>
      <q-separator />
      <q-btn-dropdown class="q-py-sm route-select" flat persistent align="left">
        <template v-slot:label>
          <span class="q-pr-sm"> Viser </span>

          <q-chip
            size="sm"
            v-for="route in selectedRouteShortNames"
            :key="route"
            :style="busColorStyle(route)"
          >
            {{ route }}
          </q-chip>
        </template>
        <q-list>
          <q-item v-for="route in quay.routes" :key="route.id">
            <q-checkbox v-model="selectedRoutes" :val="route.id">
              <strong>
                {{ route.short_name }}
              </strong>
              |
              {{ route.name }}
            </q-checkbox>
          </q-item>
        </q-list>
      </q-btn-dropdown>

      <q-separator />
      <DepartureItem
        :journey-id="departure.id"
        :estimated-call="departure.estimated_call"
        :route="departure.route"
        v-for="departure in nextDepartures"
        :key="departure.id"
        :quay-id="quay.id"
        :is-favorite="Boolean(favoriteRoutes[departure.route.id])"
        @toggle:favorite="toggleFavorite(departure.route.id)"
      ></DepartureItem>
    </q-list>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useAppStore } from '../stores/app-store';
import { QuayAugmented } from 'types/QuayAugmented';
import DepartureItem from '../components/DepartureItem.vue';
import { busColorStyle } from '../helpers';
import db from '../db';

export default defineComponent({
  name: 'QuayPage',
  components: { DepartureItem },
  setup() {
    return {
      store: useAppStore(),
      busColorStyle,
    };
  },
  data() {
    return {
      quay: null as QuayAugmented | null,
      nextDepartures: [],
      favoriteRoutes: {} as Record<number, number>,
    };
  },
  async created() {
    this.$q.loading.show();
    await this.loadData();
    this.store.setAppTitle(this.quay?.name ?? 'Busstopp');
    await this.loadDepartures();
    await this.loadFavorites();
    this.$q.loading.hide();
  },
  methods: {
    loadData() {
      return this.$axios
        .get(`/api/quays/${this.$route.params.id}`)
        .then((response) => {
          this.quay = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },
    loadDepartures() {
      return this.$axios
        .get('/api/journeys', {
          params: {
            quay: this.$route.params.id,
            routes: this.selectedRoutes.join(','),
          },
        })
        .then((response) => {
          this.nextDepartures = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },
    loadFavorites() {
      if (!this.quay) return;
      return db.favorites
        .where('quayId')
        .equals(this.quay.id)
        .toArray()
        .then((favorites) => {
          this.favoriteRoutes = favorites.reduce(
            (acc, favorite) => ({
              ...acc,
              [favorite.routeId]: favorite.id,
            }),
            {}
          );
        });
    },
    async toggleFavorite(routeId: number) {
      if (!this.quay) return;

      let favoriteId = this.favoriteRoutes[routeId];
      if (favoriteId) {
        await db.favorites.delete(favoriteId);
        delete this.favoriteRoutes[routeId];
      } else {
        this.favoriteRoutes[routeId] = await db.favorites.add({
          routeId,
          quayId: this.quay.id,
        });
      }
    },
  },

  watch: {
    selectedRoutes() {
      this.loadDepartures();
    },
  },

  computed: {
    selectedRouteShortNames() {
      return this.quay?.routes
        .filter((x) => this.selectedRoutes.includes(x.id))
        .map((x) => x.short_name);
    },
    selectedRoutes: {
      get() {
        let routesString = this.$route.query.routes as string;
        if (!routesString) {
          return this.quay?.routes.map((x) => x.id) ?? [];
        } else {
          return `${routesString}`.split(',').map(Number) ?? [];
        }
      },
      set(routeIds: number[]) {
        this.$router.replace({
          ...this.$route,
          query: {
            routes: routeIds
              .filter((x) => !!x)
              .sort((a, b) => a - b)
              .join(','),
          },
        });
      },
    },
  },
});
</script>

<style lang="scss" scoped>
.route-select {
  width: 100%;
}
</style>
