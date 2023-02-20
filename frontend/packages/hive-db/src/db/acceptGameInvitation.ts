import { doc, serverTimestamp, setDoc } from 'firebase/firestore';
import { newPartialGameMetaWithFieldValues } from '../game/meta';
import { newPartialGameWithFieldValues } from '../game/game';
import { gamesCollection } from './collections';

/**
 * Accept a game invitation.
 *
 * @param gid The game ID of the invitation to accept.
 */
export function acceptGameInvitation(gid: string): Promise<void> {
  const meta = newPartialGameMetaWithFieldValues();
  meta.isStarted = true;
  meta.acceptedDate = serverTimestamp();
  const gameUpdate = newPartialGameWithFieldValues();
  gameUpdate.meta = meta;
  return setDoc(doc(gamesCollection, gid), gameUpdate, { merge: true });
}
