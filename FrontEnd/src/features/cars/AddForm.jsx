import { useState } from 'react';

import { Form, Box, Btn, Input, Label } from '../../ui/Form';

function Add({ onCloseModal }) {
  const [brand, setBrand] = useState('');
  const [model, setModel] = useState('');

  function handleSubmit(e) {
    e.preventDefault();

    onCloseModal();
  }

  return (
    <Form onSubmit={handleSubmit}>
      <Box>
        <Label>Brand</Label>
        <Input
          type='text'
          name='brand'
          value={brand}
          onChange={(e) => setBrand(e.target.value)}
        />
      </Box>

      <Box>
        <Label>Model</Label>
        <Input
          type='text'
          name='model'
          value={model}
          onChange={(e) => setModel(e.target.value)}
        />
      </Box>

      <Btn>Add</Btn>
    </Form>
  );
}

export default Add;
