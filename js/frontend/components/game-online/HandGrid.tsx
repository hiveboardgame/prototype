import { As, forwardRef, SimpleGrid, SimpleGridProps } from '@chakra-ui/react';
import { TileId } from 'hive-lib';
import { useCallback } from 'react';
import {
  useGameDispatch,
  useGameSelector
} from '../../state/game-online/hooks';
import { selectSelectedTileId } from '../../state/game-online/selectors';
import { handStackClicked, tableClicked } from '../../state/game-online/slice';
import { TileStack } from '../game-common/TileStack';

interface HandGridProps {
  stacks: TileId[][];
}

const HandGrid = forwardRef<SimpleGridProps & HandGridProps, As>(
  (props, ref) => {
    const { stacks, ...rest } = props;
    const dispatch = useGameDispatch();
    const selectedTileId = useGameSelector(selectSelectedTileId);

    const onClickHandStack = useCallback(
      (stack: TileId[]) => dispatch(handStackClicked(stack)),
      [dispatch]
    );

    const onClickTable = useCallback(
      () => dispatch(tableClicked()),
      [dispatch]
    );

    return (
      <SimpleGrid
        minChildWidth='80px'
        ref={ref}
        {...rest}
        onClick={onClickTable}
      >
        {stacks.map((stack, index) => {
          return (
            <svg key={index} width={80} height={80} viewBox='-40 -40 80 80'>
              <TileStack
                stack={stack}
                hexSize={35}
                tilePadding={0}
                selectedTileId={selectedTileId || undefined}
                onClick={onClickHandStack}
              />
            </svg>
          );
        })}
      </SimpleGrid>
    );
  }
);

export { HandGrid };
