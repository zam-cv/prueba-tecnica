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
