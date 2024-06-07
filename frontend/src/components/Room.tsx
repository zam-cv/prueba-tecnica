import { useParams } from "react-router-dom";

export default function Room() {
  const { id } = useParams();

  return (
    <div className="p-5">
      <div>
        <h2 className="text-3xl font-bold text-center">Room</h2>
      </div>
      <h1>Room</h1>
      <p>Room ID: {id}</p>
    </div>
  );
}
