import { HTMLAttributes } from 'react';
import { RowItem } from '../Row';
import { LadybugIcon } from '../../common/LadybugIcon';
import { MosquitoIcon } from '../../common/MosquitoIcon';
import { PillbugIcon } from '../../common/PillbugIcon';

interface ExpansionsItemProps {
  ladybug: boolean;
  mosquito: boolean;
  pillbug: boolean;
}

const ExpansionsItem = (
  props: HTMLAttributes<HTMLDivElement> & ExpansionsItemProps
) => {
  const { ladybug, mosquito, pillbug, ...rest } = props;
  return (
    <RowItem className='-ml-1' {...rest}>
      {ladybug && (
        <LadybugIcon aria-label='Ladybug' width={24} height={24} className='fill-ladybug' />
      )}
      {mosquito && (
        <MosquitoIcon aria-label='Mosquito' width={24} height={24} className='fill-mosquito' />
      )}
      {pillbug && (
        <PillbugIcon aria-label='Pillbug' width={24} height={24} className='fill-pillbug' />
      )}
    </RowItem>
  );
};

export { ExpansionsItem };
