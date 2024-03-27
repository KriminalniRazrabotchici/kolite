import styled from 'styled-components';

import useMoveBack from '../hooks/useMoveBack';

const StyledPageNotFound = styled.main`
  height: 100vh;
  background-color: grey;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4.8rem;
`;

const Box = styled.div`
  /* box */
  background-color: white;
  border: 1px solid lightgrey;
  border-radius: 11px;

  padding: 4.8rem;
  flex: 0 1 96rem;
  text-align: center;

  & h1 {
    margin-bottom: 3.2rem;
  }

  & p {
    font-size: large;
  }
`;

const Button = styled.button`
  background-color: black;
  color: white;
  padding: 8px;
`;

function PageNotFound() {
  const moveBack = useMoveBack();

  return (
    <StyledPageNotFound>
      <Box>
        <h1>The page you are looking for could not be found 😢</h1>
        <p>Click the button below to return to the previous page!</p>
        <Button onClick={moveBack} size='large'>
          &larr;
        </Button>
      </Box>
    </StyledPageNotFound>
  );
}

export default PageNotFound;
