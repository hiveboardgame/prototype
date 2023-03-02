import { getAuth, User as FirebaseUser } from '@firebase/auth';
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
  user: UserData | null;
  incompleteProfile: boolean;
  invitations: Game[];
  activeGames: Game[];
  completedGames: Game[];
  usernameChanged: (username: string) => Promise<void>;
  signInWithGoogle: () => Promise<void>;
  signInAsGuest: () => Promise<void>;
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
  const [user, setUser] = useState<UserData | null>(null);
  const [firebaseUser, setFirebaseUser] = useState<FirebaseUser | null>(null);
  const [incompleteProfile, setIncompleteProfile] = useState<boolean>(false);
  const [invitations, setInvitations] = useState<Game[]>([]);
  const [activeGames, setActiveGames] = useState<Game[]>([]);
  const [completedGames, setCompletedGames] = useState<Game[]>([]);

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
        const invitations = games.filter((game) => !getGameIsStarted(game));
        setActiveGames(activeGames);
        setCompletedGames(completedGames);
        setInvitations(invitations);
      });
  }, [user]);

  async function usernameChanged(username: string) {
    if (!firebaseUser) {
      return;
    }

    // TODO: better error handling w/ helpful user-facing messages
    setUser(await createUser(username));
    setIncompleteProfile(false);
  }

  async function handleFirebaseUserChanged() {
    if (!firebaseUser) {
      return;
    }

    const uid = firebaseUser.uid;
    const isGuest = firebaseUser.isAnonymous;

    // Check if a user already exists for this uid. If so, we're done.
    // Otherwise, either create a guest account or prompt for a username
    const user = await getUser(uid);
    if (user) {
      setUser(user);
    } else if (isGuest) {
      setUser(await createGuestUser());
    } else {
      setIncompleteProfile(true);
    }
  }

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
    const creds = await signInWithPopup(auth, provider);
    setFirebaseUser(creds.user);
  };

  /**
   * Sign in anonymously.
   */
  const signInAsGuest = async () => {
    const creds = await signInAnonymously(auth);
    setFirebaseUser(creds.user);
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
        setInvitations([]);
        if (redirect) { /* router.push(redirect) */ }
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return {
    user,
    incompleteProfile,
    activeGames,
    completedGames,
    invitations,
    usernameChanged,
    signInWithGoogle,
    signInAsGuest,
    signout
  };
}

function defaultPlayerContext(): PlayerContextProps {
  const message = 'Player context not properly initialized.';
  return {
    user: null,
    incompleteProfile: false,
    activeGames: [],
    completedGames: [],
    invitations: [],
    usernameChanged: (_) => Promise.reject(message),
    signInWithGoogle: () => Promise.reject(message),
    signInAsGuest: () => Promise.reject(message),
    signout: () => Promise.reject(message)
  };
}

export { PlayerProvider, usePlayer };
