import { createSlice } from '@reduxjs/toolkit';

const initialStateModal = {
  isOpen: false,
  isOpenSearch: false,
};

const modalSlice = createSlice({
  name: 'modal',
  initialState: initialStateModal,
  reducers: {
    open(state) {
      state.isOpen = true;
    },
    close(state) {
      state.isOpen = false;
    },
    openSearch(state) {
      state.isOpenSearch = true;
    },
    closeSearch(state) {
      state.isOpenSearch = false;
    },
  },
});

export const { open, close, openSearch, closeSearch } = modalSlice.actions;

export default modalSlice.reducer;
