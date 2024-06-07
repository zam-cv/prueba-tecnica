import React from 'react';

const Input = React.forwardRef<HTMLInputElement, React.InputHTMLAttributes<HTMLInputElement>>(
  (props, ref) => {
    return <input 
      className="p-3 rounded-md outline-none text-gray-200 bg-background-light text-base" 
      ref={ref} 
      {...props} 
    />;
  }
);

export default Input;