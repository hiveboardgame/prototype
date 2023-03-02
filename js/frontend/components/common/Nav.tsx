import { HTMLAttributes, PropsWithChildren } from 'react';

export interface NavProps extends HTMLAttributes<HTMLDivElement> {
  fullWidth?: boolean;
}

const Nav = (props: PropsWithChildren<NavProps>) => {
  const { fullWidth, className, children, ...rest } = props;
  return (
    <div
      className={`sticky top-0 p-2 bg-white select-none z-50 ${
        className || ''
      }`}
      {...rest}
    >
      <nav
        className={`${
          fullWidth ? 'max-w-none' : 'max-w-screen-lg'
        } flex items-center justify-between mx-auto`}
      >
        {children}
      </nav>
    </div>
  );
};

export { Nav };
