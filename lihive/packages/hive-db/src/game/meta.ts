import {
  DocumentData,
  PartialWithFieldValue,
  serverTimestamp,
  Timestamp,
  WithFieldValue
} from 'firebase/firestore';

export interface GameMeta {
  // flag indicating whether the game is public
  public: boolean;
  // the uid of the player who created the game
  creator: string;
  // flag indicating whether game invitation has been accepted
  isStarted: boolean;
  // flag indicating whether game has ended
  isEnded: boolean;
  // string indicating the result of the game ([winner uid] | "draw" | "tie")
  result: string;
  // a string from Date.toJSON() representing the date the invitation was created
  createdDate: string;
  // a string from Date.toJSON() representing the date the invitation was accepted and gameplay began
  acceptedDate: string;
  // a string from Date.toJSON() representing the last time a move was played
  playedDate: string;
  // a string from Date.toJSON() representing the date the game ended
  endedDate: string;
}

/**
 * Create a new game metadata object.
 *
 * @param creatorUid The UID of the player creating the game.
 * @param isPublic A boolean indicating game visibility.
 * @return A GameMeta object.
 */
export const newGameMeta = (
  creatorUid: string,
  isPublic: boolean
): GameMeta => {
  return {
    public: isPublic,
    creator: creatorUid,
    isStarted: false,
    isEnded: false,
    result: '',
    createdDate: '',
    acceptedDate: '',
    playedDate: '',
    endedDate: ''
  };
};

/**
 * Create a new game metadata object, allowing for FieldValue objects to be used
 * as field values. Objects created using this method can be used in Firestore
 * operations.
 *
 * @param creatorUid The UID of the player creating the game.
 * @param isPublic A boolean indicating game visibility.
 * @return A GameMeta object.
 */
export const newGameMetaWithFieldValues = (
  creatorUid: string,
  isPublic: boolean
): WithFieldValue<GameMeta> => {
  return {
    public: isPublic,
    creator: creatorUid,
    isStarted: false,
    isEnded: false,
    result: '',
    createdDate: serverTimestamp(),
    acceptedDate: new Timestamp(0, 0),
    playedDate: new Timestamp(0, 0),
    endedDate: new Timestamp(0, 0)
  };
};

/**
 * Create a new partial game metadata object, allowing for FieldValue objects
 * to be used as field values. Objects created using this method can be used in
 * Firestore operations.
 */
export const newPartialGameMetaWithFieldValues =
  (): PartialWithFieldValue<GameMeta> => {
    return {};
  };

/**
 * Parse game meta document data from Firestore and build a GameMeta object.
 *
 * @param data A DocumentData object from a Firestore query.
 * @return A GameMeta object.
 */
export const parseGameMetaDocument = (data: DocumentData): GameMeta => {
  const createdTS = data.createdDate as Timestamp;
  const acceptedTS = data.acceptedDate as Timestamp;
  const playedTS = data.playedDate as Timestamp;
  const endedTS = data.endedDate as Timestamp;
  const unset = new Timestamp(0, 0);
  return {
    public: data.public,
    creator: data.creator,
    isStarted: data.isStarted,
    isEnded: data.isEnded,
    result: data.result,
    createdDate: createdTS.isEqual(unset) ? '' : createdTS.toDate().toJSON(),
    acceptedDate: acceptedTS.isEqual(unset) ? '' : acceptedTS.toDate().toJSON(),
    playedDate: playedTS.isEqual(unset) ? '' : playedTS.toDate().toJSON(),
    endedDate: endedTS.isEqual(unset) ? '' : endedTS.toDate().toJSON()
  };
};

/**
 * Get the flag indicating whether the game is visible to the public.
 *
 * @param meta A GameMeta object.
 * @return true if the game is visible to the public, false otherwise.
 */
export const getGameIsPublic = (meta: GameMeta): boolean => meta.public;

/**
 * Get the UID of the player who created the game.
 *
 * @param meta A GameMeta object.
 * @return The UID of the player who created the game.
 */
export const getGameCreatorUid = (meta: GameMeta): string => meta.creator;

/**
 * Get the flag indicating whether the game has started.
 *
 * @param meta A GameMeta object.
 * @return true if the game has started, otherwise false.
 */
export const getGameIsStarted = (meta: GameMeta): boolean => meta.isStarted;

/**
 * Get the flag indicating whether game has ended.
 *
 * @param meta A GameMeta object.
 * @return true if the game has ended, otherwise false.
 */
export const getGameIsEnded = (meta: GameMeta): boolean => meta.isEnded;

/**
 * Get the end result of the game.
 *
 * @param meta A GameMeta object.
 * @return The string "draw" if the game ended in a draw, "tie" if the game
 * ended in a tie, the uid of the winning player, or the empty string if the
 * game is not over.
 */
export const getGameResult = (meta: GameMeta): string => meta.result;

/**
 * Get the Date of the game creation (when the invitation was issued).
 *
 * @param meta A GameMeta object.
 * @return The Date the game was created, or null if the invitation has not been sent.
 */
export const getGameCreatedDate = (meta: GameMeta): Date | null => {
  return meta.createdDate !== '' ? new Date(meta.createdDate) : null;
};

/**
 * Get the Date of the start of the game (when the invited played accepted the
 * invitation).
 *
 * @param meta A GameMeta object.
 * @return The Date of the start of the game, or null if the game invitation has
 * not yet been accepted.
 */
export const getGameStartedDate = (meta: GameMeta): Date | null => {
  return meta.acceptedDate !== '' ? new Date(meta.acceptedDate) : null;
};

/**
 * Get the Date of the last played move.
 *
 * @param meta A GameMeta object.
 * @return The Date of the last played move, or null if no move has been played.
 */
export const getLastPlayDate = (meta: GameMeta): Date | null => {
  return meta.playedDate !== '' ? new Date(meta.playedDate) : null;
};

/**
 * Get the Date that the game ended.
 *
 * @param meta A GameMeta object.
 * @return The Date the game ended, or null if the game is not over.
 */
export const getGameEndedDate = (meta: GameMeta): Date | null => {
  return meta.endedDate !== '' ? new Date(meta.endedDate) : null;
};
