/******************************************************************************
 * JObfuscator WebApi interface
 *
 * Version        : v1.1.0
 * Language       : Rust
 * Author         : Bartosz Wójcik
 * Web page       : https://www.pelock.com
 *
 *****************************************************************************/

use base64::engine::general_purpose::STANDARD as B64_ENGINE;
use base64::Engine;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};

/// Error code (see [`JObfuscator::ERROR_SUCCESS`] and related constants).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JObfuscatorResult {
    /// Error code (see `ERROR_*` constants on [`JObfuscator`]).
    pub error: i64,
    /// Obfuscated source when error is [`JObfuscator::ERROR_SUCCESS`].
    #[serde(default)]
    pub output: Option<String>,
    /// Demo mode when key is invalid or empty.
    #[serde(default)]
    pub demo: Option<bool>,
    /// Credits remaining after the operation.
    #[serde(default, rename = "credits_left")]
    pub credits_left: Option<i64>,
    /// Total credits for the activation key.
    #[serde(default, rename = "credits_total")]
    pub credits_total: Option<i64>,
    /// True when the last credit was used.
    #[serde(default)]
    pub expired: Option<bool>,
    /// Max. source size (bytes), e.g. demo limit.
    #[serde(default, rename = "string_limit")]
    pub string_limit: Option<i64>,
}

/// Parsed API response: either a structured object or the raw JSON string (when `return_as_object` is `false`).
#[derive(Debug, Clone)]
pub enum JObfuscatorResponse {
    /// Structured result (after optional output decompression).
    Object(JObfuscatorResult),
    /// JSON encoded string (original body, or re-encoded after decompression).
    Json(String),
}

/// JObfuscator WebApi client.
#[derive(Debug, Clone)]
pub struct JObfuscator {
    api_key: Option<String>,
    client: reqwest::Client,
    // Whether to request comment stripping on the API (not exposed; matches product defaults).
    remove_comments: bool,
    /// Should the source code be compressed.
    pub enable_compression: bool,
    /// Change linear code execution flow to non-linear version.
    pub mix_code_flow: bool,
    /// Rename variable names to random string values.
    pub rename_variables: bool,
    /// Rename method names to random string values.
    pub rename_methods: bool,
    /// Shuffle methods order in the output source.
    pub shuffle_methods: bool,
    /// Encrypt integers using more than 15 floating point math functions from the java.lang.Math.* class.
    pub ints_math_crypt: bool,
    /// Encrypt strings using polymorphic encryption algorithms.
    pub crypt_strings: bool,
    /// For each method, extract all possible integers from the code and store them in an array.
    pub ints_to_arrays: bool,
    /// For each method, extract all possible doubles from the code and store them in an array.
    pub dbls_to_arrays: bool,
    /// Encrypt doubles using floating-point math functions from `java.lang.Math`.
    pub dbls_math_crypt: bool,
    /// Store string material in a generated char vault.
    pub string_char_vault: bool,
    /// Encode integer literals using double-based math expressions.
    pub ints_from_double_math: bool,
    /// Apply opaque mixer chain obfuscation.
    pub opaque_mixer_chain: bool,
    /// Replace boolean sub-expressions with harder-to-read equivalents.
    pub complexify_booleans: bool,
    /// Inject try/finally noise around code regions.
    pub try_finally_noise: bool,
    /// Obfuscate int array initializer content.
    pub array_int_crypt: bool,
    /// Obfuscate char array initializer content.
    pub array_char_crypt: bool,
    /// Obfuscate double array initializer content.
    pub array_double_crypt: bool,
    /// Obfuscate `String` array initializer content.
    pub array_string_crypt: bool,
}

impl JObfuscator {
    /// Default JObfuscator WebApi endpoint.
    pub const API_URL: &'static str = "https://www.pelock.com/api/jobfuscator/v1";

    /// Success.
    pub const ERROR_SUCCESS: i64 = 0;

    /// Invalid size for source code (it's 1500 bytes max. for demo version).
    pub const ERROR_INPUT_SIZE: i64 = 1;

    /// Input source is empty.
    pub const ERROR_INPUT: i64 = 2;

    /// Java source code parsing error.
    pub const ERROR_PARSING: i64 = 3;

    /// Java parsed code obfuscation error.
    pub const ERROR_OBFUSCATION: i64 = 4;

    /// An error occurred while generating output code.
    pub const ERROR_OUTPUT: i64 = 5;

