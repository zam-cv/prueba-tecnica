import { Link } from "react-router-dom";

export default function Login() {
  return (
    <div>
      <div>
        <div>The Cracking the Code</div>
        <div>
          <input type="email" placeholder="Email" />
          <input type="password" placeholder="Password" />
          <button>Iniciar sesión</button>
        </div>
        <div>
          <div>¿No tienes cuenta?</div>
          <Link to="/register">Regístrate</Link>
        </div>
      </div>
    </div>
  );
}
