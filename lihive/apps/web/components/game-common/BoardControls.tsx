import { HTMLAttributes } from 'react';
import { IconButton } from '@chakra-ui/react';
import { MdCenterFocusStrong } from 'react-icons/md';
import { RiSettings3Line } from 'react-icons/ri';

interface BoardControlsProps extends HTMLAttributes<HTMLDivElement> {
  onClickCenter?: () => void;
  onClickSettings?: () => void;
}

const BoardControls = (props: BoardControlsProps) => {
  const { onClickCenter, onClickSettings } = props;
  return (
    <div className='flex flex-col border border-slate-200 rounded bg-white pointer-events-auto'>
      <IconButton
        className='grow'
        aria-label='Re-center Game Board'
        borderRadius={0}
        icon={<MdCenterFocusStrong />}
        size='sm'
        variant='solid'
        onClick={onClickCenter}
      />
      <IconButton
        className='grow'
        aria-label='Game Settings'
        borderRadius={0}
        icon={<RiSettings3Line />}
        disabled={true}
        size='sm'
        variant='solid'
        onClick={onClickSettings}
      />
    </div>
  );
};

export { BoardControls };
