import { UserData } from '../user/user';
import { DocumentData } from 'firebase/firestore';

export interface GamePlayers {
  // the uids of the players in the game
  uids: string[];
  // the user data of the player playing as black
  black: UserData;
  // the user data of the player playing as white
  white: UserData;
}

/**
 * Create a new game players object.
 *
 * @param black The user data of the player playing as black.
 * @param white The user data of the player playing as white.
 * @return A new GamePlayers object.
 */
export const newGamePlayers = (
  black: UserData,
  white: UserData
): GamePlayers => {
  return {
    uids: [black.uid, white.uid],
    black,
    white
  };
};

/**
 * Parse game player document data from Firestore and build a GamePlayers object.
 *
 * @param data A DocumentData object from a Firestore query.
 * @return A GamePlayers object.
 */
export const parseGamePlayersDocument = (data: DocumentData): GamePlayers => {
  return data as GamePlayers;
};

/**
 * Get the UID of the player playing as black.
 *
 * @param players A GamePlayers object.
 * @return The UID of the player playing as black.
 */
export const getBlackUid = (players: GamePlayers): string => players.black.uid;

/**
 * Get the UserData object of the player playing as black.
 *
 * @param players A GamePlayers object.
 * @return The UserData object of the player playing as black.
 */
export const getBlackUserData = (players: GamePlayers): UserData =>
  players.black;

/**
 * Get the username of the player playing as black
 *
 * @param players A GamePlayers object.
 * @return The username of the player playing as black.
 */
export const getBlackUsername = (players: GamePlayers): string =>
  players.black.username;

/**
 * Get the UIDs of both players in the game.
 *
 * @param players A GamePlayers object.
 * @return A list of player UIDs.
 */
export const getPlayerUids = (players: GamePlayers): string[] => players.uids;

/**
 * Get the UID of the player playing as white.
 *
 * @param players A GamePlayers object.
 * @return The UID of the player playing as white.
 */
export const getWhiteUid = (players: GamePlayers): string => players.white.uid;

/**
 * Get the UserData object of the player playing as white.
 *
 * @param players A GamePlayers object.
 * @return The UserData object of the player playing as white.
 */
export const getWhiteUserData = (players: GamePlayers): UserData =>
  players.white;

/**
 * Get the username of the player playing as white
 *
 * @param players A GamePlayers object.
 * @return The username of the player playing as white.
 */
export const getWhiteUsername = (players: GamePlayers): string =>
  players.white.username;
