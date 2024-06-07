function Square({
  value,
  description,
}: {
  value: string;
  description: string;
}) {
  return (
    <div className="flex flex-col gap-5">
      <div className="relative">
        <div className="p-6 bg-background-light rounded-md">{value}</div>
        <div className="absolute rounded-full w-3 h-3 bg-background !-left-[6px] top-1/2 -mt-1"></div>
        <div className="absolute rounded-full w-3 h-3 bg-background !-right-[6px] top-1/2 -mt-1"></div>
      </div>
      <div className="text-xs text-center capitalize text-secondary">
        {description}
      </div>
    </div>
  );
}

export default function Room() {
  // const { id } = useParams();

  return (
    <div className="flex flex-col p-10 gap-7">
      <div>
        <h2 className="text-3xl font-bold text-center">Room</h2>
      </div>
      <div>
        <div className="flex justify-center">
          <div className="flex gap-5 text-xl">
            <Square value="01" description="Hora" />
            <Square value="03" description="Minuto" />
            <Square value="43" description="Segundo" />
          </div>
        </div>
      </div>
    </div>
  );
}
