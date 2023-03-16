import { postJSON, getJSON } from '../api';
import { UserData } from '../user/user';

/**
 * Get a single user.
 *
 * @param uid The user's uid.
 * @return A promise that resolves to the user's data.
 */
export async function getUser(uid: string): Promise<UserData | null> {
  return getJSON(`/api/user/${uid}`);
}

export async function createUser(username: String, authToken: string): Promise<UserData> {
  return postJSON(`/api/user`, { username: username }, authToken);
}

export async function createGuestUser(authToken: string): Promise<UserData> {
  return postJSON(`/api/guest-user`, {}, authToken);
}
