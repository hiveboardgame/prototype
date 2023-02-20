import { UserData } from '../user/user';
import { usersCollection } from './collections';
import { getDoc, doc } from 'firebase/firestore';

/**
 * Get a single user.
 *
 * @param uid The user's uid.
 * @return A promise that resolves to the user's data.
 */
export function getUser(uid: string): Promise<UserData> {
  return getDoc(doc(usersCollection, uid)).then((snapshot) => snapshot.data());
}
