<template>
  <q-page class="inner-scroll-page">
    <q-list v-if="quay">
      <q-separator />
      <q-btn-dropdown class="q-py-sm route-select" flat persistent align="left">
        <template #label>
          <span class="q-pr-sm"> Viser </span>
          <q-chip
            v-for="route in selectedRouteShortNames"
            :key="route"
            size="sm"
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
    </q-list>
    <q-scroll-area>
      <q-pull-to-refresh @refresh="onRefresh">
        <q-list>
          <DepartureItem
            v-for="departure in nextDepartures"
            :key="departure.id"
            :estimated-call="departure.estimated_call"
            :route="departure.route"
            :direction="departure.direction"
            :is-favorite="Boolean(favoriteRoutes[departure.route.id])"
            @toggle:favorite="toggleFavorite(departure.route.id)"
            @click="onDepartureClick(departure)"
          ></DepartureItem>
        </q-list>
      </q-pull-to-refresh>
    </q-scroll-area>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useAppStore } from '../stores/app-store';
import { QuayAugmented } from 'types/QuayAugmented';
import DepartureItem from '../components/DepartureItem.vue';
import { busColorStyle } from '../helpers';
import db from '../db';
import { Departure } from 'types/Departure';

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

  watch: {
    selectedRoutes() {
      this.loadDepartures();
    },
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

    onDepartureClick(departure: Departure) {
      if (!this.quay) return;

      this.$router.push({
        name: 'Journey',
        params: { id: departure.id },
        query: { quay: this.quay.id },
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
    async onRefresh(done: () => void) {
      await this.loadDepartures();
      done();
    },
  },
});
</script>

<style lang="scss" scoped>
.route-select {
  width: 100%;
}
</style>
