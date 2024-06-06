import { useRef } from "react";
import { Link, useNavigate } from "react-router-dom";
import api from "../utils/api";

export default function Register() {
  const navigate = useNavigate();
  const usernameRef = useRef<HTMLInputElement>(null);
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);

  function handleRegister() {
    const username = usernameRef.current?.value;
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (
      username == "" ||
      email == "" ||
      password == "" ||
      !username ||
      !email ||
      !password ||
      password.length < 8
    ) {
      return;
    }

    api.auth
      .register(username, email, password)
      .then(() => {
        navigate("/login");
      })
      .catch((error) => {
        console.error(error);
      });
  }

  return (
    <div>
      <div>
        <div>Crea una cuenta</div>
        <div>
          <div>
            <label>Nombre de Usuario</label>
            <input ref={usernameRef} type="text" />
          </div>
          <div>
            <label>Email</label>
            <input ref={emailRef} type="email" />
          </div>
          <div>
            <label>Contraseña</label>
            <input ref={passwordRef} type="password" />
          </div>
          <div>
            <button onClick={handleRegister}>Registrarte</button>
          </div>
        </div>
      </div>
      <div>
        <div>¿Ya tienes cuenta?</div>
        <Link to="/login">Iniciar sesión</Link>
      </div>
    </div>
  );
}
