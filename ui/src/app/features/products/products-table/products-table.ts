import { Component, input } from '@angular/core';
import { MatButton } from '@angular/material/button';
import { CurrencyPipe } from '@angular/common';
import { Product } from '../../../shared/models/product-model';

@Component({
  imports: [MatButton, CurrencyPipe],
  selector: 'sk-products-table',
  styleUrl: './products-table.css',
  templateUrl: './products-table.html',
})
export class ProductsTable {
  public products = input.required<Product[]>();
}
