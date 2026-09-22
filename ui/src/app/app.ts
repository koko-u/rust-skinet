import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { Header } from './shared/components/header/header';
import { httpResource } from '@angular/common/http';
import { MatProgressSpinner } from '@angular/material/progress-spinner';
import { Alert } from './shared/components/alert/alert';

@Component({
  imports: [RouterOutlet, Header, MatProgressSpinner, Alert],
  selector: 'sk-root',
  styleUrl: './app.css',
  templateUrl: './app.html',
})
export class App {
  protected readonly title = signal('Skinet');

  protected products = httpResource<PagingProducts>(() => 'http://localhost:3000/api/products');
}

interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  pictureUrl: string;
  productType: string;
  productBrand: string;
  quantityInStock: number;
}

interface PagingProducts {
  currentPage: number;
  pageSize: number;
  totla: number;
  pages: number;
  items: Product[];
}
