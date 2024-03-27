import styled from 'styled-components';

export const Form = styled.form`
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 3.2rem;

  height: ${(props) => (props.register ? '80vh' : '50vh')};
  width: 50vh;

  /* background-color: red; */
`;

export const ContactsForm = styled(Form)`
  margin: 0 auto;
  padding: 0 8rem;

  height: 100vh;
  width: 50%;
`;

export const Box = styled.div`
  color: var(--color-grey-900);
  font-size: 2rem;

  position: relative;

  display: flex;
  flex-direction: column;
`;

export const Label = styled.label`
  pointer-events: none;
  transition: all 0.5s ease-in-out;
`;

export const Input = styled.input`
  font-size: 1.8rem;
  border: 0;
  border-bottom: 1px solid var(--color-grey-600);
  background: transparent;
  width: 100%;
  padding: 0.8rem 0 0.6rem 0;

  &:focus {
    border: none;
    outline: none;
    border-bottom: 1px solid var(--color-red-700);
  }
`;

export const TextArea = styled.textarea`
  border: 0;
  border-bottom: 1px solid var(--color-grey-600);
  background: transparent;
  width: 100%;
  padding: 0.8rem 0 0.6rem 0;

  resize: none;

  &:focus {
    border: none;
    outline: none;
    border-bottom: 1px solid var(--color-red-700);
  }

  &::-webkit-scrollbar {
    display: none;
  }
`;

export const Btn = styled.button`
  background-color: var(--color-red-500);
  outline: none;
  border: 0;
  color: #fff;
  padding: 1.2rem 2.4rem;
  text-transform: uppercase;
  margin-top: 5rem;
  border-radius: 2px;
  cursor: pointer;

  transition: all 0.5s;

  &:hover {
    background-color: var(--color-red-700);
  }

  &:disabled {
    /* border: 3px solid var(--color-grey-400); */
    color: var(--color-grey-300);
    background-color: var(--color-grey-400);
  }
`;
