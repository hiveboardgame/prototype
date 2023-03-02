import { postJSON } from '../api';

/**
 * Update a user's username, creating the user's profile if it doesn't exist.
 *
 * @param uid The user's unique ID.
 * @param username A username that has been verified to not already be in use.
 * @returns A promise that resolves to void.
 */
export function updateUsername(uid: string, username: string): Promise<void> {
  return postJSON(`/api/user/username`, { uid: uid, username: username }, true);
}