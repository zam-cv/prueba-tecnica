import { useEffect, useState } from "react";
import { RESOURCES_URL } from "../utils/constants";
import api, { Room } from "../utils/api";
import { useNavigate } from "react-router-dom";

export function Card({ room }: { room: Room }) {
  const navigate = useNavigate();

  function handleClick() {
    navigate(`/room/${room.id}`);
  }

  return (
    <div
      onClick={handleClick}
      className="bg-background-dark rounded-sm max-md:container w-[550px] cursor-pointer"
    >
      <div className="h-[200px]">
        <img
          src={`${RESOURCES_URL}/${room.front_image}`}
          alt={room.title}
          className="object-cover w-full h-full rounded-t-sm"
        />
      </div>
      <div className="p-5">
        <h2 className="font-bold text-lg">{room.title}</h2>
        <div className="min-h-24 max-h-24 overflow-hidden">{room.description}</div>
        <div className="flex justify-end gap-2 pt-3">
          <p className="font-bold">Duración:</p>
          <div>
            {room.duration} minuto{room.duration > 1 ? "s" : ""}
          </div>
        </div>
      </div>
      <div className="w-full h-4 bg-cyan-500"></div>
    </div>
  );
}

export default function Rooms() {
  const [rooms, setRooms] = useState<Room[]>([]);

  useEffect(() => {
    api.rooms.getRooms().then((data) => {
      const room = data.map((r) => ({
        id: r[0],
        title: r[1],
        description: r[2],
        front_image: r[3],
        duration: r[4],
      }));

      setRooms(room);
    });
  }, []);

  return (
    <div>
      <div className="p-10 flex flex-wrap gap-10">
        {rooms.map((room) => (
          <Card key={room.id} room={room} />
        ))}
      </div>
    </div>
  );
}
