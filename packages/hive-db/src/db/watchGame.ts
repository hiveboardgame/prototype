import { Game } from '../game/game';
import {
  doc,
  FirestoreError,
  onSnapshot,
  Unsubscribe
} from 'firebase/firestore';
import { gamesCollection } from './collections';

export function watchGame(
  gid: string,
  next: (game: Game | undefined) => void,
  error: (error: FirestoreError) => void
): Unsubscribe {
  return onSnapshot(
    doc(gamesCollection, gid),
    (snapshot) => {
      next(snapshot.data());
    },
    error
  );
}
