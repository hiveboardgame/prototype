import app from './app';
import {
  collection,
  getFirestore,
  QueryDocumentSnapshot
} from 'firebase/firestore';
import { Game, parseGameDocument } from '../game/game';
import { UserData } from '../user/user';

const db = getFirestore(app);
export const gamesCollection = collection(db, 'games').withConverter(
  gameConverter()
);

export const usersCollection = collection(db, 'users').withConverter(
  userConverter()
);

/**
 * A function that generates a Firestore converter for game data documents.
 */
function gameConverter() {
  return {
    toFirestore: (data: Game) => {
      const { gid, ...rest } = data;
      return rest;
    },
    fromFirestore: (snapshot: QueryDocumentSnapshot): Game => {
      const gid = snapshot.id;
      const data = snapshot.data({ serverTimestamps: 'estimate' });
      return parseGameDocument(gid, data);
    }
  };
}

/**
 * Create a Firestore converter for the user collection.
 */
function userConverter() {
  return {
    toFirestore: (data: UserData) => {
      return data;
    },
    fromFirestore: (snapshot: QueryDocumentSnapshot): UserData => {
      return snapshot.data() as UserData;
    }
  };
}
