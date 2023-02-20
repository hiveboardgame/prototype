import mosquito0 from './data/mosquito/mosquito0';
import mosquito1 from './data/mosquito/mosquito1';
import mosquito2 from './data/mosquito/mosquito2';
import mosquito3 from './data/mosquito/mosquito3';
import mosquito4 from './data/mosquito/mosquito4';
import mosquito5 from './data/mosquito/mosquito5';
import { getValidMosquitoMoveCoordinates } from './mosquito';

describe('mosquito moves', () => {
  const mosquitos = [
    { index: 0, ...mosquito0 },
    { index: 1, ...mosquito1 },
    { index: 2, ...mosquito2 },
    { index: 3, ...mosquito3 },
    { index: 4, ...mosquito4 },
    { index: 5, ...mosquito5 }
  ];

  test.each(mosquitos)(
    'test position $index',
    ({ board, tileId, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidMosquitoMoveCoordinates(
        board,
        tileId,
        tileCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});
