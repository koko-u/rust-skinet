import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: 'shop',
    loadComponent: () => import('./features/shop/shop-page/shop-page').then((m) => m.ShopPage),
  },
];
