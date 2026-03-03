declare var CmarkGFM: any;
export function renderMarkdown(md: string) {
  if (!md) return "";
  return CmarkGFM.convert(md.replace(/^ +/gm, ""));
}
