import styled from 'styled-components';
import useOpenModal from '../hooks/useOpenModal';

import {
  showCoupe,
  showBrand,
  showFuel,
  showTransmission,
  showPrice,
  showYear,
  showCity,
  showColor,
  showHP,
  showDoors,
  showExtras,
  showWheel,
} from '../slices/SearchButtonsSlice';

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
  { id: 1, name: 'Coupe', fn: showCoupe() },
  { id: 2, name: 'Brand', fn: showBrand() },
  { id: 3, name: 'Fuel', fn: showFuel() },
  { id: 4, name: 'Transmission', fn: showTransmission() },
  { id: 5, name: 'Price', fn: showPrice() },
  { id: 6, name: 'Year', fn: showYear() },
  { id: 7, name: 'City', fn: showCity() },
  { id: 8, name: 'Color', fn: showColor() },
  { id: 9, name: 'Doors', fn: showDoors() },
  { id: 10, name: 'HP', fn: showHP() },
  { id: 11, name: 'Extras', fn: showExtras() },
  { id: 12, name: 'Steering wheel', fn: showWheel() },
];

function SearchBy() {
  const handleSearchButton = useOpenModal();

  function handleClick(btn) {
    handleSearchButton(btn.fn);
  }

  return (
    <SearchDiv>
      {buttons.map((btn) => (
        <Button onClick={() => handleClick(btn)} key={btn.id}>
          {btn.name}
        </Button>
      ))}
    </SearchDiv>
  );
}

export default SearchBy;
