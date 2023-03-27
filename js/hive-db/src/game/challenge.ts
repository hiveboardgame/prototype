import { UserData } from '../user/user';
import { deleteReq, getJSON } from '../api';
import { GameOptions } from 'hive-lib';
import { newGameOptions } from '../game/options';
import { postJSON } from '../api';
import { Game } from './game';
import useSWR, { Fetcher } from 'swr';
import { usePlayer } from '../PlayerProvider';

export interface GameChallengeResponse {
  id: string,
  gameType: string,
  ranked: boolean,
  public: boolean,
  tournamentQueenRule: boolean,
  colorChoice: string,
  createdAt: Date,
  challenger: UserData,
}

export class GameChallenge {
  public id: string;
  public gameType: ExpansionsChoice;
  public ranked: boolean;
  public public: boolean;
  public tournamentQueenRule: boolean;
  public colorChoice: ColorChoice;
  public createdAt: Date;
  public challenger: UserData;

  constructor(res: GameChallengeResponse) {
    Object.assign(this, res);
    this.gameType = gameOptionsFromString(res.gameType);
    this.createdAt = new Date(res.createdAt);
  }

  public getChallengeUrl() {
    return `${window.location.origin}/challenge/${this.id}`;
  }
}

export type ColorChoice = 'Black' | 'White' | 'Random';
export type VisibilityChoice = 'Public' | 'Private';
export type ExpansionsChoice = Omit<GameOptions, 'tournament'>;

/**
 * Create a game invitation.
 *
 * @param visibility Whether game visibility choice.
 * @param color The game creator's color choice.
 * @param expansions The expansions to include in the game.
 * @return A Promise that resolves to a Game document reference.
 */
export async function createGameChallenge(
  visibility: VisibilityChoice,
  colorChoice: ColorChoice,
  expansions: ExpansionsChoice,
  authToken: string,
): Promise<GameChallenge> {
  const isPublic = visibility === 'Public';
  const gameType = gameOptionsToString(newGameOptions(
    expansions.ladybug,
    expansions.mosquito,
    expansions.pillbug
  ));
  const reqBody = {
    public: isPublic,
    ranked: false, // not implemented yet
    tournamentQueenRule: true, // always on for now
    gameType,
    colorChoice,
  };
  let res = await postJSON<GameChallengeResponse>('/api/game/challenge', reqBody, authToken);
  return new GameChallenge(res);
}

export async function getGameChallenge(id: string): Promise<GameChallenge> {
  let res = await getJSON<GameChallengeResponse>(`/api/game/challenge/${id}`);
  if (!res) {
      throw new Error(`No such challenge found`);
  }
  return new GameChallenge(res);
}

export async function acceptGameChallenge(id: string, authToken: string): Promise<Game> {
  return postJSON(`/api/game/challenge/${id}/accept`, {}, authToken);
}

export async function deleteGameChallenge(id: string, authToken: string): Promise<void> {
  await deleteReq(`/api/game/challenge/${id}`, authToken);
}

function gameOptionsToString(opts: GameOptions): string {
  if (opts.ladybug || opts.mosquito || opts.pillbug) {
    const m = opts.mosquito ? 'M' : '';
    const l = opts.ladybug ? 'L' : '';
    const p = opts.pillbug ? 'P' : '';
    return `${m}${l}${p}`;
  } else {
    return 'Base';
  }
}

function gameOptionsFromString(gameType: string): GameOptions {
  const m = gameType.includes('M');
  const l = gameType.includes('L');
  const p = gameType.includes('P');
  return newGameOptions(l, m, p);
}

async function gameChallengesFetcher([uri, authToken]): Promise<GameChallenge[] | null> {
  if (!uri) {
    return null;
  }
  const data = await getJSON<GameChallengeResponse[]>(uri, authToken);
  if (data) {
    return data.map((res) => new GameChallenge(res));
  }
  return null;
}

export function usePlayerChallenges() {
  const { user, authToken } = usePlayer();
  const uri = user ? `/api/user/${user.uid}/challenges` : null;
  let { data: challenges, error, isLoading, mutate } = useSWR<GameChallenge[] | null>([uri, authToken], gameChallengesFetcher);
  if (!error && !isLoading && !challenges) {
    error = new Error(`No challenges found for user ${user}`);
  }

  return {
    challenges,
    error,
    isLoading,
    mutate,
  };
}
