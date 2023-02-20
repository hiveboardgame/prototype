import { getDocsFromServer, limit, query, where } from 'firebase/firestore';
import { usersCollection } from './collections';

/**
 * Determine if a username is available.
 *
 * @param username The username to check.
 * @return A promise that resolves to true if the username is available, false otherwise.
 */
export function getUsernameAvailable(username: string): Promise<boolean> {
  return getDocsFromServer(
    query(usersCollection, where('username', '==', username), limit(1))
  ).then((snapshot) => snapshot.empty);
}
