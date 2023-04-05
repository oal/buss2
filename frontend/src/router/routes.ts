import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/IndexPage.vue') },
      {
        path: 'search',
        name: 'Search',
        component: () => import('pages/SearchPage.vue'),
      },
      {
        path: 'stop/:id',
        name: 'Stop',
        component: () => import('pages/StopPage.vue'),
      },
      {
        path: 'quay/:id',
        name: 'Quay',
        component: () => import('pages/QuayPage.vue'),
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
