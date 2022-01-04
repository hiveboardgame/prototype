import { Game } from '../game/game';
import {
  FirestoreError,
  onSnapshot,
  query,
  where,
  Unsubscribe
} from 'firebase/firestore';
import { gamesCollection } from './collections';

/**
 * Fetch and listen for updates to a user's games.
 *
 * @param uid The user's unique ID.
 * @param next The function to call when any of the games are updated.
 * @param error The function to call when an error occurs.
 */
export function watchUserGames(
  uid: string,
  next: (games: Game[]) => void,
  error: (error: FirestoreError) => void
): Unsubscribe {
  return onSnapshot(
    query(gamesCollection, where('players.uids', 'array-contains', uid)),
    (snapshot) => {
      next(snapshot.docs.map((doc) => doc.data()));
    },
    error
  );
}
