import { newPartialGameMetaWithFieldValues } from '../game/meta';
import { newPartialGameWithFieldValues } from '../game/game';

/**
 * Accept a game invitation.
 *
 * @param gid The game ID of the invitation to accept.
 */
export function acceptGameInvitation(gid: string): Promise<void> {
  // TODO(wgreenberg): implement invitations
  return Promise.reject("unimplemented");
}
