import { useState } from 'react';

import { Form, Box, Btn, Input, Label } from '../../ui/Form';

function Login({ onCloseModal }) {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

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
    </Form>
  );
}

export default Login;
