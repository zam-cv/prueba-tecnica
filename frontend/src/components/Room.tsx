import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import Timer from "./Timer";
import { getConfig } from "../utils/auth";
import { SOCKET_URL, RESOURCES_URL } from "../utils/constants";

export default function Room() {
  const [title, setTitle] = useState("Room ...");
  const [image, setImage] = useState<null | string>(null);
  const [duration, setDuration] = useState(0);
  const socket = useRef<WebSocket | null>(null);
  const { id } = useParams<{ id: string }>();

  useEffect(() => {
    const config = getConfig();
    const _socket = new WebSocket(SOCKET_URL + "/" + id, [
      "Authorization",
      config.headers.Authorization || "",
    ]);

    _socket.onopen = () => {
      console.log("Connected to server");

      _socket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          console.log(data);
          setTitle(data[0]);
          setImage(data[1]);
          setDuration(data[2] * 60);
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

  return (
    <div className="grid grid-rows-[auto_auto_1fr] p-10 gap-7 h-full">
      <div>
        <h2 className="text-3xl font-bold text-center">{title}</h2>
      </div>
      <div>
        <Timer duration={duration} />
      </div>
      <div className="flex justify-center">
        {image && (
          <img
            src={`${RESOURCES_URL}/${image}`}
            alt={title}
            className="object-cover h-full rounded-md"
          />
        )}
      </div>
    </div>
  );
}
