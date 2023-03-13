import app from './db/app';
import { getAuth } from '@firebase/auth';

export async function postJSON<T>(uri: string, body: any, authenticated = false): Promise<T> {
  const res = await jsonReq(uri, { method: 'POST', body: JSON.stringify(body) }, authenticated);
  if (res.ok) {
    return await res.json() as T;
  } else {
    throw new Error(`non-successful status code for POST ${uri}: ${res.statusText}`);
  }
}

export async function getJSON<T>(uri: string, authenticated = false): Promise<T | null> {
  const res = await jsonReq(uri, { method: 'GET' }, authenticated);
  if (res.ok) {
    return await res.json() as T;
  } else if (res.status === 404) {
    return null;
  } else {
    throw new Error(`non-successful status code for GET ${uri}: ${res.statusText}`);
  }
}

async function setAuthHeader(options: any) {
  const auth = getAuth(app);
  if (!auth.currentUser) {
    throw new Error('user not logged in');
  }
  if (!options.headers) {
    options.headers = {};
  }
  options.headers['X-Authentication'] = await auth.currentUser.getIdToken();
}

async function jsonReq(uri: string, options: any, authenticated: boolean): Promise<Response> {
  options.headers = {
    'Content-Type': 'application/json',
  };
  if (authenticated) {
    await setAuthHeader(options);
  }
  return fetch(uri, options);
}

export async function deleteReq(uri: string): Promise<Response> {
  const options = { method: 'DELETE' };
  await setAuthHeader(options);
  return fetch(uri, options);
}
