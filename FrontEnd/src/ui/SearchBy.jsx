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

const buttons = [
  { id: 1, name: 'Coupe' },
  { id: 2, name: 'Brand' },
  { id: 3, name: 'Fuel' },
  { id: 4, name: 'Transmission' },
  { id: 5, name: 'Price' },
  { id: 6, name: 'Year' },
  { id: 7, name: 'City' },
  { id: 8, name: 'Color' },
  { id: 9, name: 'Doors' },
  { id: 10, name: 'HP' },
  { id: 11, name: 'Extras' },
  { id: 12, name: 'Steering wheel' },
];

function SearchBy() {
  function handleClick() {}

  return (
    <SearchDiv>
      {buttons.map((btn) => (
        <Button onClick={handleClick} key={btn.id}>
          {btn.name}
        </Button>
      ))}
    </SearchDiv>
  );
}

export default SearchBy;
