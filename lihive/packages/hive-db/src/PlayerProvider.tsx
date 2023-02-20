import { getAuth, User as FirebaseUser } from '@firebase/auth';
import app from './db/app';
import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useState
} from 'react';
import { useRouter } from 'next/router';
import { UserData } from './user/user';
import {
  createUserWithEmailAndPassword,
  GoogleAuthProvider,
  signInWithEmailAndPassword,
  signInWithPopup,
  signOut
} from 'firebase/auth';
import { FirestoreError } from '@firebase/firestore-types';
import { watchUser } from './db/watchUser';
import { Game, getGameIsEnded, getGameIsStarted } from './game/game';
import { watchUserGames } from './db/watchUserGames';

export interface PlayerContextProps {
  uid: string | null;
  user: UserData | null;
  incompleteProfile: boolean;
  invitations: Game[];
  activeGames: Game[];
  completedGames: Game[];
  signInWithEmail: (email: string, password: string) => Promise<void>;
  signInWithGoogle: () => Promise<void>;
  signUpWithEmail: (email: string, password: string) => Promise<void>;
  signout: (redirect: string) => Promise<void>;
}

const auth = getAuth(app);
const playerContext = createContext<PlayerContextProps>(defaultPlayerContext());

const PlayerProvider = ({ children }: { children?: ReactNode }) => {
  const playerState = usePlayerState();
  return (
    <playerContext.Provider value={playerState}>
      {children}
    </playerContext.Provider>
  );
};

const usePlayer = () => {
  return useContext(playerContext);
};

function usePlayerState(): PlayerContextProps {
  const [uid, setUid] = useState<string | null>(null);
  const [user, setUser] = useState<UserData | null>(null);
  const [incompleteProfile, setIncompleteProfile] = useState<boolean>(false);
  const [invitations, setInvitations] = useState<Game[]>([]);
  const [activeGames, setActiveGames] = useState<Game[]>([]);
  const [completedGames, setCompletedGames] = useState<Game[]>([]);
  const router = useRouter();

  /**
   * Handle an error from Firebase.
   */
  const handleFirebaseError = useCallback((error: FirestoreError) => {
    console.error(error.message);
  }, []);

  /**
   * Handle a change in the user's data.
   */
  const handleFirebaseUser = useCallback(
    (firebaseUser: FirebaseUser | null) => {
      setUid(firebaseUser ? firebaseUser.uid : null);
    },
    []
  );

  /**
   * Handle a change to the player's games
   */
  const handleGamesChanged = useCallback((games: Game[]) => {
    const activeGames = games.filter(
      (game) => getGameIsStarted(game) && !getGameIsEnded(game)
    );
    const completedGames = games.filter((game) => getGameIsEnded(game));
    const invitations = games.filter((game) => !getGameIsStarted(game));
    setActiveGames(activeGames);
    setCompletedGames(completedGames);
    setInvitations(invitations);
  }, []);

  /**
   * Handle a change to the user's data.
   */
  const handleUserDataChanged = useCallback(
    (user: UserData | null) => {
      setIncompleteProfile(user === null && uid !== null);
      setUser(user);
    },
    [uid]
  );

  /**
   * Sign in using Google.
   */
  const signInWithGoogle = () => {
    const provider = new GoogleAuthProvider();
    provider.setCustomParameters({
      prompt: 'select_account'
    });
    return signInWithPopup(auth, provider)
      .then((creds) => creds.user)
      .then(handleFirebaseUser);
  };

  /**
   * Sign in using an email and password.
   *
   * @param email
   * @param password
   */
  const signInWithEmail = (email: string, password: string) => {
    return signInWithEmailAndPassword(auth, email, password)
      .then((creds) => creds.user)
      .then(handleFirebaseUser);
  };

  /**
   * Sign up using an email address.
   *
   * @param email
   * @param password
   */
  const signUpWithEmail = (email: string, password: string) => {
    return createUserWithEmailAndPassword(auth, email, password)
      .then((creds) => creds.user)
      .then(handleFirebaseUser);
  };

  /**
   * Sign out the current user and optionally redirect to a page.
   * @param redirect The page to redirect to after sign-out.
   */
  const signout = (redirect?: string) => {
    return signOut(auth)
      .then(() => {
        setUid(null);
        setUser(null);
        setIncompleteProfile(false);
        setActiveGames([]);
        setCompletedGames([]);
        setInvitations([]);
        if (redirect) router.push(redirect);
      })
      .catch((error) => {
        console.error(error);
      });
  };

  // Listen for auth state changes
  useEffect(() => {
    const unsubscribe = auth.onAuthStateChanged(handleFirebaseUser);
    return () => {
      unsubscribe();
    };
  }, [handleFirebaseUser]);

  /**
   * Listen for changes to the user's data
   */
  useEffect(() => {
    const unsubCallbacks: (() => void)[] = [];
    if (uid) {
      unsubCallbacks.push(
        watchUser(uid, handleUserDataChanged, handleFirebaseError)
      );
      unsubCallbacks.push(
        watchUserGames(uid, handleGamesChanged, handleFirebaseError)
      );
    }
    return () => unsubCallbacks.forEach((unsub) => unsub());
  }, [handleUserDataChanged, handleFirebaseError, handleGamesChanged, uid]);

  return {
    uid,
    user,
    incompleteProfile,
    activeGames,
    completedGames,
    invitations,
    signInWithEmail,
    signInWithGoogle,
    signUpWithEmail,
    signout
  };
}

function defaultPlayerContext(): PlayerContextProps {
  const message = 'Player context not properly initialized.';
  return {
    uid: null,
    user: null,
    incompleteProfile: false,
    activeGames: [],
    completedGames: [],
    invitations: [],
    signInWithEmail: () => Promise.reject(message),
    signInWithGoogle: () => Promise.reject(message),
    signUpWithEmail: () => Promise.reject(message),
    signout: () => Promise.reject(message)
  };
}

export { PlayerProvider, usePlayer };
