import { UserData } from '../user/user';
import { deleteReq, getJSON } from '../api';
import { GameOptions } from 'hive-lib';
import { newGameOptions } from '../game/options';
import { postJSON } from '../api';

export interface GameChallenge {
  id: string,
  challengerUid: string,
  gameType: string,
  ranked: boolean,
  public: boolean,
  tournamentQueenRule: boolean,
  colorChoice: string,
  createdAt: Date,
  challengeUrl: string
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
  let res = await postJSON<GameChallengeResponse>('/api/game/challenge', reqBody, true);
  return resToChallenge(res);
}

export async function deleteGameChallenge(id: string): Promise<null> {
    await deleteReq(`/api/game/challenge/${id}`);
    return;
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

function resToChallenge(res: GameChallengeResponse): GameChallenge {
  const fullUrl = `${window.location.origin}${res.challengeUrl}`;
  res.challenge.createdAt = new Date(res.challenge.createdAt);
  return { challengeUrl: fullUrl, ...res.challenge }
}

export interface GameChallengeResponse {
  challengeUrl: string,
  challenge: Omit<GameChallenge, 'challengeUrl'>,
}

export async function getUserChallenges(user: UserData): Promise<GameChallenge[]> {
    let responses = await getJSON<GameChallengeResponse[]>(`/api/user/${user.uid}/challenges`, true);
    return responses.map(resToChallenge);
}
