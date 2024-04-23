import { useState } from 'react';
import { Box, Btn, Form, Input, Label } from '../../ui/Form';
import { RedirectContainer } from '../../ui/RedirectContainer';
import { ButtonRedirect } from '../../ui/ButtonNav';
import { useDispatch } from 'react-redux';
import { hideRegister, showLogin } from '../../slices/LoginRegisterSlice';
import { open } from '../../slices/ModalSlice';
import { register } from '../../services/apiAuth';
import toast from 'react-hot-toast';

function Register({ onCloseModal }) {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [number, setNumber] = useState('');
  const [password, setPassword] = useState('');
  const [repass, setRePass] = useState('');

  const dispatch = useDispatch();

  function handleSubmit(e) {
    e.preventDefault();

    if (!name || !email || !number || !password) {
      toast.error('One or more fields are missing!');
      return;
    } else {
      if (password !== repass) {
        toast.error('Passwords not matching!');
        return;
      }
      onCloseModal();
      const res = register({ name, email, number, password });

      if (res.ok) {
        toast.success('You have successfully registered!');
      } else {
        toast.error('Failed');
      }
    }
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
          type='password'
          name='repass'
          value={repass}
          onChange={(e) => setRePass(e.target.value)}
        />
      </Box>

      <Btn>Register</Btn>

      <RedirectContainer>
        <p>Alredy have an account?</p>
        <ButtonRedirect
          onClick={() => {
            dispatch(open());
            dispatch(showLogin());
            dispatch(hideRegister());
          }}>
          login
        </ButtonRedirect>
      </RedirectContainer>
    </Form>
  );
}

export default Register;
