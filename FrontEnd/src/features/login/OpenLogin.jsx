import { useState } from 'react';
import { ButtonLink, ButtonRedirect } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Login from './LoginForm';
import Register from '../register/RegisterForm';
import { RedirectContainer } from '../../ui/RedirectContainer';

function OpenLogin() {
  const [isOpenModal, setIsOpenModal] = useState(false);

  const [isLogin, setIsLogin] = useState(true);

  return (
    <div>
      <ButtonLink onClick={() => setIsOpenModal((show) => !show)}>
        Login
      </ButtonLink>
      {isOpenModal && (
        <Modal
          onClose={() => {
            setIsOpenModal(false);
            setIsLogin(true);
          }}>
          {isLogin ? (
            <>
              <Login onCloseModal={() => setIsOpenModal(false)} />
              <RedirectContainer>
                <p>If you don't have an account</p>
                <ButtonRedirect onClick={() => setIsLogin((login) => !login)}>
                  register
                </ButtonRedirect>
              </RedirectContainer>
            </>
          ) : (
            <Register />
          )}
        </Modal>
      )}
    </div>
  );
}

export default OpenLogin;
