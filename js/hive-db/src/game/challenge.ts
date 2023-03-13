import { UserData } from '../user/user';
import { deleteReq, getJSON } from '../api';
import { GameOptions } from 'hive-lib';
import { newGameOptions } from '../game/options';
import { postJSON } from '../api';
import { Game } from './game';

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
  challenger?: UserData,
}

function initializeChallenge(challenge: GameChallenge, challenger?: UserData) {
  challenge.createdAt = new Date(challenge.createdAt);
  challenge.challengeUrl = `${window.location.origin}/challenge/${challenge.id}`;
  challenge.challenger = challenger;
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
  let challenge = await postJSON<GameChallenge>('/api/game/challenge', reqBody, true);
  initializeChallenge(challenge);
  return challenge;
}

export async function getGameChallenge(id: string): Promise<GameChallenge> {
  let res = await getJSON<GameChallengeWithUser>(`/api/game/challenge/${id}`);
  if (!res) {
      throw new Error(`No such challenge found`);
  }
  const challenge = res.challenge as GameChallenge;
  initializeChallenge(challenge, res.challenger);
  return challenge;
}

export async function acceptGameChallenge(id: string): Promise<Game> {
  return postJSON(`/api/game/challenge/${id}/accept`, {}, true);
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

export interface GameChallengeWithUser {
  challenger: UserData,
  challenge: Omit<GameChallenge, 'challengeUrl' | 'challenger'>,
}

export async function getUserChallenges(user: UserData): Promise<GameChallenge[]> {
    let responses = await getJSON<GameChallenge[]>(`/api/user/${user.uid}/challenges`, true);
    responses.forEach((challenge) => initializeChallenge(challenge));
    return responses;
}
