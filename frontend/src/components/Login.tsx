import { useRef } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";

export default function Login() {
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const { signin } = useAuth();
  const navigate = useNavigate();

  function HandleLogin() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (email == "" || password == "" || !email || !password) {
      return;
    }

    signin(email, password, navigate);
  }

  return (
    <div>
      <div>
        <div>The Cracking the Code</div>
        <div>
          <input ref={emailRef} type="email" placeholder="Email" />
          <input ref={passwordRef} type="password" placeholder="Password" />
          <button onClick={HandleLogin}>Iniciar sesión</button>
        </div>
        <div>
          <div>¿No tienes cuenta?</div>
          <Link to="/register">Regístrate</Link>
        </div>
      </div>
    </div>
  );
}
