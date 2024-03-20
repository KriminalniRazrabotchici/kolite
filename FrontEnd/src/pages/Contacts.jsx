import { useCallback, useEffect, useState } from 'react';
import toast from 'react-hot-toast';
import emailjs from '@emailjs/browser';

import { Box, Btn, ContactsForm, Input, Label, TextArea } from '../ui/Form';

const emailRegex = /^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/;

const serviceId = 'service_h5xc6xi';
const templateId = 'template_ph40a4c';
const key = 'vgkVc0XkcC0bq13xg';

function Contacts() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [subject, setSubject] = useState('');
  const [message, setMessage] = useState('');

  const [isDisabled, setIsDisabled] = useState(false);

  function reset() {
    setName('');
    setEmail('');
    setSubject('');
    setMessage('');
  }

  const validate = useCallback(
    function validateFN() {
      const isValidEmail = emailRegex.test(email);

      const isValidName = name.length > 0;

      return isValidName & isValidEmail;
    },
    [email, name.length]
  );

  useEffect(() => {
    const check = validate();
    setIsDisabled(check);
  }, [name, email, validate]);

  function handleSubmit(e) {
    e.preventDefault();

    if (message.length < 10) {
      toast.error('Message must have more then 10 symbols!');
      return;
    }

    try {
      emailjs.sendForm(serviceId, templateId, e.target, key);

      toast.success('Email was send successfully!');
      reset();
    } catch (err) {
      console.error(err.message);
      toast.error(err.message);
    }
  }

  return (
    <ContactsForm onSubmit={handleSubmit}>
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
          placeholder='email@example.com'
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
      </Box>

      <Box>
        <Label>Subject</Label>
        <Input
          type='text'
          name='subject'
          value={subject}
          onChange={(e) => setSubject(e.target.value)}
        />
      </Box>

      <Box>
        <Label>Message</Label>
        <TextArea
          cols='55'
          rows='5'
          name='message'
          value={message}
          onChange={(e) => setMessage(e.target.value)}
        />
      </Box>

      <Btn disabled={!isDisabled}>Submit</Btn>
    </ContactsForm>
  );
}

export default Contacts;
