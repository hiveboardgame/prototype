export async function postJSON<T>(uri: string, body: any, authToken: string | null = null): Promise<T> {
  const res = await jsonReq(uri, { method: 'POST', body: JSON.stringify(body) }, authToken);
  if (res.ok) {
    return await res.json() as T;
  } else {
    throw new Error(`non-successful status code for POST ${uri}: ${res.statusText}`);
  }
}

export async function getJSON<T>(uri: string, authToken: string | null = null): Promise<T | null> {
  const res = await jsonReq(uri, { method: 'GET' }, authToken);
  if (res.ok) {
    return await res.json() as T;
  } else if (res.status === 404) {
    return null;
  } else {
    throw new Error(`non-successful status code for GET ${uri}: ${res.statusText}`);
  }
}

function setAuthHeader(options: any, authToken: string) {
  if (!options.headers) {
    options.headers = {};
  }
  options.headers['X-Authentication'] = authToken;
}

async function jsonReq(uri: string, options: any, authToken: string | null): Promise<Response> {
  options.headers = {
    'Content-Type': 'application/json',
  };
  if (authToken) {
    setAuthHeader(options, authToken);
  }
  return fetch(uri, options);
}

export async function deleteReq(uri: string, authToken: string): Promise<Response> {
  const options = { method: 'DELETE' };
  setAuthHeader(options, authToken);
  return fetch(uri, options);
}
