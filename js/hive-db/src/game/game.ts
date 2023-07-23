import type { ColorKey, GameOptions, HexCoordinate } from 'hive-lib';
import type { GameMeta } from './meta';
import type { GamePlayers } from './players';
import type { GameState } from './state';
import * as meta from './meta';
import * as options from './options';
import * as players from './players';
import * as state from './state';
import { UserData } from '../user/user';
import { newGameMeta, newGameMetaWithFieldValues } from './meta';
import { newGameState } from './state';
import { getJSON } from '../api';

// TODO: move this to the right place
export interface BackendGame {
  black_uid: string;
  white_uid: string;
  game_control_history: string;
  game_status: string;
  game_type: string;
  history: string;
  id: string;
  rated: boolean;
  tournament_queen_rule: boolean;
  turn: number;
  moves: string;
  spawns: HexCoordinate[];
}

export interface GameOverview {
  gid: string;
  meta: GameMeta;
  options: GameOptions;
  players: GamePlayers;
  state: GameState;
}

export type Game = GameOverview & {
  validMoves: string[];
  validSpawns: string[];
};

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
): OverviewGame {
  return {
    gid: '',
    options,
    players,
    meta: newGameMeta(creatorUid, isPublic),
    state: newGameState()
  };
}

function getOptionsFromBackendGame(backendGame: BackendGame): GameOptions {
  let pillbug = false;
  let ladybug = false;
  let mosquito = false;
  switch (backendGame.game_type) {
    case 'Base+M':
      mosquito = true;
      break;
    case 'Base+L':
      ladybug = true;
      break;
    case 'Base+P':
      pillbug = true;
      break;
    case 'Base+ML':
      mosquito = true;
      ladybug = true;
      break;
    case 'Base+MP':
      mosquito = true;
      pillbug = true;
      break;
    case 'Base+LP':
      ladybug = true;
      pillbug = true;
      break;
    case 'Base+MLP':
      mosquito = true;
      ladybug = true;
      pillbug = true;
      break;
  }

  return {
    tournament: backendGame.tournament_queen_rule,
    pillbug,
    ladybug,
    mosquito
  };
}

function getPlayersFromBackendGame(backendGame: BackendGame): GamePlayers {
  return {
    uids: [backendGame.black_uid, backendGame.white_uid],
    // TODO: Neel: fix this
    black: {
      uid: backendGame.black_uid,
      username: "black player's username",
      is_guest: false
    },
    white: {
      uid: backendGame.white_uid,
      username: "white player's username",
      is_guest: false
    }
  };
}

function getMetaFromBackendGame(backendGame: BackendGame): GameMeta {
  let isStarted = false;
  let isEnded = false;
  let result = '';
  switch (backendGame.game_status) {
    case 'NotStarted':
      // TODO: Neel: make consistent with backend
      isStarted = true;
      break;
    case 'InProgress':
      isStarted = true;
      break;
    case 'Finished(Winner(b))':
      result = backendGame.black_uid;
    case 'Finished(Winner(w))':
      result = backendGame.white_uid;
    case 'Finished(Draw)':
      result = 'draw';
    case 'Finished(Unknown)':
      // TODO: Neel: what is this? should it (or something else) map to tie?
      isEnded = true;
      break;
    default:
      throw new Error(`unknown game status: ${backendGame.game_status}`);
  }

  // TODO: Neel: fix this
  return {
    public: true,
    creator: backendGame.black_uid,
    isStarted,
    isEnded,
    result,
    createdDate: '',
    acceptedDate: '',
    playedDate: '',
    endedDate: ''
  };
}

function getStateFromBackendGame(backendGame: BackendGame): GameState {
  return newGameState(backendGame.history);
}

function getValidMovesFromBackendGame(
  backendGame: BackendGame
): PossibleMove[] {
  console.log(backendGame.moves);
  return backendGame.moves;
}

function getValidSpawnsFromBackendGame(
  backendGame: BackendGame
): PossibleMove[] {
  console.log(backendGame);
  const reserve =
    backendGame.turn % 2 == 0
      ? backendGame.reserve_white
      : backendGame.reserve_black;
  const colorSymbol = backendGame.turn % 2 == 0 ? 'w' : 'b';
  const spawnablePieces = [];
  for (let [key, value] of reserve) {
    if (value === 0) continue;
    spawnablePieces.push(value);
    console.log(key + ' = ' + value);
  }
  for (const spawn of backendGame.spawns) {
    console.log(spawn);
  }
  return backendGame.spawns;
}

/**
 * Create a new game object from the backend's representation of a game.
 *
 * @param creatorUid The UID of the player creating the game.
 * @param players A GamePlayers object.
 * @param options A GameOptions object.
 * @param isPublic A boolean indicating game visibility.
 */
export function newGameFromBackendGame(backendGame: BackendGame): Game {
  console.log(backendGame);
  const options = getOptionsFromBackendGame(backendGame);
  const players = getPlayersFromBackendGame(backendGame);
  const meta = getMetaFromBackendGame(backendGame);
  const state = getStateFromBackendGame(backendGame);
  const validMoves = getValidMovesFromBackendGame(backendGame);
  const validSpawns = getValidSpawnsFromBackendGame(backendGame);

  return {
    gid: backendGame.id,
    options,
    players,
    meta,
    state,
    validMoves,
    validSpawns
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
): Game {
  return {
    gid: '',
    options,
    players,
    meta: newGameMetaWithFieldValues(creatorUid, isPublic),
    state: newGameState()
  };
}

export function getUserGames(user: UserData): Promise<Game[]> {
  return getJSON<Game[]>(`/api/user/${user.uid}/games`).then((maybeGames) => {
    if (!maybeGames) {
      throw new Error(`no games found for that user`);
    }
    return maybeGames;
  });
}

export function getGame(uid: string): Promise<Game[]> {
  return getJSON<Game[]>(`/api/game/${uid}`);
}

/**
 * Create a new partial game object, allowing for FieldValue objects to be used
 * as field values. Objects created using this method can be used in Firestore
 * operations.
 */
export function newPartialGameWithFieldValues(): Partial<Game> {
  return {};
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
