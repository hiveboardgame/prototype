import { usePlayer } from 'hive-db';
import { Nav, NavProps } from '../common/Nav';
import Link from 'next/link';
import { NavLink } from '../common/NavLink';
import { FinishProfileLinks } from './FinishProfileLinks';
import { HiveLogoLink } from './HiveLogoLink';
import { NotificationsBell } from './NotificationsBell';
import { SignInLink } from './SignInLink';
import { Spinner } from './Spinner';
import { Username } from './Username';

const NavBarUsername = (props: NavBarProps) => {
  const { hideFinishProfile } = props;
  const { uid, user, incompleteProfile, signout } = usePlayer();
  if (!uid) {
    return <SignInLink />;
  }
  if (!incompleteProfile) {
    if (!user) {
      return <Spinner />;
    } else {
      return <Username user={user} signout={signout} />;
    }
  }
  if (!hideFinishProfile) {
    return <FinishProfileLinks />;
  }
  return null;
};

interface NavBarProps extends NavProps {
  hideFinishProfile?: boolean;
}

const NavBar = (props: NavBarProps) => {
  const { hideFinishProfile, ...rest } = props;
  const { user } = usePlayer();
  return (
    <Nav {...rest}>
      <div className='flex items-center flex-grow'>
        <Link href='/' passHref>
          <HiveLogoLink />
        </Link>
        {user && (
          <>
            <Link href='/games' passHref>
              <NavLink>Games</NavLink>
            </Link>
            <Link href='/community' passHref>
              <NavLink>Community</NavLink>
            </Link>
          </>
        )}
      </div>
      {user && <NotificationsBell />}
      <NavBarUsername />
    </Nav>
  );
};

export { NavBar };
