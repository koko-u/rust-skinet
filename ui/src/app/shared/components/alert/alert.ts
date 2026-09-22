import { Component, computed, input } from '@angular/core';

@Component({
  imports: [],
  selector: 'sk-alert',
  styleUrl: './alert.css',
  templateUrl: './alert.html',
})
export class Alert {
  public title = input.required<string>();
  public message = input.required<string>();
  public alertType = input<'primary' | 'success' | 'danger' | 'info' | 'warning'>('primary');

  protected alertTypeClass = computed(() => {
    const alertType = this.alertType();

    return {
      titleClass: `${alertType}-title`,
      messageClass: `${alertType}-message`,
    };
  });
}
