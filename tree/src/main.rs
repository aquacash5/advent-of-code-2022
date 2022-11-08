use console::style;

const TREE: &str = r"
        |
       \|/
      --*--
       >o<
      >O<<<
     >>o>>*<
    >o<<<o<<<
   >>@>*<<O<<<
  >o>>@>>>o>o<<
 >*>>*<o<@<o<<<<
>o>o<<<O<*>>*>>O<
   _ __| |__ _
";
const ORANGE: u8 = 166;

fn main() {
    println!(
        "{}",
        TREE.replace('|', &style("|").bold().white().to_string())
            .replace('_', &style("_").bold().white().to_string())
            .replace('*', &style("*").bold().yellow().to_string())
            .replacen('|', &style("|").bold().yellow().to_string(), 2)
            .replace('\\', &style("\\").bold().yellow().to_string())
            .replace('/', &style("/").bold().yellow().to_string())
            .replace('-', &style("-").bold().yellow().to_string())
            .replace('>', &style(">").bold().green().to_string())
            .replace('<', &style("<").bold().green().to_string())
            .replace('O', &style("O").bold().blue().to_string())
            .replace('@', &style("@").bold().red().to_string())
            .replace('o', &style("o").bold().color256(ORANGE).to_string())
    );
}
