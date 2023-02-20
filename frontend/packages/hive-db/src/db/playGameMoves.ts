import {
  Game,
  getBlackUid,
  getGameNotation,
  getGameOptions,
  getWhiteUid,
  newPartialGameWithFieldValues
} from '../game/game';
import {
  buildBoard,
  buildGameNotation,
  buildPassMove,
  canMove,
  GameBoard,
  getGameMoves,
  getGameResult,
  Move
} from 'hive-lib';
import { getColorTurn, newGameState } from '../game/state';
import { newPartialGameMetaWithFieldValues } from '../game/meta';
import { doc, serverTimestamp, setDoc } from 'firebase/firestore';
import { gamesCollection } from './collections';

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
export function playGameMoves(game: Game, moves: Move[]): Promise<void> {
  const gid = game.gid;
  if (!gid.length) {
    throw new Error('Cannot play move, no game id provided.');
  }
  if (!moves.length) {
    return Promise.resolve();
  }

  const currentNotation = getGameNotation(game);
  const currentMoves = getGameMoves(currentNotation);
  const nextMoves = [...currentMoves, ...moves];
  const nextNotation = buildGameNotation(nextMoves);
  const nextBoard = buildBoard(nextMoves);

  const nextGameState = newGameState(nextNotation);
  const nextGameMeta = newPartialGameMetaWithFieldValues();

  const lastNewMove = moves[moves.length - 1];
  if (lastNewMove.end === true) {
    nextGameMeta.isEnded = true;
    nextGameMeta.endedDate = serverTimestamp();
    nextGameMeta.result = determineGameResult(game, nextBoard);
  } else {
    const nextColorTurn = getColorTurn(nextGameState);
    const gameOptions = getGameOptions(game);
    const mustPass = !canMove(nextBoard, nextColorTurn, gameOptions);
    if (mustPass) {
      if (lastNewMove.notation === 'x') throwStalemateError();
      const passMove = buildPassMove();
      return playGameMoves(game, [...moves, passMove]);
    }
  }

  nextGameMeta.playedDate = serverTimestamp();

  const gameUpdate = newPartialGameWithFieldValues();
  gameUpdate.state = nextGameState;
  gameUpdate.meta = nextGameMeta;
  return setDoc(doc(gamesCollection, gid), gameUpdate, { merge: true });
}

function determineGameResult(game: Game, board: GameBoard): string {
  const result = getGameResult(board);
  if (result === 'black') return getBlackUid(game);
  if (result === 'white') return getWhiteUid(game);
  return result;
}

/**
 * TODO: Use Alloy to check if a stalemate is possible. Until then, let's
 *       include this failsafe to prevent infinite passing.
 */
function throwStalemateError() {
  throw new Error(
    'This move ends the game in a stalemate. If you are seeing this ' +
      'message, PLEASE notify the developers because you have stumbled upon ' +
      'a very unique case!'
  );
}
