import { getAuth, User as FirebaseUser, onAuthStateChanged } from '@firebase/auth';
import app from './db/app';
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState
} from 'react';
import { UserData } from './user/user';
import {
  GoogleAuthProvider,
  signInWithPopup,
  signInAnonymously,
  signOut
} from 'firebase/auth';
import { Game, getGameIsEnded, getGameIsStarted, getUserGames } from './game/game';
import { createGuestUser, createUser, getUser } from '..';

export interface PlayerContextProps {
  isLoading: boolean;
  user: UserData | null;
  authToken: string | null;
  incompleteProfile: boolean;
  activeGames: Game[];
  completedGames: Game[];
  usernameChanged: (username: string) => Promise<void>;
  signInWithGoogle: () => Promise<void>;
  signInAsGuest: () => Promise<void>;
  signout: (redirect: string) => Promise<void>;
}

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
  const auth = getAuth(app);
  const [user, setUser] = useState<UserData | null>(null);
  const [authToken, setAuthToken] = useState<string | null>(null);
  const [firebaseUser, setFirebaseUser] = useState<FirebaseUser | null>(null);
  const [incompleteProfile, setIncompleteProfile] = useState<boolean>(false);
  const [activeGames, setActiveGames] = useState<Game[]>([]);
  const [completedGames, setCompletedGames] = useState<Game[]>([]);

  // FIXME: until we can get handleFirebaseUserChanged to fire more reliably,
  // isLoading will remain unused
  const [isLoading, setIsLoading] = useState<boolean>(false);

  /**
   * Handle a change to the player's games
   */
  useEffect(() => {
    if (user === null) return;
    getUserGames(user)
      .then((games: Game[]) => {
        const activeGames = games.filter(
          (game) => getGameIsStarted(game) && !getGameIsEnded(game)
        );
        const completedGames = games.filter((game) => getGameIsEnded(game));
        setActiveGames(activeGames);
        setCompletedGames(completedGames);
      });
  }, [user]);

  async function usernameChanged(username: string) {
    if (!firebaseUser) {
      return;
    }

    // TODO: better error handling w/ helpful user-facing messages
    setUser(await createUser(username, authToken));
    setIncompleteProfile(false);
  }

  // FIXME: for some reason, this fires twice when an already-logged-in user loads the page,
  // once with firebaseUser set to null and the other with the correct user.
  async function handleFirebaseUserChanged() {
    if (!firebaseUser) {
      return;
    }

    const uid = firebaseUser.uid;
    const isGuest = firebaseUser.isAnonymous;
    const token = await firebaseUser.getIdToken();
    setAuthToken(token);

    // Check if a user already exists for this uid. If so, we're done.
    // Otherwise, either create a guest account or prompt for a username
    const user = await getUser(uid);
    if (user) {
      setUser(user);
    } else if (isGuest) {
      setUser(await createGuestUser(token));
    } else {
      setIncompleteProfile(true);
    }
  }

  useEffect(() => {
    return onAuthStateChanged(auth, setFirebaseUser);
  }, [])

  useEffect(() => {
    handleFirebaseUserChanged();
  }, [firebaseUser])

  /**
   * Sign in using Google.
   */
  const signInWithGoogle = async () => {
    const provider = new GoogleAuthProvider();
    provider.setCustomParameters({
      prompt: 'select_account'
    });
    await signInWithPopup(auth, provider);
  };

  /**
   * Sign in anonymously.
   */
  const signInAsGuest = async () => {
    await signInAnonymously(auth);
  }

  /**
   * Sign out the current user and optionally redirect to a page.
   * @param redirect The page to redirect to after sign-out.
   */
  const signout = (redirect?: string) => {
    return signOut(auth)
      .then(() => {
        setUser(null);
        setFirebaseUser(null);
        setIncompleteProfile(false);
        setActiveGames([]);
        setCompletedGames([]);
        if (redirect) { /* router.push(redirect) */ }
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return {
    isLoading,
    user,
    authToken,
    incompleteProfile,
    activeGames,
    completedGames,
    usernameChanged,
    signInWithGoogle,
    signInAsGuest,
    signout
  };
}

function defaultPlayerContext(): PlayerContextProps {
  const message = 'Player context not properly initialized.';
  return {
    isLoading: false,
    user: null,
    authToken: null,
    incompleteProfile: false,
    activeGames: [],
    completedGames: [],
    usernameChanged: (_) => Promise.reject(message),
    signInWithGoogle: () => Promise.reject(message),
    signInAsGuest: () => Promise.reject(message),
    signout: () => Promise.reject(message)
  };
}

export { PlayerProvider, usePlayer };
