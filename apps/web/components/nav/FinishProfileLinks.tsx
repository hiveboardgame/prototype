import Link from 'next/link';
import { usePlayer } from 'hive-db';
import { NavLink } from '../common/NavLink';

const FinishProfileLinks = () => {
  const { signout } = usePlayer();
  return (
    <>
      <Link href='/profile' passHref>
        <NavLink className='underline'>
          Finish profile to start playing!
        </NavLink>
      </Link>
      <NavLink className='underline' onClick={() => signout('/')}>
        Sign Out
      </NavLink>
    </>
  );
};

export { FinishProfileLinks };
