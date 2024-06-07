import { useRef } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { handleKeyDown, handleEnter } from "../utils";
import Input from "./Input";
import Button from "./Button";
import Label from "./Label";
import Logo from "../assets/logo.svg";

export default function Login() {
  const { signin } = useAuth();
  const navigate = useNavigate();
  const emailRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);

  function HandleLogin() {
    const email = emailRef.current?.value;
    const password = passwordRef.current?.value;

    if (email == "" || password == "" || !email || !password) {
      return;
    }

    signin(email, password, navigate);
  }

  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="flex flex-col gap-5 max-md:container w-[600px] p-7">
        <div className="flex gap-5 items-center">
          <img src={Logo} alt="Logo" width="50px" />
          <h2 className="font-bold text-lg">The Cracking the Code</h2>
        </div>
        <div className="flex flex-col gap-10 bg-background-dark p-10 rounded-md">
          <h2 className="text-xl font-bold">Iniciar Sesión</h2>
          <div className="flex flex-col gap-7">
            <div className="flex flex-col gap-2">
              <Label>Correo electrónico</Label>
              <Input
                ref={emailRef}
                type="email"
                onKeyDown={(e) => handleKeyDown(e, passwordRef)}
              />
            </div>
            <div className="flex flex-col gap-2">
              <Label>Contraseña</Label>
              <Input
                ref={passwordRef}
                type="password"
                onKeyDown={(e) => handleEnter(e, HandleLogin)}
              />
            </div>
          </div>
          <Button onClick={HandleLogin}>Iniciar sesión</Button>
        </div>
        <div className="flex gap-2">
          <div className="text-secondary">¿No tienes cuenta?</div>
          <Link to="/register" className="text-primary cursor-pointer">
            Regístrate
          </Link>
        </div>
      </div>
    </div>
  );
}
