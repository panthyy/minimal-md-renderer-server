/*
structure
<token> <value> (. | # ){<Value>}
    <scoped tailwind>
*/
/*

# Heading 1 .{main-heading nav} #{main-heading}
    font-semibold 2rem text-red-500
 */

const mkd = `
# Heading 1 .{main-heading nav} #{main-heading}
    font-semibold 2rem text-red-500
    `;

const map = {
  "font-semibold": "font-weight: 600;",
  "text-red-500": "color: #EF4444;",
  "2rem": "font-size: 2rem;",
};

const transpileTailwind = (tailwind: string) => {
  let id = Math.random().toString(36).substring(7);
  const tailwindClasses = tailwind.split(" ");
  let css = `.${id} {`;
  tailwindClasses.forEach((twClass) => {
    css += map[twClass];
  });
  css += "}";
  return { id, css };
};

const transpile = (mkd: string) => {
  const lines = mkd.split("\n");
  let html = "";
  let css = "";
  lines.forEach((line) => {
    const [token, value, tailwind] = line.split(" ");
    if (token === "#") {
      const { id, css: tailwindCss } = transpileTailwind(tailwind);
      css += tailwindCss;
      html += `<h1 class="${id}">${value}</h1>`;
    }
  });
  return { html, css };
};

console.log(transpile(mkd));
