import { Center, CenterProps, forwardRef } from '@chakra-ui/react';
import { useGameSelector } from '../../state/game-online/hooks';
import { selectDisplayMessage } from '../../state/game-online/selectors';

const GameOnlineMessage = forwardRef<CenterProps, 'div'>((props, ref) => {
  const message = useGameSelector(selectDisplayMessage);
  return (
    <Center p={1.5} bg='gray.100' fontSize='sm' ref={ref} {...props}>
      {message}
    </Center>
  );
});

export { GameOnlineMessage };
