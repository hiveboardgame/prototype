import { Button, useDisclosure } from '@chakra-ui/react';
import { HTMLAttributes, useState } from 'react';
import { UserData } from 'hive-db';
import { InviteModal } from '../modals/NewGameModal';
import { Row, RowItem } from './Row';

interface ListUsersProps extends HTMLAttributes<HTMLDivElement> {
  users: UserData[];
}

const ListUsers = (props: ListUsersProps) => {
  const { users, className, ...rest } = props;
  const [user, setUser] = useState<UserData>();
  const { isOpen, onOpen, onClose } = useDisclosure();

  const sorted = users.sort((a, b) =>
    a.username.toLowerCase() < b.username.toLowerCase() ? -1 : 1
  );

  return (
    <div className={`grid grid-cols-2 w-full ${className || ''}`} {...rest}>
      {users.map((user) => {
        return (
          <Row key={user.uid}>
            <RowItem>{user.username}</RowItem>
            <RowItem>
              <Button
                size='sm'
                colorScheme='teal'
                onClick={() => {
                  setUser(user);
                  onOpen();
                }}
              >
                Invite to Play
              </Button>
            </RowItem>
          </Row>
        );
      })}
      <InviteModal isOpen={isOpen} onClose={onClose} opponent={user} />
    </div>
  );
};

export { ListUsers };
