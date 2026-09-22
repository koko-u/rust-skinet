import {Component, input} from '@angular/core';
import {RouterLink, UrlTree} from '@angular/router';

@Component({
  imports: [
    RouterLink
  ],
  selector: 'sk-nav-link',
  styleUrl: './nav-link.css',
  templateUrl: './nav-link.html',
})
export class NavLink {
  public link = input<string | readonly any[] | UrlTree>();
}
