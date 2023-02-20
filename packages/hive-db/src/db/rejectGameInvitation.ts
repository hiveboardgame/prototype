import { deleteDoc, doc } from 'firebase/firestore';
import { gamesCollection } from './collections';

/**
 * Reject a game invitation.
 *
 * Deletes the game document from the server.
 *
 * @param gid The game ID of the invitation to reject.
 */
export function rejectGameInvitation(gid: string): Promise<void> {
  return deleteDoc(doc(gamesCollection, gid));
}
