import styled from 'styled-components';
import {
  respondToLandscapeTablets,
  respondToMobile,
  respondToSmallLaptop,
} from '../styles/mediaQueries';

export const Form = styled.form`
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 3.2rem;

  height: ${(props) => (props.register ? '90vh' : '50vh')};
  width: 50vh;

  ${respondToSmallLaptop(`width: 45vh;`)}
  ${respondToLandscapeTablets(`width: 40vh;`)}
  ${respondToMobile(`width: 30vh;`)}
`;

export const ContactsForm = styled(Form)`
  margin: 0 auto;
  padding: 0 8rem;

  height: 100vh;
  width: 50%;

  ${respondToSmallLaptop(`width: 60%;`)}
  ${respondToLandscapeTablets(`width: 75%;`)}
  ${respondToMobile(`width: 100%;`)}
`;

export const Box = styled.div`
  color: var(--color-grey-900);
  font-size: 2rem;

  position: relative;

  display: flex;
  flex-direction: column;

  ${respondToSmallLaptop(`font-size: 1.8rem;`)}

  ${respondToLandscapeTablets(`font-size: 1.6rem;`)}
  ${respondToMobile(`font-size: 1.4rem;`)}
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

  &::placeholder {
    font-size: 1.8rem;
  }

  ${respondToLandscapeTablets(`&::placeholder {
    font-size: 1.6rem;
  }`)}
  ${respondToMobile(`&::placeholder {
    font-size: 1.4rem;
  }`)}
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
  font-size: 1.8rem;
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

  ${respondToSmallLaptop(`font-size: 1.6rem;`)}
  ${respondToLandscapeTablets(`font-size: 1.4rem;`)}
  ${respondToMobile(`font-size: 1.2rem;
  padding: 0.8rem 1.6rem;`)}
`;
