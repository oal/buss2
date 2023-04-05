<template>
  <q-page>
    <div v-if="quay"></div>
    <q-list>
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
        <q-list v-if="quay?.routes">
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
        :value="departure"
        v-for="departure in nextDepartures"
        :key="departure.id"
      ></DepartureItem>
    </q-list>
  </q-page>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useAppStore } from '../stores/app-store';
import { QuayAugmented } from 'types/QuayAugmented';
import DepartureItem from '../components/DepartureItem.vue';
import uniqolor from 'uniqolor';
import { busColorStyle } from '../helpers';

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
    };
  },
  async created() {
    await this.loadData();
    this.store.setAppTitle(this.quay?.name ?? 'Busstopp');
    await this.loadDepartures();
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
