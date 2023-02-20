import firebase from 'firebase/compat';
import {
  FirestoreError,
  doc,
  onSnapshot,
  Unsubscribe
} from 'firebase/firestore';
import { UserData } from '../user/user';
import { usersCollection } from './collections';

/**
 * Listen for updates to a user's data.
 *
 * @param uid The user's unique ID.
 * @param next The function to call when the user's data changes.
 * @param error The function to call when an error occurs.
 * @return A function that can be called to stop listening.
 */
export function watchUser(
  uid: string,
  next: (user: UserData | null) => void,
  error: (error: FirestoreError) => void
): Unsubscribe {
  return onSnapshot(
    doc(usersCollection, uid),
    (snapshot) => {
      next(snapshot.data() || null);
    },
    error
  );
}
