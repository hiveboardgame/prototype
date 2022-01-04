import type { ColorKey, GameOptions } from 'hive-lib';
import type { GameMeta } from './meta';
import type { GamePlayers } from './players';
import type { GameState } from './state';
import * as meta from './meta';
import * as options from './options';
import * as players from './players';
import * as state from './state';
import { UserData } from '../user/user';
import {
  DocumentData,
  PartialWithFieldValue,
  WithFieldValue
} from 'firebase/firestore';
import { newGameMeta, newGameMetaWithFieldValues } from './meta';
import { newGameState } from './state';

export interface Game {
  gid: string;
  meta: GameMeta;
  options: GameOptions;
  players: GamePlayers;
  state: GameState;
}

/**
 * Create a new game object.
 *
 * @param creatorUid The UID of the player creating the game.
 * @param players A GamePlayers object.
 * @param options A GameOptions object.
 * @param isPublic A boolean indicating game visibility.
 */
export function newGame(
  creatorUid: string,
  players: GamePlayers,
  options: GameOptions,
  isPublic: boolean
): Game {
  return {
    gid: '',
    options,
    players,
    meta: newGameMeta(creatorUid, isPublic),
    state: newGameState()
  };
}

/**
 * Create a new game object using FieldValues in place of timestamp strings.
 * This allows for FieldValue objects to be used while maintaining type safety.
 * Objects created using this method can be used in Firestore operations.
 *
 * @param creatorUid The UID of the player creating the game.
 * @param isPublic A boolean indicating game visibility.
 * @param players A GamePlayers object.
 * @param options A GameOptions object.
 */
export function newGameWithFieldValues(
  creatorUid: string,
  isPublic: boolean,
  players: GamePlayers,
  options: GameOptions
): WithFieldValue<Game> {
  return {
    gid: '',
    options,
    players,
    meta: newGameMetaWithFieldValues(creatorUid, isPublic),
    state: newGameState()
  };
}

/**
 * Create a new partial game object, allowing for FieldValue objects to be used
 * as field values. Objects created using this method can be used in Firestore
 * operations.
 */
export function newPartialGameWithFieldValues(): PartialWithFieldValue<Game> {
  return {};
}

/**
 * Parse game document data from Firestore and build a Game object.
 *
 * @param gid The document ID associated with the game data.
 * @param data A DocumentData object from a Firestore query.
 * @return A Game object.
 */
export function parseGameDocument(gid: string, data: DocumentData): Game {
  return {
    gid: gid,
    meta: meta.parseGameMetaDocument(data.meta),
    options: options.parseGameOptionsDocument(data.options),
    players: players.parseGamePlayersDocument(data.players),
    state: state.parseGameStateDocument(data.state)
  };
}

/**
 * Get the meta data from a Game object.
 *
 * @param game A Game object.
 */
export function getGameMeta(game: Game) {
  return game.meta;
}

/**
 * Get the options data from a Game object.
 *
 * @param game A game object.
 */
export function getGameOptions(game: Game) {
  return game.options;
}

/**
 * Get the player data from a Game object.
 *
 * @param game A Game object.
 */
export function getGamePlayers(game: Game) {
  return game.players;
}

/**
 * Get the game state data from a Game object.
 *
 * @param game A Game object.
 */
export function getGameState(game: Game) {
  return game.state;
}

/**
 * Get the flag indicating whether the game is visible to the public.
 *
 * @param game A Game object.
 * @return true if the game is visible to the public, false otherwise.
 */
export function getGameIsPublic(game: Game): boolean {
  return meta.getGameIsPublic(game.meta);
}

/**
 * Get the UID of the player who created the game.
 *
 * @param game A Game object.
 * @return The UID of the player who created the game.
 */
export function getGameCreatorUid(game: Game): string {
  return meta.getGameCreatorUid(game.meta);
}

/**
 * Get the flag indicating whether the game has started.
 *
 * @param game A Game object.
 * @return true if the game has started, otherwise false.
 */
export function getGameIsStarted(game: Game): boolean {
  return meta.getGameIsStarted(game.meta);
}

/**
 * Get the flag indicating whether game has ended.
 *
 * @param game A Game object.
 * @return true if the game has ended, otherwise false.
 */
export function getGameIsEnded(game: Game): boolean {
  return meta.getGameIsEnded(game.meta);
}

