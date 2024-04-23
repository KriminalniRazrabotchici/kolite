import { useState } from 'react';
import { toast } from 'react-hot-toast';

import { Form, Box, Btn, Input, Label } from '../../ui/Form';
import { RedirectContainer } from '../../ui/RedirectContainer';
import { ButtonRedirect } from '../../ui/ButtonNav';
import { useDispatch } from 'react-redux';
import { showRegister } from '../../slices/LoginRegisterSlice';
import { hideLogin } from '../../slices/LoginRegisterSlice';
import { open } from '../../slices/ModalSlice';
import { login } from '../../services/apiAuth';

function Login({ onCloseModal }) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const dispatch = useDispatch();

  function handleSubmit(e) {
    e.preventDefault();

    if (!email || !password) {
      toast.error('Email or password missing!');
      return;
    } else {
      onCloseModal();
      const res = login({ email, password });

      if (res.ok) {
        toast.success('You have successfully logged in!');
      } else {
        toast.error('Failed');
      }
    }
  }

  return (
    <Form onSubmit={handleSubmit}>
      <Box>
        <Label>Email</Label>
        <Input
          type='email'
          name='email'
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
      </Box>

      <Box>
        <Label>Password</Label>
        <Input
          type='password'
          name='password'
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </Box>

      <Btn>Log in</Btn>

      <RedirectContainer>
        <p>If you don't have an account</p>
        <ButtonRedirect
          onClick={() => {
            dispatch(open());
            dispatch(showRegister());
            dispatch(hideLogin());
          }}>
          register
        </ButtonRedirect>
      </RedirectContainer>
    </Form>
  );
}

export default Login;
