import { useAuth } from "../hooks/useAuth";

export default function Header() {
  const { signout } = useAuth();

  return (
    <div>
      <div>Header</div>
      <div>
        <button onClick={signout}>Cerrar sesión</button>
      </div>
    </div>
  );
}
