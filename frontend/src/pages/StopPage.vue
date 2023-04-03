<template>
  <div v-if="stop">
    <h1>{{ stop.name }}</h1>
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
  </div>
</template>

<script>
import 'leaflet';
import 'leaflet/dist/leaflet.css';
import { LMap, LMarker, LPopup, LTileLayer } from '@vue-leaflet/vue-leaflet';

export default {
  name: 'StopPage',
  components: { LPopup, LMarker, LTileLayer, LMap },
  created() {
    this.loadData();
  },
  data() {
    return {
      stop: null,
      zoom: 17,
    };
  },

  methods: {
    loadData() {
      this.$axios
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
  min-height: 50vh;
}
</style>
