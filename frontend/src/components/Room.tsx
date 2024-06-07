import Timer from "./Timer";

export default function Room() {
  return (
    <div className="flex flex-col p-10 gap-7">
      <div>
        <h2 className="text-3xl font-bold text-center">Room</h2>
      </div>
      <div>
        <Timer duration={4000} />
      </div>
    </div>
  );
}
