import { useState, useEffect } from "react";

export function Square({
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

// duration is in seconds
export default function Timer({ duration }: { duration: number }) {
  const [timeLeft, setTimeLeft] = useState(duration);

  useEffect(() => {
    const interval = setInterval(() => {
      if (timeLeft <= 0) {
        clearInterval(interval);
      } else {
        setTimeLeft(timeLeft - 1);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [timeLeft]);

  const hours = Math.floor(timeLeft / 3600);
  const minutes = Math.floor((timeLeft % 3600) / 60);
  const seconds = timeLeft % 60;

  return (
    <div className="flex justify-center">
      <div className="flex gap-5 text-xl">
        <Square
          value={hours.toString().padStart(2, "0")}
          description={"Hora" + (hours > 1 ? "s" : "")}
        />
        <Square
          value={minutes.toString().padStart(2, "0")}
          description={"Minuto" + (minutes > 1 ? "s" : "")}
        />
        <Square
          value={seconds.toString().padStart(2, "0")}
          description={"Segundo" + (seconds > 1 ? "s" : "")}
        />
      </div>
    </div>
  );
}
