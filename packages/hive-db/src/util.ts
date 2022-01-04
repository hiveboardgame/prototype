import { AuthError } from 'firebase/auth';
import { Game, getLastPlayDate } from './game/game';

export function parseAuthError(error: AuthError) {
  switch (error.code) {
    case 'auth/email-already-in-use':
      return 'Email address already in use';
    case 'auth/user-not-found':
      return 'Email address not found';
    case 'auth/wrong-password':
      return 'Incorrect password';
    case 'auth/weak-password':
      return 'Password is too weak';
    default:
      console.log(error.code);
      return 'Something has gone wrong...';
  }
}

export function sortByLastPlayed(a: Game, b: Game): number {
  const lastA = getLastPlayDate(a) || new Date();
  const lastB = getLastPlayDate(b) || new Date();
  return lastB.getTime() - lastA.getTime();
}
