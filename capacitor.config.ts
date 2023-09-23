import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'app.serenium.app',
  appName: 'serenium',
  webDir: 'build',
  server: {
    androidScheme: 'https'
  }
};

export default config;
