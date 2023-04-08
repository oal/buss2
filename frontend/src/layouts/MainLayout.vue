<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
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
          v-for="link in essentialLinks"
          :key="link.title"
          :exact="link.to?.name === 'Index'"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
      <q-footer class="bg-grey-3 q-pa-sm">
        <div class="flex justify-center">
          <q-btn-group outline>
            <q-btn
              :outline="$route.name !== 'Index'"
              color="secondary"
              label="Favoritter"
              icon="favorite"
              :to="{ name: 'Index' }"
            />
            <q-btn
              :outline="$route.name !== 'Search'"
              color="secondary"
              label="Søk"
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

const linksList = [
  {
    title: 'Favoritter',
    caption: 'Lagrede stopp & ruter',
    icon: 'departure_board',
    to: { name: 'Index' },
  },
  {
    title: 'Søk',
    caption: 'Finn busstopp',
    icon: 'search',
    to: { name: 'Search' },
  },
];

export default defineComponent({
  name: 'MainLayout',

  components: {
    MenuItem,
  },

  setup() {
    const leftDrawerOpen = ref(false);

    return {
      essentialLinks: linksList,
      leftDrawerOpen,
      toggleLeftDrawer() {
        leftDrawerOpen.value = !leftDrawerOpen.value;
      },
      store: useAppStore(),
    };
  },
});
</script>
