import { HTMLAttributes } from 'react';

const Header = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <div className={`contents prose max-w-none ${className || ''}`} {...rest} />
  );
};

const HeaderItem = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <div
      className={`
        flex items-center prose prose-sm font-bold truncate px-2 py-3 border-b ${
          className || ''
        }`}
      {...rest}
    />
  );
};

export { Header, HeaderItem };
