import styled from 'styled-components';

const SearchDiv = styled.div`
  width: 90%;
  height: 10%;
  margin: 2.4rem;

  display: flex;
  align-items: center;
  justify-content: center;

  flex-flow: row wrap;

  gap: 0.8rem;
`;

const Button = styled.button`
  display: flex;
  align-items: center;
  justify-content: center;

  width: 13rem;
  height: 3.5rem;

  font-size: 1.6rem;

  background-color: transparent;
  color: var(--color-red-500);
  /* word-wrap: break-word; */
  border: none;
  /* border-radius: var(--border-radius-round); */
  box-shadow: var(--shadow-md);

  position: relative;

  transition: all 0.5s;

  &::before {
    content: '';
    background-color: var(--color-red-500);
    position: absolute;
    left: 0;
    top: 100%;
    width: 0;
    height: 0.2rem;

    transition: all 0.5s;
  }

  &:hover,
  &:active {
    &::before {
      width: 100%;
    }
    color: var(--black);
    box-shadow: none;
  }
`;

function SearchBy() {
  return (
    <SearchDiv>
      <Button>Coupe</Button>
      <Button>Brand</Button>
      <Button>Fuel type</Button>
      <Button>Transmission</Button>
      <Button>Price</Button>
      <Button>Year</Button>
      <Button>City</Button>
      <Button>Color</Button>
      <Button>Door number</Button>
      <Button>Horse power</Button>
      <Button>Extras</Button>
      <Button>Steering wheel</Button>
    </SearchDiv>
  );
}

export default SearchBy;
