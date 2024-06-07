import { useRef } from "react";
import { Link, useNavigate } from "react-router-dom";
import { handleKeyDown, handleEnter } from "../utils";
import api from "../utils/api";
import Input from "./Input";
import Button from "./Button";
import Label from "./Label";
import Logo from "../assets/logo.svg";

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
        navigate("/");
      })
      .catch((error) => {
        console.error(error);
      });
  }

  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="flex flex-col gap-5 max-md:container w-[600px] p-7">
        <div className="flex gap-5 items-center">
          <img src={Logo} alt="Logo" width="50px" />
          <h2 className="font-bold text-lg">The Cracking the Code</h2>
        </div>
        <div className="flex flex-col gap-10 bg-background-dark p-10 rounded-md">
          <div className="text-xl font-bold">Crea una cuenta</div>
          <div className="flex flex-col gap-7">
            <div className="flex flex-col gap-2">
              <Label>Nombre de Usuario</Label>
              <Input
                ref={usernameRef}
                type="text"
                onKeyDown={(e) => handleKeyDown(e, emailRef)}
              />
            </div>
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
                onKeyDown={(e) => handleEnter(e, handleRegister)}
              />
            </div>
          </div>
          <Button onClick={handleRegister}>Registrarte</Button>
        </div>
        <div className="flex gap-2">
          <div className="text-secondary">¿Ya tienes cuenta?</div>
          <Link to="/" className="text-primary cursor-pointer">
            Iniciar sesión
          </Link>
        </div>
      </div>
    </div>
  );
}
