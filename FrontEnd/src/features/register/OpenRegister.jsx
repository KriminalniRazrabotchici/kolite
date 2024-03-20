import { useState } from 'react';
import { ButtonLink } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Register from './RegisterForm';

function OpenRegister() {
  const [isOpenModal, setIsOpenModal] = useState(false);

  return (
    <div>
      <ButtonLink onClick={() => setIsOpenModal((show) => !show)}>
        Register
      </ButtonLink>
      {isOpenModal && (
        <Modal onClose={() => setIsOpenModal(false)}>
          <Register onCloseModal={() => setIsOpenModal(false)} />
        </Modal>
      )}
    </div>
  );
}

export default OpenRegister;
