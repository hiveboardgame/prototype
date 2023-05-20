import { Game } from '../game/game';
import { Move } from 'hive-lib';
import { postJSON } from '../api';

interface GameMoveResponse {
  game: Game;
  validNextMoves: Move[];
}

/**
 * Play some number of moves by updating a game document in the Firestore
 * database.
 *
 * Determines if the given moves result in a required pass from the next player,
 * and if so adds that pass to the list of moves being played. Also determines
 * if the list of moves ends the game, and updates the metadata and player
 * data accordingly.
 *
 * @param game The game to update.
 * @param moves An ordered list of moves to play.
 */
export function playGameMove(
  game: Game,
  move: Move,
  authToken: string
): Promise<GameMoveResponse> {
  const uri = `/api/game/${game.gid}/play`;
  // TODO: Neel: need to play around with this more and figure out any conversion
  return postJSON(uri, { Turn: [move.notation, '.'] }, authToken);
}
