import styled from 'styled-components';
import OpenButtons from '../utils/OpenButtons';

import { useDispatch } from 'react-redux';
import { openSearch } from '../slices/ModalSlice';

import {
  showCoupe,
  showBrand,
  showModel,
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
  { id: 1, name: 'Coupe', fnShow: showCoupe() },
  { id: 2, name: 'Brand', fnShow: showBrand() },
  { id: 3, name: 'Model', fnShow: showModel() },
  { id: 4, name: 'Fuel', fnShow: showFuel() },
  { id: 5, name: 'Transmission', fnShow: showTransmission() },
  { id: 6, name: 'Price', fnShow: showPrice() },
  { id: 7, name: 'Year', fnShow: showYear() },
  { id: 8, name: 'City', fnShow: showCity() },
  { id: 9, name: 'Color', fnShow: showColor() },
  { id: 10, name: 'Doors', fnShow: showDoors() },
  { id: 11, name: 'HP', fnShow: showHP() },
  { id: 12, name: 'Extras', fnShow: showExtras() },
  { id: 13, name: 'Steering wheel', fnShow: showWheel() },
];

function SearchBy() {
  const dispatch = useDispatch();

  function handleClick(btn) {
    dispatch(openSearch());
    dispatch(btn.fnShow);
  }

  return (
    <>
      <OpenButtons />
      <SearchDiv>
        {buttons.map((btn) => (
          <Button onClick={() => handleClick(btn)} key={btn.id}>
            {btn.name}
          </Button>
        ))}
      </SearchDiv>
    </>
  );
}

export default SearchBy;
