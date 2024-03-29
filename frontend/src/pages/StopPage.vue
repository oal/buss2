<template>
  <q-page v-if="stop">
    <l-map ref="map" v-model:zoom="zoom" :center="[stop.lat, stop.lon]">
      <l-marker
        v-for="quay in stop.quays"
        :key="quay.id"
        :lat-lng="[quay.lat, quay.lon]"
        @click="loadQuay(quay.id)"
      >
        <l-popup :lat-lng="[quay.lat, quay.lon]">
          <q-spinner v-if="!quays[quay.id]" size="280px" />
          <q-card v-else flat>
            <q-card-section>
              <div role="heading" class="text-h6">
                {{ quays[quay.id].name }}
              </div>
            </q-card-section>
            <template v-if="quays[quay.id].routes?.length">
              <q-separator />
              <q-card-actions>
                <q-chip
                  v-for="route in quays[quay.id].routes"
                  :key="route.id"
                  clickable
                  @click="onRouteClick(quay.id, route.id)"
                >
                  <q-avatar :style="busColorStyle(route.short_name)">
                    {{ route.short_name }}
                  </q-avatar>
                  {{ route.name }}
                </q-chip>
              </q-card-actions>
            </template>
            <q-separator />
            <q-card-actions>
              <q-btn
                label="Vis tider for stopp"
                icon-right="arrow_forward"
                color="primary"
                flat
                @click="onRouteClick(quay.id)"
              />
            </q-card-actions>
          </q-card>
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

<script lang="ts">
import 'leaflet';
import 'leaflet/dist/leaflet.css';
import { LMap, LMarker, LPopup, LTileLayer } from '@vue-leaflet/vue-leaflet';
import { useAppStore } from 'stores/app-store.ts';
import { defineComponent } from 'vue';
import { QuayAugmented } from 'types/QuayAugmented';
import { StopWithQuays } from 'types/StopWithQuays';
import { busColorStyle } from '../helpers';

export default defineComponent({
  name: 'StopPage',
  components: { LPopup, LMarker, LTileLayer, LMap },
  setup() {
    return {
      store: useAppStore(),
      busColorStyle,
    };
  },
  data() {
    return {
      stop: null as StopWithQuays | null,
      zoom: 18,
      quays: {} as Record<number, QuayAugmented>,
    };
  },
  async created() {
    await this.loadData();
    this.store.setAppTitle(this.stop?.name ?? 'Stopp');
  },
  methods: {
    loadData() {
      return this.$axios
        .get(`/api/stops/${this.$route.params.id}`)
        .then((response) => {
          if (response.data.quays.length === 1) {
            // Go directly to quay (no need to select from map).
            this.$router.replace({
              name: 'Quay',
              params: { id: response.data.quays[0].id },
            });
          } else {
            this.stop = response.data;
          }
        })
        .catch((error) => {
          console.log(error);
        });
    },

    loadQuay(id: number) {
      this.$axios
        .get(`/api/quays/${id}`)
        .then((response) => {
          this.quays[id] = response.data;
        })
        .catch((error) => {
          console.log(error);
        });
    },

    onRouteClick(quayId: number, routeId?: number) {
      this.$router.push({
        name: 'Quay',
        params: { id: quayId },
        query: routeId ? { routes: [routeId] } : undefined,
      });
    },
  },
});
</script>

<style lang="scss">
.leaflet-container {
  min-height: inherit;
}

.leaflet-popup-content-wrapper {
  border-radius: 2px;
  background-color: transparent;
}

.leaflet-popup-content {
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  .leaflet-tile-container img {
    filter: brightness(0.6) invert(1) contrast(3) hue-rotate(200deg)
      saturate(0.3) brightness(0.7);
  }
}
</style>
