import { useState } from 'react';
import { ButtonLink, ButtonRedirect } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Register from './RegisterForm';
import Login from '../login/LoginForm';
import { RedirectContainer } from '../../ui/RedirectContainer';

function OpenRegister() {
  const [isOpenModal, setIsOpenModal] = useState(false);
  const [isRegister, setIsRegister] = useState(true);

  return (
    <div>
      <ButtonLink onClick={() => setIsOpenModal((show) => !show)}>
        Register
      </ButtonLink>
      {isOpenModal && (
        <Modal
          onClose={() => {
            setIsOpenModal(false);
            setIsRegister(true);
          }}>
          {isRegister ? (
            <>
              <Register onCloseModal={() => setIsOpenModal(false)} />
              <RedirectContainer>
                <p>Alredy have an account?</p>
                <ButtonRedirect
                  onClick={() => setIsRegister((register) => !register)}>
                  login
                </ButtonRedirect>
              </RedirectContainer>
            </>
          ) : (
            <Login />
          )}
        </Modal>
      )}
    </div>
  );
}

export default OpenRegister;
