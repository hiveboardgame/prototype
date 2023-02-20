import { UserData } from '../user/user';
import { usersCollection } from './collections';
import { getDocs, query, where } from 'firebase/firestore';

/**
 * Get all users except one.
 *
 * @param except The uid of the user to exclude.
 * @return A promise that resolves to an array of users.
 */
export function getUsersExcept(except: string): Promise<UserData[]> {
  return getDocs(query(usersCollection, where('uid', '!=', except))).then(
    (snapshot) => {
      return snapshot.docs.map((doc) => doc.data());
    }
  );
}