/**
 * Get the end result of the game.
 *
 * @param game A Game object.
 * @return The string "draw" if the game ended in a draw, "tie" if the game
 * ended in a tie, the uid of the winning player, or the empty string if the
 * game is not over.
 */
export function getGameResult(game: Game): string {
  return meta.getGameResult(game.meta);
}

/**
 * Get the Date of the game creation (when the invitation was issued).
 *
 * @param game A Game object.
 * @return The Date the game was created, or null if the invitation has not been sent.
 */
export function getGameCreatedDate(game: Game): Date | null {
  return meta.getGameCreatedDate(game.meta);
}

/**
 * Get the Date of the start of the game (when the invited played accepted the
 * invitation).
 *
 * @param game A Game object.
 * @return The Date of the start of the game, or null if the game invitation has
 * not yet been accepted.
 */
export function getGameStartedDate(game: Game): Date | null {
  return meta.getGameStartedDate(game.meta);
}

/**
 * Get the Date of the last played move.
 *
 * @param game A Game object.
 * @return The Date of the last played move, or null if no move has been played.
 */
export function getLastPlayDate(game: Game): Date | null {
  return meta.getLastPlayDate(game.meta);
}

/**
 * Get the Date that the game ended.
 *
 * @param game A Game object.
 * @return The Date the game ended, or null if the game is not over.
 */
export function getGameEndedDate(game: Game): Date | null {
  return meta.getGameEndedDate(game.meta);
}

/**
 * Get the flag indicating whether the ladybug expansion is used.
 *
 * @param game A Game object.
 * @return true if the ladybug expansion is used, otherwise false.
 */
export function getIsLadybugUsed(game: Game): boolean {
  return options.getIsLadybugUsed(game.options);
}

/**
 * Get the flag indicating whether the mosquito expansion is used.
 *
 * @param game A Game object.
 * @return true if the mosquito expansion is used, otherwise false.
 */
export function getIsMosquitoUsed(game: Game): boolean {
  return options.getIsMosquitoUsed(game.options);
}

/**
 * Get the flag indicating whether the pillbug expansion is used.
 *
 * @param game A Game object.
 * @return true if the pillbug expansion is used, otherwise false.
 */
export function getIsPillbugUsed(game: Game): boolean {
  return options.getIsPillbugUsed(game.options);
}

/**
 * Get the flag indicating whether the tournament opening rule is used.
 *
 * @param game A Game object.
 * @return true if the tournament opening rule is used, otherwise false.
 */
export function getIsTournamentRuleUsed(game: Game): boolean {
  return options.getIsTournamentRuleUsed(game.options);
}

/**
 * Get the UID of the player playing as black.
 *
 * @param game A Game object.
 * @return The UID of the player playing as black.
 */
export function getBlackUid(game: Game): string {
  return players.getBlackUid(game.players);
}

/**
 * Get the UserData object of the player playing as black.
 *
 * @param game A Game object.
 * @return The UserData object of the player playing as black.
 */
export function getBlackUserData(game: Game): UserData {
  return players.getBlackUserData(game.players);
}

/**
 * Get the username of the player playing as black
 *
 * @param game A Game object.
 * @return The username of the player playing as black.
 */
export function getBlackUsername(game: Game): string {
  return players.getBlackUsername(game.players);
}

/**
 * Get the UID of the player playing as white.
 *
 * @param game A Game object.
 * @return The UID of the player playing as white.
 */
export function getWhiteUid(game: Game): string {
  return players.getWhiteUid(game.players);
}

/**
 * Get the UserData object of the player playing as white.
 *
 * @param game A Game object.
 * @return The UserData object of the player playing as white.
 */
export function getWhiteUserData(game: Game): UserData {
  return players.getWhiteUserData(game.players);
}

/**
 * Get the username of the player playing as white
 *
 * @param game A Game object.
 * @return The username of the player playing as white.
 */
export function getWhiteUsername(game: Game): string {
  return players.getWhiteUsername(game.players);
}

/**
 * Get the game notation string from a game state object.
 *
 * @param game A Game object.
 * @return A game notation string.
 */
export function getGameNotation(game: Game) {
  return state.getGameNotation(game.state);
}

/**
 * Get the color of the player whose turn it is from a game state object.
 *
 * @param game A Game object.
 * @return A player color.
 */
