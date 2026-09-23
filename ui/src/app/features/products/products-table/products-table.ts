import { Component, input } from '@angular/core';
import { Product } from '../product-model';
import {
  MatCell,
  MatCellDef,
  MatColumnDef,
  MatHeaderCell,
  MatHeaderCellDef,
  MatHeaderRow,
  MatHeaderRowDef,
  MatRow,
  MatRowDef,
  MatTable,
} from '@angular/material/table';
import { MatButton } from '@angular/material/button';
import { CurrencyPipe } from '@angular/common';

@Component({
  imports: [
    MatTable,
    MatColumnDef,
    MatHeaderCell,
    MatCell,
    MatHeaderCellDef,
    MatCellDef,
    MatHeaderRow,
    MatHeaderRowDef,
    MatRow,
    MatRowDef,
    MatButton,
    CurrencyPipe,
  ],
  selector: 'sk-products-table',
  styleUrl: './products-table.css',
  templateUrl: './products-table.html',
})
export class ProductsTable {
  public products = input.required<Product[]>();
}
