import styled from 'styled-components';

export const HomePage = styled.div`
  display: grid;
  grid-template-columns: repeat(4, 1fr);

  align-items: start;
  justify-items: center;

  row-gap: 6.4rem;

  /* height: 80vh; */

  /* background-color: red; */
  height: 90vh;
  margin: 2.4rem;
  margin-top: 1.2rem;
  padding: 3.2rem;

  overflow: scroll;

  &::-webkit-scrollbar {
    display: none;
  }
`;
