import { Form, Formik, Field } from 'formik';
import * as Yup from 'yup';
import { parseAuthError, usePlayer } from 'hive-db';
import {
  Button,
  FormControl,
  FormErrorMessage,
  Input,
  useToast,
  VStack
} from '@chakra-ui/react';

interface SignUpEmailFormProps {
  setPending: (isPending: boolean) => void;
  onSuccess: () => void;
  disabled?: boolean;
}

const validationSchema = Yup.object({
  email: Yup.string()
    .email('Invalid email address')
    .required('Email address is required'),
  password: Yup.string()
    .min(6, 'Password must be at least 6 characters long')
    .required('Password is required'),
  passwordConfirm: Yup.string().oneOf(
    [Yup.ref('password'), null],
    'Passwords must match'
  )
});

const initialValues = { email: '', password: '', passwordConfirm: '' };

const SignUpEmailForm = (props: SignUpEmailFormProps) => {
  const { setPending, disabled, onSuccess } = props;
  const { signUpWithEmail } = usePlayer();

  const toast = useToast();

  const onSubmit = ({
    email,
    password
  }: {
    email: string;
    password: string;
  }) => {
    setPending(true);
    signUpWithEmail(email, password)
      .then(onSuccess)
      .catch((error) => {
        setPending(false);
        toast({
          title: 'Error',
          description: parseAuthError(error),
          status: 'error',
          duration: 9000,
          isClosable: true
        });
      });
  };

  return (
    <Formik
      initialValues={initialValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      <Form>
        <VStack spacing={2} width='full'>
          <Field name='email'>
            {({ field, form }: any) => (
              <FormControl
                isDisabled={disabled}
                isInvalid={form.errors.email && form.touched.email}
              >
                <Input {...field} id='email' placeholder='Email' />
                <FormErrorMessage>{form.errors.email}</FormErrorMessage>
              </FormControl>
            )}
          </Field>
          <Field name='password'>
            {({ field, form }: any) => (
              <FormControl
                isDisabled={props.disabled}
                isInvalid={form.errors.password && form.touched.password}
              >
                <Input
                  {...field}
                  id='password'
                  placeholder='Password'
                  type='password'
                />
                <FormErrorMessage>{form.errors.password}</FormErrorMessage>
              </FormControl>
            )}
          </Field>
          <Field name='passwordConfirm'>
            {({ field, form }: any) => (
              <FormControl
                isDisabled={props.disabled}
                isInvalid={
                  form.errors.passwordConfirm && form.touched.passwordConfirm
                }
              >
                <Input
                  {...field}
                  id='passwordConfirm'
                  placeholder='Confirm Password'
                  type='password'
                />
                <FormErrorMessage>
                  {form.errors.passwordConfirm}
                </FormErrorMessage>
              </FormControl>
            )}
          </Field>
          <Button isDisabled={disabled} type='submit' width='100%'>
            Sign Up
          </Button>
        </VStack>
      </Form>
    </Formik>
  );
};

export { SignUpEmailForm };
