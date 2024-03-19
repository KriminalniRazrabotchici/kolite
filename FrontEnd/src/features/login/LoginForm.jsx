import { useState } from 'react';
import styled from 'styled-components';

const Form = styled.form`
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 3.2rem;

  height: 50vh;
  width: 50vh;

  /* background-color: red; */
`;

const Box = styled.div`
  color: var(--color-grey-900);
  font-size: 2.4rem;

  position: relative;

  display: flex;
  flex-direction: column;
`;

const Label = styled.label`
  pointer-events: none;
  transition: all 0.5s ease-in-out;
`;

const Input = styled.input`
  border: 0;
  border-bottom: 1px solid var(--color-grey-600);
  background: transparent;
  width: 100%;
  padding: 0.8rem 0 0.6rem 0;

  &:focus {
    border: none;
    outline: none;
    border-bottom: 1px solid #e74c3c;
  }
`;

const Btn = styled.button`
  background-color: #e74c3c;
  outline: none;
  border: 0;
  color: #fff;
  padding: 1.2rem 2.4rem;
  text-transform: uppercase;
  margin-top: 5rem;
  border-radius: 2px;
  cursor: pointer;
`;

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  return (
    <Form>
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
    </Form>
  );
}

export default Login;
