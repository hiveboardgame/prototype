import { getStacksInHand, getTilesInHand } from './hand';
import { BLACK_TILES, ColorKey, GameOptions, WHITE_TILES } from './types';

describe('player hands', () => {
  describe('getStacksInHand', () => {
    test('all stacks in hand', () => {
      const options: GameOptions = {
        ladybug: true,
        mosquito: true,
        pillbug: true,
        tournament: false
      };
      const blackHand = getStacksInHand({}, 'b', options);
      const whiteHand = getStacksInHand({}, 'w', options);
      expect(blackHand).toHaveLength(8);
      expect(whiteHand).toHaveLength(8);
      expect(blackHand).toEqual(
        expect.arrayContaining([
          expect.arrayContaining(['bA1', 'bA2', 'bA3']),
          expect.arrayContaining(['bB1', 'bB2']),
          expect.arrayContaining(['bG1', 'bG2', 'bG3']),
          expect.arrayContaining(['bS1', 'bS2']),
          expect.arrayContaining(['bQ']),
          expect.arrayContaining(['bL']),
          expect.arrayContaining(['bM']),
          expect.arrayContaining(['bP'])
        ])
      );
      expect(whiteHand).toEqual(
        expect.arrayContaining([
          expect.arrayContaining(['wA1', 'wA2', 'wA3']),
          expect.arrayContaining(['wB1', 'wB2']),
          expect.arrayContaining(['wG1', 'wG2', 'wG3']),
          expect.arrayContaining(['wS1', 'wS2']),
          expect.arrayContaining(['wQ']),
          expect.arrayContaining(['wL']),
          expect.arrayContaining(['wM']),
          expect.arrayContaining(['wP'])
        ])
      );
    });
  });
  describe('getTilesInHand', () => {
    const options: GameOptions = {
      ladybug: true,
      mosquito: true,
      pillbug: true,
      tournament: false
    };
    test('all tiles in hand', () => {
      const blackHand = getTilesInHand({}, 'b', options);
      const whiteHand = getTilesInHand({}, 'w', options);
      expect(blackHand).toHaveLength(BLACK_TILES.length);
      expect(whiteHand).toHaveLength(WHITE_TILES.length);
      expect(blackHand).toEqual(expect.arrayContaining(BLACK_TILES));
      expect(whiteHand).toEqual(expect.arrayContaining(WHITE_TILES));
    });
    test('invalid color returns no tiles', () => {
      const wrongHand = getTilesInHand({}, 'a' as ColorKey, options);
      expect(wrongHand).toHaveLength(0);
    });
  });
});