export function getColorTurn(game: Game) {
  return state.getColorTurn(game.state);
}

/**
 * Get the total number of moves that have been played in the game defined by
 * the game notation string in a game state object.
 *
 * @param game A Game object.
 * @return The total number of moves that have been played
 */
export function getMoveCount(game: Game): number {
  return state.getMoveCount(game.state);
}

/**
 * Get the color of the opponent of the given user.
 *
 * @param game A Game object.
 * @param uid The UID of the player whose opponent's color will be returned.
 * @return The color of the given player's opponent, or null if the given UID is
 * not a player in the game.
 */
export function getOpponentColor(game: Game, uid: string): ColorKey | null {
  const opponent = getOpponentUid(game, uid);
  return opponent ? getPlayerColor(game, opponent) : null;
}

/**
 * Get the UID of the opponent of the given user.
 *
 * @param game A Game object.
 * @param uid The UID of the player whose opponent will be returned.
 * @return The UID of the given player's opponent, or null if the given UID is
 * not a player in the game.
 */
export function getOpponentUid(game: Game, uid: string): string | null {
  if (!getGamePlayers(game).uids.includes(uid)) return '';
  const blackUid = getBlackUid(game);
  const whiteUid = getWhiteUid(game);
  if (uid === blackUid) return whiteUid;
  if (uid === whiteUid) return blackUid;
  return null;
}

/**
 * Get the username of the opponent of the given user.
 *
 * @param game A Game object.
 * @param uid The UID of the player whose opponent will be returned.
 * @return The username of the given player's opponent, or null if the given UID
 * is not a player in the game.
 */
export function getOpponentUsername(game: Game, uid: string): string | null {
  const opponentUid = getOpponentUid(game, uid);
  return opponentUid ? getPlayerUsername(game, opponentUid) : null;
}

/**
 * Get the color of a player in the game.
 *
 * @param game A Game object.
 * @param uid The UID of the player whose color will be returned.
 * @return The color of the player with the given UID, or null if the player is
 * not a player in the game.
 */
export function getPlayerColor(game: Game, uid: string): ColorKey | null {
  if (!getGamePlayers(game).uids.includes(uid)) return null;
  return uid === getBlackUid(game) ? 'b' : 'w';
}

/**
 * Get the result of a game for a player in the game.
 *
 * @param game A Game object.
 * @param uid The UID of the player to retrieve results for.
 * @return The empty string if the player is not in the game, the letter 'T' if
 * the game ended in a tie, the letter 'D' if the game ended in a draw, the
 * letter 'W' if the given player won the game, the letter 'L' if the given
 * player lost the game, otherwise the empty string.
 */
export function getPlayerResult(game: Game, uid: string): string {
  if (!getPlayerUids(game).includes(uid)) return '';

  const result = getGameResult(game);
  switch (result) {
    case 'tie':
      return 'T';
    case 'draw':
      return 'D';
    case '':
      return '';
  }
  return uid === result ? 'W' : 'L';
}

/**
 * Get the UIDs of both players in the game.
 *
 * @param game A Game object.
 * @return A list of player UIDs.
 */
export function getPlayerUids(game: Game): string[] {
  return players.getPlayerUids(game.players);
}

/**
 * Get the username of one of the game's players from their UID.
 *
 * @param game A game object.
 * @param uid A player's UID.
 * @return The given player's username, or the empty string if the UID is not a player in the game.
 */
export function getPlayerUsername(game: Game, uid: string): string {
  const players = getGamePlayers(game);
  if (players.black.uid === uid) return players.black.username;
  if (players.white.uid === uid) return players.white.username;
  return '';
}

/**
 * Get the UID of the player whose turn it is.
 *
 * @param game A Game object
 * @return The UID of the player whose turn it is.
 */
export function getTurnUid(game: Game): string {
  const colorTurn = getColorTurn(game);
  switch (colorTurn) {
    case 'b':
      return getBlackUid(game);
    case 'w':
      return getWhiteUid(game);
  }
}

/**
 * Determine if the user with the given UID is the creator of the given game.
 *
 * @param game A Game object.
 * @param uid A UID.
 * @return true if the user with the given UID created the game, otherwise false.
 */
export function getUserCreatedGame(game: Game, uid: string): boolean {
  return getGameCreatorUid(game) === uid;
}
