import { GameOptions } from 'hive-lib';
import { GamePlayers, newGamePlayers } from '../game/players';
import { Game, newGameWithFieldValues } from '../game/game';
import { UserData } from '../user/user';
import { newGameOptions } from '../game/options';

export type ColorChoice = 'Black' | 'White' | 'Random';
export type OpeningChoice = 'Normal' | 'Tournament';
export type VisibilityChoice = 'Public' | 'Private';
export type ExpansionsChoice = Omit<GameOptions, 'tournament'>;

/**
 * Create a game invitation.
 *
 * @param creator The creator of the game.
 * @param opponent The opponent.
 * @param visibility Whether game visibility choice.
 * @param color The game creator's color choice.
 * @param expansions The expansions to include in the game.
 * @param opening The game opening rule.
 * @return A Promise that resolves to a Game document reference.
 */
export function createGameInvitation(
  creator: UserData,
  opponent: UserData,
  visibility: VisibilityChoice,
  color: ColorChoice,
  expansions: ExpansionsChoice,
  opening: OpeningChoice
): Promise<Game> {
  const isPublic = visibility === 'Public';
  const players = createGamePlayers(color, creator, opponent);
  const options = newGameOptions(
    expansions.ladybug,
    expansions.mosquito,
    expansions.pillbug,
    opening === 'Tournament'
  );
  // TODO(wgreenberg): implement invitations
  return Promise.reject('unimplemented');
}

/**
 * Create a GamePlayers object from the game creator's color choice.
 *
 * @param color The color choice of the game creator.
 * @param creator The game creator.
 * @param opponent The opponent.
 */
function createGamePlayers(
  color: ColorChoice,
  creator: UserData,
  opponent: UserData
): GamePlayers {
  if (color === 'Black') return newGamePlayers(creator, opponent);
  if (color === 'White') return newGamePlayers(opponent, creator);
  return pickRandomPlayerColors(creator, opponent);
}

/**
 * Randomly assign colors to two players in a game.
 *
 * @param a A player.
 * @param b Another player.
 */
function pickRandomPlayerColors(a: UserData, b: UserData): GamePlayers {
  return Math.random() < 0.5 ? newGamePlayers(a, b) : newGamePlayers(b, a);
}
