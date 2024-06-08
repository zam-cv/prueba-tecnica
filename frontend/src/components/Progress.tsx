import { useState, useEffect } from 'react';

// duration is in seconds
export default function Progress({ duration, pause }: { duration: number, pause: boolean }) {
  const [startTime, setStartTime] = useState(Date.now());
  const [porcentage, setPorcentage] = useState(0);

  useEffect(() => {
    setStartTime(Date.now());
    setPorcentage(0);
  }, [duration]);

  useEffect(() => {
    if (pause) {
      return;
    }

    const updateProgress = () => {
      const elapsedTime = Date.now() - startTime;
      const newPorcentage = (elapsedTime / (duration * 1000)) * 100;

      if (newPorcentage >= 100) {
        setPorcentage(100);
      } else {
        setPorcentage(newPorcentage);
      }
    };

    const interval = setInterval(() => {
      if (!pause) {
        updateProgress();
      }
    }, 100);

    return () => clearInterval(interval);
  }, [startTime, duration, pause]);

  return (
    <div className="relative h-1">
      <div className="h-1 bg-white opacity-20 absolute w-full"></div>
      <div
        className="h-1 bg-red-500 absolute transition-all duration-300 ease-linear"
        style={{ width: `${porcentage}%` }}
      ></div>
    </div>
  );
}

