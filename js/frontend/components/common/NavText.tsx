import { HTMLAttributes } from 'react';

const NavText = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  return (
    <div
      className={`px-2 pt-0.5 prose tracking-tight ${className || ''}`}
      {...rest}
    />
  );
};

export { NavText };
