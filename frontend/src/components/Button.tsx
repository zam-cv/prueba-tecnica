export default function Button(
  props: React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  >
) {
  return (
    <button
      className="p-3 rounded-md outline-none text-white bg-transparent border hover:bg-opacity"
      {...props}
    />
  );
}

export function ButtonPrimary(
  props: React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  >
) {
  return (
    <Button
      className="p-3 px-5 rounded-md outline-none bg-red-600 hover:bg-red-500"
      {...props}
    />
  );
}
