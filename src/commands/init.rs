use crate::config::Config;
use crate::error::Result;
use crate::store::Store;
use colored::Colorize;

pub fn exec() -> Result<()> {
    let config = Config::default();
    let store = Store::new(&config);

    if store.exists() {
        println!("{} Store already initialized at {}", 
            "✓".green(), 
            store.root().display()
        );
        return Ok(());
    }

    // Create store directory
    store.init()?;

    // Create config file
    Config::save_default()?;

    println!("{} Initialized cmdx store at {}", 
        "✓".green(), 
        store.root().display()
    );
    println!("{} Config created at {}", 
        "✓".green(), 
        Config::config_path().display()
    );

    Ok(())
}
