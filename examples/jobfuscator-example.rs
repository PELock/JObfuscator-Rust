/******************************************************************************
 * JObfuscator WebApi interface usage example.
 *
 * In this example we will obfuscate sample source with custom options.
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
    // when developing from a repository clone without installing the package use:
    //
    // use jobfuscator::{JObfuscator, JObfuscatorResponse};
    // (path dependency in Cargo.toml pointing to this crate)

    //
    // create JObfuscator class instance (we are using our activation key)
    //
    let mut my_jobfuscator = JObfuscator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));

    //
    // should the source code be compressed (both input & compressed)
    //
    my_jobfuscator.enable_compression = true;

    //
    // global obfuscation options
    //
    // when disabled will discard any @Obfuscate annotation declaration
    // in the Java source code
    //
    // you can disable a particular obfuscation strategy globally if it
    // fails or you don't want to use it without modifying the source codes
    //
    // by default all obfuscation strategies are enabled
    //

    //
    // change linear code execution flow to non-linear version
    //
    my_jobfuscator.mix_code_flow = true;

    //
    // rename variable names to random string values
    //
    my_jobfuscator.rename_variables = true;

    //
    // rename method names to random string values
    //
    my_jobfuscator.rename_methods = true;

    //
    // shuffle order of methods in the output source
    //
    my_jobfuscator.shuffle_methods = true;

    //
    // encrypt integers using more than 15 floating point math functions from the java.lang.Math.* class
    //
    my_jobfuscator.ints_math_crypt = true;

    //
    // encrypt strings using polymorphic encryption algorithms
    //
    my_jobfuscator.crypt_strings = true;

    //
    // for each method, extract all possible integers from the code and store them in an array
    //
    my_jobfuscator.ints_to_arrays = true;

    //
    // for each method, extract all possible doubles from the code and store them in an array
    //
    my_jobfuscator.dbls_to_arrays = true;

    //
    // strip comments when parsing (not toggled via @Obfuscate in source)
    //
    my_jobfuscator.remove_comments = true;

    //
    // encrypt doubles using java.lang.Math-style floating-point expressions
    //
    my_jobfuscator.dbls_math_crypt = true;

    //
    // string char vault strategy
    //
    my_jobfuscator.string_char_vault = true;

    //
    // encode integers using double-based math
    //
    my_jobfuscator.ints_from_double_math = true;

    //
    // opaque mixer chain
    //
    my_jobfuscator.opaque_mixer_chain = true;

    //
    // complexify boolean conditions
    //
    my_jobfuscator.complexify_booleans = true;

    //
    // try/finally noise injection
    //
    my_jobfuscator.try_finally_noise = true;

    //
    // array literal encryption (int, char, double, String)
    //
    my_jobfuscator.array_int_crypt = true;
    my_jobfuscator.array_char_crypt = true;
    my_jobfuscator.array_double_crypt = true;
    my_jobfuscator.array_string_crypt = true;

    //
    // source code in Java format
    //
    let source_code = r#"import java.util.*;
import java.lang.*;
import java.io.*;

//
// you must include custom annotation
// to enable entire class or a single
// method obfuscation
//
@Obfuscate
class Ideone
{
    //@Obfuscate
    public static double calculateSD(double numArray[])
    {
        double sum = 0.0, standardDeviation = 0.0;
        int length = numArray.length;

        for(double num : numArray) {
            sum += num;
        }

        double mean = sum/length;

        for(double num: numArray) {
            standardDeviation += Math.pow(num - mean, 2);
        }

        return Math.sqrt(standardDeviation/length);
    }

    //
    // selective obfuscation strategies
    // can be applied for the entire
    // class or a single method (by
    // default all obfuscation strategies
    // are enabled when you use @Obfuscate
    // annotation alone)
    //
    //@Obfuscate(
    //  array_int_crypt = true,
    //  array_char_crypt = true,
    //  array_double_crypt = true,
    //  array_string_crypt = true,
    //  crypt_strings = true,
    //  rename_methods = false,
    //  rename_variables = true,
    //  shuffle_methods = true,
    //  ints_math_crypt = true,
    //  dbls_math_crypt = true,
    //  mix_code_flow = true,
    //  string_char_vault = true,
    //  ints_from_double_math = true,
    //  opaque_mixer_chain = true,
    //  complexify_booleans = true,
    //  try_finally_noise = true,
    //  ints_to_arrays = true,
    //  dbls_to_arrays = true
    // )
    public static void main(String[] args) {

        double[] numArray = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
        double SD = calculateSD(numArray);

        System.out.format("Standard Deviation = %.6f", SD);
    }
}"#;

    //
    // by default all options are enabled, both helper random numbers
    // generation & obfuscation strategies, so we can just simply call:
    //
    let result = my_jobfuscator
        .obfuscate_java_source(source_code, true)
        .await;

    //
    // it's also possible to pass a Java source file path instead of a string e.g.
    //
    // let result = my_jobfuscator
    //     .obfuscate_java_file(std::path::Path::new("/path/to/project/source.java"), true)
    //     .await;

    //
    // result object holds the obfuscation results as well as other information
    //
    // result.error         - error code
    // result.output        - obfuscated code
    // result.demo          - was it used in demo mode (invalid or empty activation key was used)
    // result.credits_left  - usage credits left after this operation
    // result.credits_total - total number of credits for this activation code
    // result.expired       - if this was the last usage credit for the activation key it will be set to true
    //
    match result {
        Some(JObfuscatorResponse::Object(obj)) => {
            //
            // display obfuscated code
            //
            if obj.error == JObfuscator::ERROR_SUCCESS {
                println!("{}", obj.output.unwrap_or_default());
            } else {
                return Err(
                    format!("An error occurred, error code: {}", obj.error).into(),
                );
            }
        }
        Some(JObfuscatorResponse::Json(_)) => {}
        None => {
            return Err("Something unexpected happen while trying to obfuscate the code.".into());
        }
    }

    Ok(())
}
