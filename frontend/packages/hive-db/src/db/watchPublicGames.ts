import { Game } from '../game/game';
import {
  FirestoreError,
  limit,
  onSnapshot,
  orderBy,
  query,
  where,
  Unsubscribe
} from 'firebase/firestore';
import { gamesCollection } from './collections';

/**
 * Fetch and listen for updates to the most recently played public games.
 *
 * @param count The maximum number of games to watch.
 * @param next The function to call when the query updates.
 * @param error The function to call when an errors occure.
 */
export function watchPublicGames(
  count: number,
  next: (games: Game[]) => void,
  error: (error: FirestoreError) => void
): Unsubscribe {
  return onSnapshot(
    query(
      gamesCollection,
      where('meta.public', '==', true),
      where('meta.isStarted', '==', true),
      where('meta.isEnded', '==', false),
      orderBy('meta.playedDate', 'desc'),
      limit(count)
    ),
    (snapshot) => {
      next(snapshot.docs.map((doc) => doc.data()));
    },
    error
  );
}
