import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter, withComponentInputBinding } from '@angular/router';
import { routes } from './app.routes';
import { provideHttpClient } from '@angular/common/http';
import { API_BASE_URL } from './shared/tokens/api-base-url';
import { environment } from '../environments/environment';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes, withComponentInputBinding({ queryParams: false })),
    provideHttpClient(),
    {
      provide: API_BASE_URL,
      useValue: environment.apiBaseUrl,
    },
  ],
};
