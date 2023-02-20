import { Button, ButtonProps, Flex } from '@chakra-ui/react';
import { useCallback } from 'react';
import { useGameSelector } from '../../state/game-online/hooks';
import { selectDisplayGameBoard } from '../../state/game-online/selectors';

const Btn = (props: ButtonProps) => {
  return <Button size='sm' width='full' mb={1} {...props} />;
};

const DebugPanel = () => {
  const board = useGameSelector(selectDisplayGameBoard);
  const printBoard = useCallback(() => {
    console.log(JSON.stringify(board, null, 2));
  }, [board]);
  return (
    <Flex width='full' direction='column'>
      <Btn onClick={printBoard}>Print Board JSON</Btn>
    </Flex>
  );
};

export { DebugPanel };
