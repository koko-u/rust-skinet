import { Component, inject, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { Header } from './shared/components/header/header';
import { httpResource } from '@angular/common/http';
import { MatProgressSpinner } from '@angular/material/progress-spinner';
import { Alert } from './shared/components/alert/alert';
import { Pagination } from './shared/models/pagination-model';
import { Product } from './features/products/product-model';
import { ProductsTable } from './features/products/products-table/products-table';
import { API_BASE_URL } from './shared/tokens/api-base-url';

@Component({
  imports: [RouterOutlet, Header, MatProgressSpinner, Alert, ProductsTable],
  selector: 'sk-root',
  styleUrl: './app.css',
  templateUrl: './app.html',
})
export class App {
  private apiBaseUrl = inject(API_BASE_URL);
  protected readonly title = signal('Skinet');

  protected products = httpResource<Pagination<Product>>(() => `${this.apiBaseUrl}/api/products`);
}
