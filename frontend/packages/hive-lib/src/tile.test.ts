import {
  getBugLetter,
  getBugNumber,
  getTileBug,
  getTileColor,
  isOwnTile
} from './tile';
import { BLACK_TILES, WHITE_TILES } from './types';

describe('bug metadata extraction', () => {
  describe('getBugLetter', () => {
    test('ant 1', () => expect(getBugLetter('A1')).toBe('A'));
    test('ant 2', () => expect(getBugLetter('A2')).toBe('A'));
    test('ant 3', () => expect(getBugLetter('A3')).toBe('A'));
    test('beetle 1', () => expect(getBugLetter('B1')).toBe('B'));
    test('beetle 2', () => expect(getBugLetter('B2')).toBe('B'));
    test('grasshopper 1', () => expect(getBugLetter('G1')).toBe('G'));
    test('grasshopper 2', () => expect(getBugLetter('G2')).toBe('G'));
    test('grasshopper 3', () => expect(getBugLetter('G3')).toBe('G'));
    test('spider 1', () => expect(getBugLetter('S1')).toBe('S'));
    test('spider 2', () => expect(getBugLetter('S2')).toBe('S'));
    test('queen', () => expect(getBugLetter('Q')).toBe('Q'));
    test('ladybug', () => expect(getBugLetter('L')).toBe('L'));
    test('mosquito', () => expect(getBugLetter('M')).toBe('M'));
    test('pillbug', () => expect(getBugLetter('P')).toBe('P'));
  });
  describe('getBugNumber', () => {
    test('ant 1', () => expect(getBugNumber('A1')).toBe(1));
    test('ant 2', () => expect(getBugNumber('A2')).toBe(2));
    test('ant 3', () => expect(getBugNumber('A3')).toBe(3));
    test('beetle 1', () => expect(getBugNumber('B1')).toBe(1));
    test('beetle 2', () => expect(getBugNumber('B2')).toBe(2));
    test('grasshopper 1', () => expect(getBugNumber('G1')).toBe(1));
    test('grasshopper 2', () => expect(getBugNumber('G2')).toBe(2));
    test('grasshopper 3', () => expect(getBugNumber('G3')).toBe(3));
    test('spider 1', () => expect(getBugNumber('S1')).toBe(1));
    test('spider 2', () => expect(getBugNumber('S2')).toBe(2));
    test('queen', () => expect(getBugNumber('Q')).toBeUndefined());
    test('ladybug', () => expect(getBugNumber('L')).toBeUndefined());
    test('mosquito', () => expect(getBugNumber('M')).toBeUndefined());
    test('pillbug', () => expect(getBugNumber('P')).toBeUndefined());
  });
  describe('getTileBug', () => {
    test('white ant 1', () => expect(getTileBug('wA1')).toBe('A1'));
    test('white ant 2', () => expect(getTileBug('wA2')).toBe('A2'));
    test('white ant 3', () => expect(getTileBug('wA3')).toBe('A3'));
    test('white beetle 1', () => expect(getTileBug('wB1')).toBe('B1'));
    test('white beetle 2', () => expect(getTileBug('wB2')).toBe('B2'));
    test('white grasshopper 1', () => expect(getTileBug('wG1')).toBe('G1'));
    test('white grasshopper 2', () => expect(getTileBug('wG2')).toBe('G2'));
    test('white grasshopper 3', () => expect(getTileBug('wG3')).toBe('G3'));
    test('white spider 1', () => expect(getTileBug('wS1')).toBe('S1'));
    test('white spider 2', () => expect(getTileBug('wS2')).toBe('S2'));
    test('white queen', () => expect(getTileBug('wQ')).toBe('Q'));
    test('white ladybug', () => expect(getTileBug('wL')).toBe('L'));
    test('white mosquito', () => expect(getTileBug('wM')).toBe('M'));
    test('white pillbug', () => expect(getTileBug('wP')).toBe('P'));
    test('black ant 1', () => expect(getTileBug('bA1')).toBe('A1'));
    test('black ant 2', () => expect(getTileBug('bA2')).toBe('A2'));
    test('black ant 3', () => expect(getTileBug('bA3')).toBe('A3'));
    test('black beetle 1', () => expect(getTileBug('bB1')).toBe('B1'));
    test('black beetle 2', () => expect(getTileBug('bB2')).toBe('B2'));
    test('black grasshopper 1', () => expect(getTileBug('bG1')).toBe('G1'));
    test('black grasshopper 2', () => expect(getTileBug('bG2')).toBe('G2'));
    test('black grasshopper 3', () => expect(getTileBug('bG3')).toBe('G3'));
    test('black spider 1', () => expect(getTileBug('bS1')).toBe('S1'));
    test('black spider 2', () => expect(getTileBug('bS2')).toBe('S2'));
    test('black queen', () => expect(getTileBug('bQ')).toBe('Q'));
    test('black ladybug', () => expect(getTileBug('bL')).toBe('L'));
    test('black mosquito', () => expect(getTileBug('bM')).toBe('M'));
    test('black pillbug', () => expect(getTileBug('bP')).toBe('P'));
  });
  describe('getTileColor', () => {
    test.each(WHITE_TILES)('%s', (tile) =>
      expect(getTileColor(tile)).toBe('w')
    );
    test.each(BLACK_TILES)('%s', (tile) =>
      expect(getTileColor(tile)).toBe('b')
    );
  });
  describe('isOwnTile', () => {
    test.each(WHITE_TILES)('%s, w', (tile) =>
      expect(isOwnTile(tile, 'w')).toBe(true)
    );
    test.each(WHITE_TILES)('%s, b', (tile) =>
      expect(isOwnTile(tile, 'b')).toBe(false)
    );
    test.each(BLACK_TILES)('%s, w', (tile) =>
      expect(isOwnTile(tile, 'w')).toBe(false)
    );
    test.each(BLACK_TILES)('%s, b', (tile) =>
      expect(isOwnTile(tile, 'b')).toBe(true)
    );
  });
});
