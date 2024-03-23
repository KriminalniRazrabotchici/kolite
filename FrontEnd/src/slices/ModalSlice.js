import { createSlice } from '@reduxjs/toolkit';

const initialStateModal = {
  isOpen: false,
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
  },
});

export const { open, close } = modalSlice.actions;

export default modalSlice.reducer;
