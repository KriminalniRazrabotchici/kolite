import { createSlice } from '@reduxjs/toolkit';

const initialStateLoginRegister = {
  isLoginForm: false,
  isRegisterForm: false,
};

const loginRegisterSlice = createSlice({
  name: 'logReg',
  initialState: initialStateLoginRegister,
  reducers: {
    showLogin(state) {
      state.isLoginForm = true;
    },
    hideLogin(state) {
      state.isLoginForm = false;
    },
    showRegister(state) {
      state.isRegisterForm = true;
    },
    hideRegister(state) {
      state.isRegisterForm = false;
    },
  },
});

export const { showLogin, hideLogin, showRegister, hideRegister } =
  loginRegisterSlice.actions;

export default loginRegisterSlice.reducer;
