import { Menu, MenuButton, MenuItem, MenuList } from '@chakra-ui/react';
import { UserData } from 'hive-db';
import { NavText } from '../common/NavText';

interface UsernameProps {
  user: UserData;
  signout: (redirect: string) => Promise<void>;
}

const Username = (props: UsernameProps) => {
  const { user, signout } = props;
  return (
    <Menu placement='bottom-end'>
      <MenuButton className='flex items-center hover:underline decoration-[#f8a61c] decoration-2 underline-offset-4 active:text-slate-900'>
        <NavText>{user.username}</NavText>
      </MenuButton>
      <MenuList>
        <MenuItem onClick={() => signout('/')}>Sign Out</MenuItem>
      </MenuList>
    </Menu>
  );
};

export { Username };
