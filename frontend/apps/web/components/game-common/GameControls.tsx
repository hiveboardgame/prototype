import {
  As,
  ButtonGroup,
  Center,
  Flex,
  FlexProps,
  forwardRef,
  IconButton,
  keyframes
} from '@chakra-ui/react';
import {
  BiArrowToLeft,
  BiArrowToRight,
  BiLeftArrowAlt,
  BiRightArrowAlt
} from 'react-icons/bi';

interface GameControlsProps {
  pulseLast?: boolean;
  totalMoves?: number;
  viewUpTo?: number;
  onClickFirst?: () => void;
  onClickLast?: () => void;
  onClickNext?: () => void;
  onClickPrevious?: () => void;
  onClickCenter?: () => void;
  onClickSettings?: () => void;
}

const pulse = keyframes`
  from { background-color: #f8a61c }
  to { background-color: white}
`;
const pulseAnimation = `${pulse} infinite 1.5s linear alternate`;

const GameControls = forwardRef<FlexProps & GameControlsProps, As>(
  (props, ref) => {
    const {
      pulseLast,
      totalMoves,
      viewUpTo,
      onClickFirst,
      onClickLast,
      onClickNext,
      onClickPrevious,
      ...rest
    } = props;
    return (
      <Flex ref={ref} {...rest}>
        <ButtonGroup width='full' isAttached size='sm' variant='solid'>
          <IconButton
            aria-label='First Move'
            borderRadius={0}
            icon={<BiArrowToLeft />}
            isDisabled={!onClickFirst || viewUpTo === 0}
            onClick={onClickFirst}
          />
          <IconButton
            aria-label='Previous Move'
            borderRadius={0}
            icon={<BiLeftArrowAlt />}
            isDisabled={!onClickPrevious || viewUpTo === 0}
            onClick={onClickPrevious}
          />
          <Center width='full' px={2} bg='gray.100' fontSize='sm'>
            Move {viewUpTo}/{totalMoves}
          </Center>
          <IconButton
            aria-label='Next Move'
            borderRadius={0}
            icon={<BiRightArrowAlt />}
            isDisabled={!onClickNext || viewUpTo === totalMoves}
            onClick={onClickNext}
          />
          <IconButton
            aria-label='Last Move'
            borderRadius={0}
            icon={<BiArrowToRight />}
            animation={pulseLast ? pulseAnimation : undefined}
            isDisabled={!onClickLast || viewUpTo === totalMoves}
            onClick={onClickLast}
          />
        </ButtonGroup>
      </Flex>
    );
  }
);

export { GameControls };
