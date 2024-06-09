import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { getConfig } from "../utils/auth";
import { SOCKET_URL, RESOURCES_URL } from "../utils/constants";
import { handleEnter } from "../utils";
import api from "../utils/api";
import Timer from "./Timer";
import Progress from "./Progress";
import Input from "./Input";
import { ButtonPrimary } from "./Button";
import Alert from "./Alert";

export default function Room() {
  const [username, setUsername] = useState("");
  const [time, setTime] = useState(0);

  const [type, setType] = useState<"Win" | "Lose" | null>(null);
  const [alert, setAlert] = useState(false);
  const [pause, setPause] = useState(false);

  const answerRef = useRef<HTMLInputElement | null>(null);
  const [title, setTitle] = useState("Room ...");
  const [image, setImage] = useState<null | string>(null);
  const [example, setExample] = useState<null | string>(null);
  const [duration, setDuration] = useState(0);
  const socket = useRef<WebSocket | null>(null);
  const { id } = useParams<{ id: string }>();

  useEffect(() => {
    if (!id) {
      return;
    }

    api.rooms.getBestSolvingTime(parseInt(id)).then((data) => {
      if (data) {
        setUsername(data[1]);
        setTime(data[0]);
      }
    });
  }, [id]);

  useEffect(() => {
    const config = getConfig();
    const _socket = new WebSocket(SOCKET_URL + "/" + id, [
      "Authorization",
      config.headers.Authorization || "",
    ]);

    _socket.onopen = () => {
      _socket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);

          switch (data.type) {
            case "Init":
              setTitle(data.title);
              setImage(data.image);
              setExample(data.example);
              setDuration(data.duration * 60); // duration in seconds
              break;
            case "Win":
              setAlert(true);
              setPause(true);
              setType("Win");
              break;
            case "Lose":
              setAlert(true);
              setPause(true);
              setType("Lose");
              break;
          }
        } catch (error) {
          console.error(error);
        }
      };

      _socket.onclose = () => {
        console.log("Disconnected from server");
      };

      socket.current = _socket;
    };

    return () => {
      if (socket.current) {
        socket.current.close();
      }
    };
  }, [id]);

  function sendAnswer() {
    if (socket.current && answerRef.current && answerRef.current.value !== "") {
      socket.current.send(answerRef.current.value);
      answerRef.current.value = "";
    }
  }

  return (
    <>
      {alert && <Alert type={type} />}
      <div className="grid grid-rows-[auto_1fr] h-full">
        <Progress duration={duration} pause={pause} />
        <div className="grid grid-rows-[auto_1fr] p-10 gap-10">
          <div className="grid md:grid-cols-[auto_1fr] max-md:gap-10">
            <div className="flex flex-col gap-6">
              <div className="flex gap-5 max-md:flex-col">
                <h2 className="text-3xl font-bold max-md:text-center">
                  {title}
                </h2>
                <div className="flex items-end max-md:justify-center">
                  {username !== "" && (
                    <p className="text-sm font-semibold pb-1">
                      ⭐ {username} en {time} s
                    </p>
                  )}
                </div>
              </div>
              <div className="flex gap-5 max-md:justify-center">
                <Input
                  ref={answerRef}
                  placeholder={"Respuesta" + (example ? ` (${example})` : "")}
                  onKeyDown={(e) => handleEnter(e, sendAnswer)}
                />
                <ButtonPrimary onClick={sendAnswer}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    strokeWidth={2.3}
                    stroke="currentColor"
                    className="size-6"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      d="M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5"
                    />
                  </svg>
                </ButtonPrimary>
              </div>
            </div>
            <div className="flex md:justify-end max-md:justify-center">
              <Timer duration={duration} pause={pause} />
            </div>
          </div>
          <div className="flex justify-center">
            <div className="relative h-full w-full">
              {image && (
                <img
                  src={`${RESOURCES_URL}/${image}`}
                  alt={title}
                  className="h-full w-full object-contain object-top absolute"
                />
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
