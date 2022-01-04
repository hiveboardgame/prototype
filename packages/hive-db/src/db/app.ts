import { getApp, initializeApp } from 'firebase/app';
import type { FirebaseApp } from 'firebase/app';

let app: FirebaseApp;
try {
  app = getApp('not-hive');
} catch (e) {
  app = initializeApp({
    apiKey: process.env.NEXT_PUBLIC_FIREBASE_API_KEY,
    authDomain: process.env.NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN,
    projectId: process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID
  });
}

export default app;
