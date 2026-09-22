import { Component } from '@angular/core';
import { NgOptimizedImage } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { RouterLink } from '@angular/router';
import {NavLink} from '../nav-link/nav-link';
import {MatBadge} from '@angular/material/badge';

@Component({
  imports: [NgOptimizedImage, MatIconModule, RouterLink, NavLink, MatBadge],
  selector: 'sk-header',
  styleUrl: './header.css',
  templateUrl: './header.html',
})
export class Header {}
