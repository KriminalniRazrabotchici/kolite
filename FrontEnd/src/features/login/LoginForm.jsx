import { useState } from 'react';

import { Form, Box, Btn, Input, Label } from '../../ui/Form';
import { RedirectContainer } from '../../ui/RedirectContainer';
import { ButtonRedirect } from '../../ui/ButtonNav';
import { useDispatch } from 'react-redux';
import { showRegister } from '../../slices/RegisterSlice';

function Login({ onCloseModal }) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const dispatch = useDispatch();

  function handleSubmit(e) {
    e.preventDefault();

    onCloseModal();
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
        <ButtonRedirect onClick={() => dispatch(showRegister())}>
          register
        </ButtonRedirect>
      </RedirectContainer>
    </Form>
  );
}

export default Login;
