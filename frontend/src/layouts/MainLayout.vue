<template>
  <q-layout view="lHh Lpr lFf">
    <q-header :class="$q.dark.isActive ? 'bg-grey-10' : null">
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />
        <q-toolbar-title>{{ store.appTitle }}</q-toolbar-title>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" show-if-above bordered>
      <q-list>
        <q-item-label header> Navigasjon </q-item-label>

        <MenuItem
          v-for="link in links"
          :key="link.title"
          :exact="link.to?.name === 'Index'"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
      <q-footer
        :class="
          $q.dark.isActive ? 'bg-grey-10 text-white' : 'bg-white text-black'
        "
        class="app-footer"
      >
        <div class="flex justify-center">
          <q-btn-group flat>
            <q-btn
              flat
              :color="$route.name === 'Index' ? 'accent' : ''"
              :label="$t('favorites')"
              icon="favorite"
              :to="{ name: 'Index' }"
            />
            <q-btn
              flat
              :color="$route.name === 'Search' ? 'accent' : ''"
              :label="$t('search')"
              icon="search"
              :to="{ name: 'Search' }"
            />
          </q-btn-group>
        </div>
      </q-footer>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import MenuItem from 'components/MenuItem.vue';
import { useAppStore } from 'stores/app-store';

export default defineComponent({
  name: 'MainLayout',

  components: {
    MenuItem,
  },

  setup() {
    const leftDrawerOpen = ref(false);

    return {
      leftDrawerOpen,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
      store: useAppStore(),
    };
  },

  data() {
    return {
      links: [
        {
          title: this.$t('favorites'),
          caption: this.$t('favoritesDescription'),
          icon: 'departure_board',
          to: { name: 'Index' },
        },
        {
          title: this.$t('search'),
          caption: this.$t('searchDescription'),
          icon: 'search',
          to: { name: 'Search' },
        },
      ],
    };
  },
});
</script>

<style lang="scss">
.app-footer {
  .q-btn__content {
    flex-direction: column;
    text-transform: none;
    padding: 0.65rem;
    line-height: 1.2;
    .q-icon {
      margin: 0;
    }
  }
}
</style>
