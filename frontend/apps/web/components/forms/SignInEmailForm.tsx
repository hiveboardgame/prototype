import {
  Button,
  FormControl,
  FormErrorMessage,
  Input,
  VStack
} from '@chakra-ui/react';
import { Form, Formik, Field } from 'formik';
import * as Yup from 'yup';
import { parseAuthError, usePlayer } from 'hive-db';
import { FormSubmitButtonProps } from './FormSubmitButtonProps';

const emailValidationSchema = Yup.object({
  email: Yup.string()
    .email('Invalid email address')
    .required('Email address is required'),
  password: Yup.string().required('Password is required')
});
const emailInitialValues = { email: '', password: '', passwordConfirm: '' };

const SignInEmailForm = (props: FormSubmitButtonProps) => {
  const { disabled, onPending, onSuccess, onFailure } = props;
  const { signInWithEmail } = usePlayer();
  const onSubmit = ({
    email,
    password
  }: {
    email: string;
    password: string;
  }) => {
    onPending();
    signInWithEmail(email, password)
      .then(onSuccess)
      .catch((error) => {
        onFailure(parseAuthError(error));
      });
  };
  return (
    <Formik
      initialValues={emailInitialValues}
      validationSchema={emailValidationSchema}
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
          <Button isDisabled={disabled} type='submit' width='100%'>
            Sign In with Email
          </Button>
        </VStack>
      </Form>
    </Formik>
  );
};

export { SignInEmailForm };
