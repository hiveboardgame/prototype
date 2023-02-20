import {
  As,
  Flex,
  FlexProps,
  Text,
  forwardRef,
  Box,
  BoxProps
} from '@chakra-ui/react';
import { UserData } from 'hive-db';
import { ColorKey, TileId } from 'hive-lib';
import { useGameSelector } from '../../state/game-online/hooks';
import {
  selectBlackPlayer,
  selectColorTurn,
  selectDisplayBlackHand,
  selectDisplayWhiteHand,
  selectWhitePlayer
} from '../../state/game-online/selectors';
import { HiveIcon } from '../common/HiveIcon';
import { HandGrid } from './HandGrid';

interface UserBadgeProps {
  user: UserData;
  isTurn: boolean;
  tileColor: ColorKey;
}

const UserBadge = forwardRef<FlexProps & UserBadgeProps, As>((props, ref) => {
  const { user, isTurn, tileColor, ...rest } = props;
  const hexStroke = isTurn ? '#f8a61c' : '#222';
  const hexFill = tileColor === 'w' ? 'white' : hexStroke;
  const fontWeight = isTurn ? 'bold' : 'normal';
  return (
    <Flex ref={ref} {...rest}>
      <HiveIcon
        className='mr-2'
        width={24}
        height={24}
        stroke={hexStroke}
        fill={hexFill}
      />
      <Text fontWeight={fontWeight} color={hexStroke}>
        {user.username}
      </Text>
    </Flex>
  );
});

interface GameHandProps {
  user: UserData;
  isTurn: boolean;
  tileColor: ColorKey;
  stacks: TileId[][];
}

const GameHand = (props: GameHandProps) => {
  const { user, isTurn, tileColor, stacks } = props;
  return (
    <Box mb={4}>
      <UserBadge mb={2} user={user} tileColor={tileColor} isTurn={isTurn} />
      <HandGrid stacks={stacks} />
    </Box>
  );
};

const GameOnlineHands = forwardRef<BoxProps, As>((props, ref) => {
  const blackPlayer = useGameSelector(selectBlackPlayer);
  const whitePlayer = useGameSelector(selectWhitePlayer);
  const blackHand = useGameSelector(selectDisplayBlackHand);
  const whiteHand = useGameSelector(selectDisplayWhiteHand);
  const colorTurn = useGameSelector(selectColorTurn);
  return (
    <Box ref={ref} {...props}>
      {whitePlayer && (
        <GameHand
          user={whitePlayer}
          isTurn={colorTurn === 'w'}
          tileColor={'w'}
          stacks={whiteHand}
        />
      )}
      {blackPlayer && (
        <GameHand
          user={blackPlayer}
          isTurn={colorTurn === 'b'}
          tileColor={'b'}
          stacks={blackHand}
        />
      )}
    </Box>
  );
});

export { GameOnlineHands };