    /// Initialize JObfuscator client.
    ///
    /// `api_key` — activation key for the service (it can be empty for demo mode).
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            remove_comments: true,
            enable_compression: true,
            mix_code_flow: true,
            rename_variables: true,
            rename_methods: true,
            shuffle_methods: true,
            ints_math_crypt: true,
            crypt_strings: true,
            ints_to_arrays: true,
            dbls_to_arrays: true,
            dbls_math_crypt: true,
            string_char_vault: true,
            ints_from_double_math: true,
            opaque_mixer_chain: true,
            complexify_booleans: true,
            try_finally_noise: true,
            array_int_crypt: true,
            array_char_crypt: true,
            array_double_crypt: true,
            array_string_crypt: true,
        }
    }

    /// Login to the service and get the information about the current license limits.
    ///
    /// `return_as_object` — return result as an object or JSON encoded string.
    pub async fn login(&self, return_as_object: bool) -> Option<JObfuscatorResponse> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "login".to_string());
        self.post_request(params, return_as_object).await
    }

    /// Obfuscate Java source code file using provided parameters.
    ///
    /// `return_as_object` — return result as an object or JSON encoded string.
    pub async fn obfuscate_java_file(
        &self,
        java_file_path: &std::path::Path,
        return_as_object: bool,
    ) -> Option<JObfuscatorResponse> {
        let source = tokio::fs::read_to_string(java_file_path).await.ok()?;
        if source.is_empty() {
            return None;
        }
        self.obfuscate_java_source(&source, return_as_object).await
    }

    /// Obfuscate Java source code (string) using provided parameters.
    ///
    /// `return_as_object` — return result as an object or JSON encoded string.
    pub async fn obfuscate_java_source(
        &self,
        java_source: &str,
        return_as_object: bool,
    ) -> Option<JObfuscatorResponse> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "obfuscate".to_string());
        params.insert("source".to_string(), java_source.to_string());
        self.post_request(params, return_as_object).await
    }

    /// Send a POST request to the server.
    async fn post_request(
        &self,
        mut params: HashMap<String, String>,
        return_as_object: bool,
    ) -> Option<JObfuscatorResponse> {
        //
        // add activation key to the parameters array
        //
        if let Some(ref key) = self.api_key {
            if !key.is_empty() {
                params.insert("key".to_string(), key.clone());
            }
        }

        //
        // obfuscation strategies
        //
        if self.mix_code_flow {
            params.insert("mix_code_flow".to_string(), "1".to_string());
        }
        if self.rename_variables {
            params.insert("rename_variables".to_string(), "1".to_string());
        }
        if self.rename_methods {
            params.insert("rename_methods".to_string(), "1".to_string());
        }
        if self.shuffle_methods {
            params.insert("shuffle_methods".to_string(), "1".to_string());
        }
        if self.ints_math_crypt {
            params.insert("ints_math_crypt".to_string(), "1".to_string());
        }
        if self.crypt_strings {
            params.insert("crypt_strings".to_string(), "1".to_string());
        }
        if self.ints_to_arrays {
            params.insert("ints_to_arrays".to_string(), "1".to_string());
        }
        if self.dbls_to_arrays {
            params.insert("dbls_to_arrays".to_string(), "1".to_string());
        }
        if self.remove_comments {
            params.insert("remove_comments".to_string(), "1".to_string());
        }
        if self.dbls_math_crypt {
            params.insert("dbls_math_crypt".to_string(), "1".to_string());
        }
        if self.string_char_vault {
            params.insert("string_char_vault".to_string(), "1".to_string());
        }
        if self.ints_from_double_math {
            params.insert("ints_from_double_math".to_string(), "1".to_string());
        }
        if self.opaque_mixer_chain {
            params.insert("opaque_mixer_chain".to_string(), "1".to_string());
        }
        if self.complexify_booleans {
            params.insert("complexify_booleans".to_string(), "1".to_string());
        }
        if self.try_finally_noise {
            params.insert("try_finally_noise".to_string(), "1".to_string());
        }
        if self.array_int_crypt {
            params.insert("array_int_crypt".to_string(), "1".to_string());
        }
        if self.array_char_crypt {
            params.insert("array_char_crypt".to_string(), "1".to_string());
        }
        if self.array_double_crypt {
            params.insert("array_double_crypt".to_string(), "1".to_string());
        }
        if self.array_string_crypt {
            params.insert("array_string_crypt".to_string(), "1".to_string());
        }

        //
        // check if compression is enabled
        //
        if self.enable_compression {
            if let Some(source) = params.get("source").cloned() {
                let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
                enc.write_all(source.as_bytes()).ok()?;
                let compressed = enc.finish().ok()?;
                params.insert("source".to_string(), B64_ENGINE.encode(compressed));
                params.insert("compression".to_string(), "1".to_string());
            }
        }

        let mut form = reqwest::multipart::Form::new();
        for (k, v) in params {
            form = form.text(k, v);
        }

        let response_text = self
            .client
            .post(Self::API_URL)
            .header("User-Agent", "PELock JObfuscator")
            .multipart(form)
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

        //
        // check the result
        //
        if response_text.is_empty() {
            return None;
        }

        let mut result: JObfuscatorResult = serde_json::from_str(&response_text).ok()?;

        //
        // depack output code back into the string
        //
        let mut depacked = false;

        if self.enable_compression
            && result.error == Self::ERROR_SUCCESS
            && result.output.as_ref().is_some_and(|s| !s.is_empty())
        {
            let b64 = result.output.as_ref()?;
            let buf = B64_ENGINE.decode(b64.as_bytes()).ok()?;
            let mut decoder = ZlibDecoder::new(&buf[..]);
            let mut out = String::new();
            decoder.read_to_string(&mut out).ok()?;
            result.output = Some(out);
            depacked = true;
        }

        if return_as_object {
            return Some(JObfuscatorResponse::Object(result));
        }

        //
        // if output was depacked - pack it again to JSON format
        //
        if depacked {
            let s = serde_json::to_string(&result).ok()?;
            return Some(JObfuscatorResponse::Json(s));
        }

        //
        // return original JSON response code
        //
        Some(JObfuscatorResponse::Json(response_text))
    }
}
