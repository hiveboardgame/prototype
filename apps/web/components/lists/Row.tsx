import { HTMLAttributes } from 'react';

const Row = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, onClick, ...rest } = props;
  return (
    <div
      className={`contents ${onClick ? 'group' : ''} ${className || ''}`}
      onClick={onClick}
      {...rest}
    />
  );
};

const RowItem = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <div
      className={`
        flex items-center prose prose-sm truncate p-2 group-hover:bg-[#f8a61c] group-hover:text-white select-none cursor-default group-hover:cursor-pointer ${
          className || ''
        }`}
      {...rest}
    />
  );
};

const RowLoading = () => {
  return (
    <RowItem className='h-32 col-span-full max-w-none flex items-center justify-center'>
      Loading...
    </RowItem>
  );
};

export { Row, RowItem, RowLoading };
