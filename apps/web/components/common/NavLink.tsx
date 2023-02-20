import { ForwardedRef, forwardRef, HTMLAttributes } from 'react';

const NavLink = forwardRef(
  (
    props: HTMLAttributes<HTMLAnchorElement>,
    ref: ForwardedRef<HTMLAnchorElement>
  ) => {
    const { className, ...rest } = props;
    return (
      <a
        className={`px-2 pt-0.5 prose tracking-tight cursor-pointer hover:underline decoration-hive decoration-2 underline-offset-4 active:text-slate-900 ${
          className || ''
        }`}
        ref={ref}
        {...rest}
      />
    );
  }
);
NavLink.displayName = 'NavLink';

export { NavLink };
