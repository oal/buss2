<template>
  <q-page v-if="stop">
    <l-map ref="map" v-model:zoom="zoom" :center="[stop.lat, stop.lon]">
      <l-marker
        :lat-lng="[quay.lat, quay.lon]"
        v-for="quay in stop.quays"
        :key="quay.id"
      >
        <l-popup :lat-lng="[quay.lat, quay.lon]">
          <div>
            <h3>{{ quay.name }}</h3>
          </div>
        </l-popup>
      </l-marker>

      <l-tile-layer
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        layer-type="base"
        name="OpenStreetMap"
      ></l-tile-layer>
    </l-map>
  </q-page>
</template>

<script>
import 'leaflet';
import 'leaflet/dist/leaflet.css';
import { LMap, LMarker, LPopup, LTileLayer } from '@vue-leaflet/vue-leaflet';
import { useAppStore } from 'stores/app-store.ts';

export default {
  name: 'StopPage',
  components: { LPopup, LMarker, LTileLayer, LMap },
  setup() {
    return {
      store: useAppStore(),
    };
  },
  async created() {
    await this.loadData();
    this.store.setAppTitle(this.stop.name ?? 'Stopp');
  },
  data() {
    return {
      stop: null,
      zoom: 18,
    };
  },

  methods: {
    loadData() {
      return this.$axios
        .get(`/api/stops/${this.$route.params.id}`)
        .then((response) => {
          this.stop = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },
  },
};
</script>

<style lang="scss">
.leaflet-container {
  min-height: inherit;
}
</style>
