use colored_text::{ColorDepthMode, ColorMode, Colorize, ColorizeConfig, RenderTarget};

fn main() {
    // Basic colors
    println!("\nBasic colors:");
    println!("{}", "Red text".red());
    println!("{}", "Green text".green());
    println!("{}", "Blue text".blue());
    println!("{}", "Yellow text".yellow());
    println!("{}", "Magenta text".magenta());
    println!("{}", "Cyan text".cyan());
    println!("{}", "White text".white());
    println!("{}", "Black text".black());

    // Bright colors
    println!("\nBright colors:");
    println!("{}", "Bright black text".bright_black());
    println!("{}", "Bright red text".bright_red());
    println!("{}", "Bright green text".bright_green());
    println!("{}", "Bright blue text".bright_blue());

    // Background colors
    println!("\nBackground colors:");
    println!("{}", "Red background".on_red());
    println!("{}", "Green background".on_green());
    println!("{}", "Blue background".on_blue());
    println!("{}", "Bright black background".on_bright_black());
    println!("{}", "Bright red background".on_bright_red());
    println!("{}", "Bright blue background".on_bright_blue());

    // Text styles
    println!("\nText styles:");
    println!("{}", "Bold text".bold());
    println!("{}", "Dim text".dim());
    println!("{}", "Italic text".italic());
    println!("{}", "Underlined text".underline());
    println!("{}", "Inverse text".inverse());
    println!("{}", "Strikethrough text".strikethrough());

    // ANSI 256, RGB, HSL, and Hex colors
    println!("\nANSI 256, RGB, HSL, and Hex colors:");
    println!("{}", "ANSI 256 color".ansi256(208));
    println!("{}", "ANSI 256 background".on_ansi256(236));
    println!("{}", "ANSI 256 aliases".color256(208).on_color256(236));
    println!("{}", "Custom RGB color".rgb(255, 128, 0));
    println!("{}", "Custom RGB background".on_rgb(0, 128, 255));

    println!("{}", "Pure Red (HSL)".hsl(0.0, 100.0, 50.0));
    println!("{}", "Pure Green (HSL)".hsl(120.0, 100.0, 50.0));
    println!("{}", "Pure Blue (HSL)".hsl(240.0, 100.0, 50.0));
    println!("{}", "Pink (HSL)".hsl(350.0, 100.0, 75.0));
    println!("{}", "HSL Background".on_hsl(200.0, 100.0, 50.0));

    println!("{}", "Hex color (#ff8000)".hex("#ff8000"));
    println!("{}", "Hex without # (ff8000)".hex("ff8000"));
    println!("{}", "Hex shorthand (#f80)".hex("#f80"));
    println!("{}", "Hex shorthand without # (f80)".hex("f80"));
    println!("{}", "Hex background (#0080ff)".on_hex("#0080ff"));

    // Chaining styles
    println!("\nChained styles:");
    println!("{}", "Bold red text".red().bold());
    println!(
        "{}",
        "Italic blue text on yellow background"
            .blue()
            .italic()
            .on_yellow()
    );
    println!("{}", "RGB text with background".rgb(255, 128, 0).on_blue());

    // Using with format! macro
    println!("\nUsing with format! macro:");
    let name = "World";
    println!("Hello, {}!", name.blue().bold());

    // Using with String
    println!("\nUsing with String:");
    let message = String::from("This is a String");
    println!("{}", message.green().underline());

    // Mixing styles in a single line
    println!("\nMixing styles:");
    println!(
        "{}. {} {} {}!",
        "Notice".red().bold(),
        "This".blue(),
        "is".green(),
        "important".yellow().underline()
    );

    // Runtime color modes
    println!("\nRuntime color modes:");
    let caps = ColorizeConfig::terminal_capabilities(RenderTarget::Stdout);
    println!("stdout is terminal: {}", caps.is_terminal);
    println!("stdout color level: {:?}", caps.color_level);

    ColorizeConfig::set_color_mode(ColorMode::Always);
    println!("{}", "Forced color".red().bold());
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::TrueColor);
    println!("{}", "RGB as truecolor".rgb(255, 128, 0));
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi256);
    println!("{}", "RGB degraded to ANSI 256".rgb(255, 128, 0));
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi16);
    println!("{}", "RGB degraded to ANSI 16".rgb(255, 128, 0));
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::NoColor);
    println!("{}", "RGB rendered plain".rgb(255, 128, 0));
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::Auto);
    ColorizeConfig::set_color_mode(ColorMode::Never);
    println!("{}", "Forced plain output".red().bold());
    ColorizeConfig::set_color_mode(ColorMode::Auto);
}
