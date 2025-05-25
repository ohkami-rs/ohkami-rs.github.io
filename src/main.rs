mod pages;

use uibeam::{UI, Beam};

struct Layout {
    children: UI,
}
impl Beam for Layout {
    fn render(self) -> UI {
        UI! {
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="./tailwind.css">
                <title>"Ohkami organization"</title>
            </head>
            <body>
                {self.children}
            </body>
            </html>
        }
    }
}

fn main() -> std::io::Result<()> {
    std::fs::remove_dir_all("./pages").ok(/* ok when not exists at the first time */);
    std::fs::create_dir("./pages")?;

    //$ tailwindcss -c tailwind.config.js -o ./pages/tailwind.css
    std::process::Command::new("tailwindcss")
        .args(["-c", "tailwind.config.js", "-o", "./pages/tailwind.css"])
        .status()?
        .success().then_some(()).expect("Failed to build Tailwind CSS");

    for (path, page) in pages::pages() {
        std::fs::write(
            format!("./pages{}.html", if *path == "/" {"/index"} else {path}),
            uibeam::shoot(UI! {
                <Layout>
                    {page()}
                </Layout>
            }).as_bytes()
        )?;
    }

    Ok(())
}
