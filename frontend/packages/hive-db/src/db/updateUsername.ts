import { doc, setDoc } from 'firebase/firestore';
import { usersCollection } from './collections';

/**
 * Update a user's username, creating the user's profile if it doesn't exist.
 *
 * @param uid The user's unique ID.
 * @param username A username that has been verified to not already be in use.
 * @returns A promise that resolves to void.
 */
export function updateUsername(uid: string, username: string): Promise<void> {
  return setDoc(doc(usersCollection, uid), { uid, username }, { merge: true });
}
