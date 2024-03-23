import { createSlice } from '@reduxjs/toolkit';

const initialStateLogin = {
  isLoginForm: false,
};

const loginSlice = createSlice({
  name: 'login',
  initialState: initialStateLogin,
  reducers: {
    showLogin(state) {
      state.isLoginForm = true;
    },
    hideLogin(state) {
      state.isLoginForm = false;
    },
  },
});

export const { showLogin, hideLogin } = loginSlice.actions;

export default loginSlice.reducer;
