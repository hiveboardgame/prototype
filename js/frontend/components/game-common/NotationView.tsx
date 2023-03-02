import {
  As,
  forwardRef,
  Grid,
  GridItem,
  GridItemProps,
  GridProps,
  Text
} from '@chakra-ui/react';
import { Move } from 'hive-lib';
import { PropsWithChildren } from 'react';

interface NotationViewProps {
  moves: Move[];
  upTo: number;
  onClickMove?: (index: number, move: Move) => void;
  onHoverMove?: (index: number, move: Move) => void;
}

const Header = (props: PropsWithChildren<any>) => {
  return (
    <GridItem>
      <Text fontWeight='bold'>{props.children}</Text>
    </GridItem>
  );
};

const Cell = forwardRef<GridItemProps, As>((props, ref) => {
  const { children, ...rest } = props;
  return (
    <GridItem py={'1px'} ref={ref} {...rest}>
      <Text userSelect='none' fontFamily='monospace'>
        {children}
      </Text>
    </GridItem>
  );
});

const NotationView = forwardRef<GridProps & NotationViewProps, As>(
  (props, ref) => {
    const { moves, upTo, onClickMove, onHoverMove, ...rest } = props;
    return (
      <Grid templateColumns='1fr 2fr 2fr' ref={ref} {...rest}>
        <Header>Turn</Header>
        <Header>White Move</Header>
        <Header>Black Move</Header>
        {moves.map((move, index) => {
          const color = index >= upTo ? 'gray.400' : undefined;
          const onMouseEnter = () => {
            if (onHoverMove) onHoverMove(index, move);
          };
          const onClick = () => {
            if (onClickMove) onClickMove(index, move);
          };

          if (index % 2 === 0) {
            const turn = Math.floor(index / 2) + 1;
            return [
              <Cell key={`turn-${turn}`} color={color}>
                {turn}.
              </Cell>,
              <Cell
                key={`move-${index}`}
                color={color}
                onMouseEnter={onMouseEnter}
                onClick={onClick}
              >
                {move.notation}
              </Cell>
            ];
          } else {
            return (
              <Cell
                key={`move=${index}`}
                color={color}
                onMouseEnter={onMouseEnter}
                onClick={onClick}
              >
                {move.notation}
              </Cell>
            );
          }
        })}
      </Grid>
    );
  }
);

export { NotationView };
