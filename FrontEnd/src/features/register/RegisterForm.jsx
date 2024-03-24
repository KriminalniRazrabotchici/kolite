import { useState } from 'react';
import { Box, Btn, Form, Input, Label } from '../../ui/Form';
import { RedirectContainer } from '../../ui/RedirectContainer';
import { ButtonRedirect } from '../../ui/ButtonNav';
import { useDispatch } from 'react-redux';
import { showLogin } from '../../slices/LoginRegisterSlice';

function Register({ onCloseModal }) {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [number, setNumber] = useState('');
  const [password, setPassword] = useState('');
  const [repass, setRePass] = useState('');

  const dispatch = useDispatch();

  function handleSubmit(e) {
    e.preventDefault();

    onCloseModal();
  }

  return (
    <Form onSubmit={handleSubmit} register>
      <Box>
        <Label>Full name</Label>
        <Input
          type='text'
          name='name'
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </Box>

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
        <Label>Phone number</Label>
        <Input
          type='tel'
          name='number'
          value={number}
          onChange={(e) => setNumber(e.target.value)}
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

      <Box>
        <Label>Repeat password</Label>
        <Input
          type='repass'
          name='repass'
          value={repass}
          onChange={(e) => setRePass(e.target.value)}
        />
      </Box>

      <Btn>Register</Btn>

      <RedirectContainer>
        <p>Alredy have an account?</p>
        <ButtonRedirect onClick={() => dispatch(showLogin())}>
          login
        </ButtonRedirect>
      </RedirectContainer>
    </Form>
  );
}

export default Register;
