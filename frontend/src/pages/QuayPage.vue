<template>
  <q-page>
    <div v-if="quay">
      <q-item-label header>Bussruter</q-item-label>
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
    </div>
    <q-list>
      <q-item-label header>Neste busser</q-item-label>

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

export default defineComponent({
  name: 'QuayPage',
  components: { DepartureItem },
  setup() {
    return {
      store: useAppStore(),
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
    selectedRoutes: {
      get() {
        return (
          (this.$route.query.routes as string)?.split(',').map(Number) ?? []
        );
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

<style lang="scss" scoped></style>
