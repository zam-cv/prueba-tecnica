import Button from "./Button";
import { useNavigate } from "react-router-dom";

const messages = {
  Win: {
    title: "Felicidades, !has ganado!",
    description: "Has logrado escapar a tiempo, ¡buen trabajo!",
    image: "&#x1F389;",
  },
  Lose: {
    title: "¡Oh no, has perdido!",
    description: "No has logrado escapar a tiempo, ¡inténtalo de nuevo!",
    image: "&#x1F61E;",
  },
};

export default function Alert({ type }: { type: "Win" | "Lose" | null }) {
  const navigate = useNavigate();

  if (!type) {
    return null;
  }

  const message = messages[type];

  return (
    <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center bg-[#1b272b6b] z-10">
      <div className="p-7 max-md:container w-[600px]">
        <div className="bg-background-light rounded-md flex flex-col gap-10 p-10">
          <div>
            <h2 className="text-3xl font-bold text-center">{message.title}</h2>
            <div className="text-lg text-center text-secondary">
              {message.description}
            </div>
          </div>
          <div
            className="text-9xl text-center"
            dangerouslySetInnerHTML={{ __html: message.image }}
          ></div>
          <div className="flex justify-center gap-5">
            <Button onClick={() => window.location.reload()}>
              Volver a jugar
            </Button>
            <Button onClick={() => navigate("/rooms")}>
              Intentar con otra Room
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
