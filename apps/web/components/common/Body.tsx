import { HTMLAttributes } from 'react';

const Body = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <div
      className={`max-w-screen-lg w-full mx-auto px-2 prose ${className || ''}`}
      {...rest}
    />
  );
};

export { Body };
