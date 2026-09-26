import { Component, inject } from '@angular/core';
import { CurrencyPipe } from '@angular/common';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { httpResource } from '@angular/common/http';
import { API_BASE_URL } from '../../../shared/tokens/api-base-url';
import { Pagination } from '../../../shared/models/pagination-model';
import { Product } from '../../../shared/models/product-model';

@Component({
  imports: [CurrencyPipe, MatCardModule, MatButtonModule],
  selector: 'sk-shop-page',
  styleUrl: './shop-page.css',
  templateUrl: './shop-page.html',
})
export class ShopPage {
  private apiBaseUrl = inject(API_BASE_URL);
  protected productsResource = httpResource<Pagination<Product>>(
    () => `${this.apiBaseUrl}/api/products`,
  );
}
