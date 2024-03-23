import { createSlice } from '@reduxjs/toolkit';

const initialStateRegister = {
  isRegisterForm: false,
};

const registerSlice = createSlice({
  name: 'register',
  initialState: initialStateRegister,
  reducers: {
    showRegister(state) {
      state.isRegisterForm = true;
    },
    hideRegister(state) {
      state.isRegisterForm = false;
    },
  },
});

export const { showRegister, hideRegister } = registerSlice.actions;

export default registerSlice.reducer;
