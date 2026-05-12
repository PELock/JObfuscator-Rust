/******************************************************************************
 * JObfuscator WebApi interface usage example.
 *
 * In this example we will verify our activation key status.
 *
 * Version        : v1.1.0
 * Language       : Rust
 * Author         : Bartosz Wójcik
 * Web page       : https://www.pelock.com
 *
 *****************************************************************************/

use jobfuscator::{JObfuscator, JObfuscatorResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // include JObfuscator class
    //

    //
    // if you don't want to use crates.io use a path dependency to this crate
    //
    // [dependencies]
    // jobfuscator = { path = ".." }

    //
    // create JObfuscator class instance (we are using our activation key)
    //
    let my_jobfuscator = JObfuscator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));

    //
    // login to the service
    //
    let result = my_jobfuscator.login(true).await;

    //
    // result object holds the information about the license
    //
    // result.demo          - is it a demo mode (invalid or empty activation key was used)
    // result.credits_left  - usage credits left after this operation
    // result.credits_total - total number of credits for this activation code
    // result.string_limit  - max. source code size allowed (it's 1500 bytes for demo mode)
    //
    match result {
        Some(JObfuscatorResponse::Object(obj)) => {
            println!(
                "Demo version status - {}",
                if obj.demo.unwrap_or(false) {
                    "true"
                } else {
                    "false"
                }
            );
            println!(
                "Usage credits left - {}",
                obj.credits_left.unwrap_or(0)
            );
            println!(
                "Total usage credits - {}",
                obj.credits_total.unwrap_or(0)
            );
            println!(
                "Max. source code size - {}",
                obj.string_limit.unwrap_or(0)
            );
        }
        Some(JObfuscatorResponse::Json(_)) => {}
        None => {
            return Err("Something unexpected happen while trying to login to the service.".into());
        }
    }

    Ok(())
}
