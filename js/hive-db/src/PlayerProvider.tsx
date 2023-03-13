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
import { GameChallenge, getUserChallenges } from './game/challenge';
import { createGuestUser, createUser, getUser } from '..';

export interface PlayerContextProps {
  user: UserData | null;
  incompleteProfile: boolean;
  activeChallenges: GameChallenge[];
  newChallenge: (challenge: GameChallenge) => void,
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
  const [firebaseUser, setFirebaseUser] = useState<FirebaseUser | null>(null);
  const [incompleteProfile, setIncompleteProfile] = useState<boolean>(false);
  const [activeChallenges, setActiveChallenges] = useState<GameChallenge[]>([]);
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
        setActiveGames(activeGames);
        setCompletedGames(completedGames);
      });
    getUserChallenges(user)
      .then(setActiveChallenges);
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

  onAuthStateChanged(auth, setFirebaseUser)

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
        setActiveChallenges([]);
        if (redirect) { /* router.push(redirect) */ }
      })
      .catch((error) => {
        console.error(error);
      });
  };

  function newChallenge(challenge: GameChallenge) {
    setActiveChallenges([challenge].concat(activeChallenges));
  }

  return {
    user,
    incompleteProfile,
    activeGames,
    completedGames,
    newChallenge,
    activeChallenges,
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
    activeChallenges: [],
    newChallenge: (_) => Promise.reject(message),
    usernameChanged: (_) => Promise.reject(message),
    signInWithGoogle: () => Promise.reject(message),
    signInAsGuest: () => Promise.reject(message),
    signout: () => Promise.reject(message)
  };
}

export { PlayerProvider, usePlayer };
