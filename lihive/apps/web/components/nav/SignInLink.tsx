import { useDisclosure } from '@chakra-ui/react';
import { NavLink } from '../common/NavLink';
import { SignInModal } from '../modals/SignInModal';

const SignInLink = () => {
  const { isOpen, onOpen, onClose } = useDisclosure();
  return (
    <>
      <NavLink onClick={onOpen}>Sign In</NavLink>
      <SignInModal isOpen={isOpen} onClose={onClose} />
    </>
  );
};

export { SignInLink };
