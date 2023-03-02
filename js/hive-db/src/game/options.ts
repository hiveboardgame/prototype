import { GameOptions } from 'hive-lib';

/**
 * Create a new game options object.
 *
 * @param ladybug A boolean indicating whether to include the ladybug expansion.
 * @param mosquito A boolean indicating whether to include the mosquito expansion.
 * @param pillbug A boolean indicating whether to include the pillbug expansion.
 * @param tournament A boolean indicating whether to use the tournament opening rule.
 * @return A GameOptions object.
 */
export const newGameOptions = (
  ladybug: boolean,
  mosquito: boolean,
  pillbug: boolean,
  tournament: boolean
): GameOptions => {
  return {
    ladybug,
    mosquito,
    pillbug,
    tournament
  };
};

/**
 * Get the flag indicating whether the ladybug expansion is used.
 *
 * @param options A GameOptions object.
 * @return true if the ladybug expansion is used, otherwise false.
 */
export const getIsLadybugUsed = (options: GameOptions): boolean =>
  options.ladybug;

/**
 * Get the flag indicating whether the mosquito expansion is used.
 *
 * @param options A GameOptions object.
 * @return true if the mosquito expansion is used, otherwise false.
 */
export const getIsMosquitoUsed = (options: GameOptions): boolean =>
  options.mosquito;

/**
 * Get the flag indicating whether the pillbug expansion is used.
 *
 * @param options A GameOptions object.
 * @return true if the pillbug expansion is used, otherwise false.
 */
export const getIsPillbugUsed = (options: GameOptions): boolean =>
  options.pillbug;

/**
 * Get the flag indicating whether the tournament opening rule is used.
 *
 * @param options A GameOptions object.
 * @return true if the tournament opening rule is used, otherwise false.
 */
export const getIsTournamentRuleUsed = (options: GameOptions): boolean =>
  options.tournament;
