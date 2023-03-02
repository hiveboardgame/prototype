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

export async function createUser(username: String): Promise<UserData> {
  return postJSON(`/api/user`, { username: username }, true);
}

export async function createGuestUser(): Promise<UserData> {
  return postJSON(`/api/guest-user`, {}, true);
}
