import {
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
  ModalOverlay,
  ModalProps,
  useToast
} from '@chakra-ui/react';
import { useState } from 'react';
import { GoogleButton } from '../forms/GoogleButton';
import { SignInEmailForm } from '../forms/SignInEmailForm';

interface SignInModalProps extends Omit<ModalProps, 'children'> {}

const SignInModal = (props: SignInModalProps) => {
  const toast = useToast();
  const [disabled, setDisabled] = useState(false);

  const onPending = () => {
    setDisabled(true);
  };

  const onSuccess = () => {
    props.onClose();
  };

  const errorToast = (message: string) => {
    toast({
      title: 'Error',
      description: message,
      status: 'error',
      duration: 9000,
      isClosable: true
    });
  };

  return (
    <Modal {...props}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Sign In</ModalHeader>
        <ModalBody>
          <div className='divide-y divide-dashed'>
            <div className='pb-4'>
              <GoogleButton
                disabled={disabled}
                onPending={onPending}
                onSuccess={onSuccess}
                onFailure={() => {
                  setDisabled(false);
                  errorToast('Unable to sign in with Google');
                }}
              >
                Sign in with Google
              </GoogleButton>
            </div>
            <div className='py-4'>
              <SignInEmailForm
                disabled={disabled}
                onPending={onPending}
                onSuccess={onSuccess}
                onFailure={(message) => {
                  setDisabled(false);
                  errorToast(message || 'Unable to sign in with email');
                }}
              />
            </div>
          </div>
        </ModalBody>
      </ModalContent>
    </Modal>
  );
};

export { SignInModal };
