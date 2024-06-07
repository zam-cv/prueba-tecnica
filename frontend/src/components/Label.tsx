export default function Label(
  props: React.DetailedHTMLProps<
    React.LabelHTMLAttributes<HTMLLabelElement>,
    HTMLLabelElement
  >
) {
  return <label className="text-secondary text-sm" {...props} />;
}
