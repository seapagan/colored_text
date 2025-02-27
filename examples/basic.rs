use colored_text::Colorize;

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
    println!("{}", "Bright red text".bright_red());
    println!("{}", "Bright green text".bright_green());
    println!("{}", "Bright blue text".bright_blue());

    // Background colors
    println!("\nBackground colors:");
    println!("{}", "Red background".on_red());
    println!("{}", "Green background".on_green());
    println!("{}", "Blue background".on_blue());

    // Text styles
    println!("\nText styles:");
    println!("{}", "Bold text".bold());
    println!("{}", "Dim text".dim());
    println!("{}", "Italic text".italic());
    println!("{}", "Underlined text".underline());
    println!("{}", "Inverse text".inverse());
    println!("{}", "Strikethrough text".strikethrough());

    // RGB, HSL, and Hex colors
    println!("\nRGB, HSL, and Hex colors:");
    println!("{}", "Custom RGB color".rgb(255, 128, 0));
    println!("{}", "Custom RGB background".on_rgb(0, 128, 255));

    println!("{}", "Pure Red (HSL)".hsl(0.0, 100.0, 50.0));
    println!("{}", "Pure Green (HSL)".hsl(120.0, 100.0, 50.0));
    println!("{}", "Pure Blue (HSL)".hsl(240.0, 100.0, 50.0));
    println!("{}", "Pink (HSL)".hsl(350.0, 100.0, 75.0));
    println!("{}", "HSL Background".on_hsl(200.0, 100.0, 50.0));

    println!("{}", "Hex color (#ff8000)".hex("#ff8000"));
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
    println!("{}", format!("Hello, {}!", name.blue().bold()));

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

    // Disabling colors
    println!("\nDisabling colors by setting NO_COLOR environment variable:");
    std::env::set_var("NO_COLOR", "1");
    println!("{}", "This text should have no color".red().bold());
    std::env::remove_var("NO_COLOR");
}
