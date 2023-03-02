import { getValidAntMoveCoordinates } from './ant';
import ant0 from './data/ant/ant0';
import ant1 from './data/ant/ant1';
import ant2 from './data/ant/ant2';
import ant3 from './data/ant/ant3';
import ant4 from './data/ant/ant4';

describe('ant moves', () => {
  const ants = [
    { index: 0, ...ant0 },
    { index: 1, ...ant1 },
    { index: 2, ...ant2 },
    { index: 3, ...ant3 },
    { index: 4, ...ant4 }
  ];

  test.each(ants)(
    'test position $index',
    ({ board, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidAntMoveCoordinates(
        board,
        tileCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});
