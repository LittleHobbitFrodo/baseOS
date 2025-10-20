
//! runs the OS in emulator

mod util;
use std::borrow::Cow;
use std::io::{stdout, Write};
use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;
use toml::Value;
use util::*;

use crate::util::cmd::cmd;

type Params = Vec<String>;
type SpecificParams = HashMap<Arch, Params>;
type Variables = HashMap<String, String>;

/// Parameters given to qemu when the `qemu-parameters.toml` file does not exists
const QEMU_PARAMS_DEFAULT: &'static [&'static str] = &["-drive", "file={ISO},if=virtio,media=disk,discard=unmap"];

const QEMU_PARAMETERS_DEFAULT_DEBUG: &'static [&'static str] = QEMU_PARAMS_DEFAULT;


fn main() {

    let (archs, _, debug, verbose) = collect_args();


    let (default, specific, vars) = qemu_parameters(debug);

    for arch in archs.iter() {

        noteln!("running the emulator for the {} target", arch.triplet().blue());

        let params = parse_params_for(*arch, &default, &specific, &vars);

        run_qemu(*arch, &params);

    }

}

/// Loads and parses qemu parameters
/// - returns `(default, specific, variables)`
///   - `default` = default parameters (debug for debug profile, normal for normal)
///   - `specific` = target-specific parameters
///   - `vars` = user-defined variables
/// - variables are not resolved
//fn qemu_parameters(debug: bool) -> (Vec<String>, HashMap<String, Vec<String>>, Vec<(String, Vec<String>)>) {
fn qemu_parameters(debug: bool) -> (Params, SpecificParams, Variables) {

    let path = PATH.config.qemu_parameters();

    //  Read the file or use the default parameters
    let cfg: toml::Table = match read_toml(path) {
        Ok(c) => c,
        Err(e) => {
            //  use the defaults
            if let ConfigError::FailedToOpenFile(_) = e {
                error!("failed to open the {} file, using default parameters", "qemu-parameters.toml".blue());
                let default: Vec<String> = if debug {
                    QEMU_PARAMETERS_DEFAULT_DEBUG
                } else {
                    QEMU_PARAMS_DEFAULT
                }.iter().map(|p| p.to_string()).collect();

                return (default, HashMap::new(), HashMap::new());

            } else {
                failln!(internal: "could not load the qemu parameters: {e}");
            }
        }
    };

    let mut default: Params = Params::new();
    let mut specific: SpecificParams = SpecificParams::new();
    let mut vars: Variables = Variables::new();

    //  parse the loaded config
    for (name, table) in &cfg {

        //  check table name
        match name.as_str() {
            "vars" => {         //  add variables
                if let Value::Table(tab) = table {
                    
                    for (nm, v) in tab.iter() {

                        let str_val = match v {     //  make sure its a string
                            Value::String(s) => s,
                            _ => failln!(internal: "var {}: all variables must store {}", name.blue(), "strings".red()),
                        };

                        //  add the variable or report duplicit definitions
                        if let Some(_) = vars.insert(nm.clone(), str_val.clone()) {
                            failln!(internal: "found duplicit user-defined variable definitions for {}", name.blue());
                        }

                    }

                } else {
                    failln!(internal: "{} must be a table", "vars".blue());
                }
            },
            "default" => {      //  the default parameters for the emulator
                match validate_table_duo(&table, debug) {
                    Ok(params) => default = params,
                    Err(e) => failln!(internal: "failed to parse table {}: {e}", "default".blue()),
                }
            },
            _ => {              //  arch-specific parameters, errors
                if let Ok(a) = Arch::from_str(name) {
                    match validate_table_duo(&table, debug) {
                        Ok(params) => {
                            if let Some(_) = specific.insert(a, params) {
                                failln!(internal: "found duplicit target specific parameters for {}", name.blue());
                            }
                        },
                        Err(e) => failln!(internal: "failed to parse target specific parameters: {e}"),
                    }
                } else {
                    failln!(internal: "unknown target {} in the {} file", name.blue(), "qemu-parameters.toml".blue());
                }
            }
        }
    }

    return (default, specific, vars);




    /// Validates the normal/debug duo in an table
    /// - returns chosen parameters or an error with description
    fn validate_table_duo(table: &Value, debug: bool) -> Result<Vec<String>, String> {

        let key = if debug { "debug" } else { "normal" };

        if let Value::Table(tab) = table {
            if let Some(value) = tab.get(key) {
                if let Value::Array(arr) = value {
                    for i in arr.iter() {
                        if !i.is_str() { return Err(format!("unsupported type for \"{key}\", expected array of strings")) }
                    }
                    Ok(arr.iter().map(|x| x.as_str().unwrap().to_string() ).collect())
                } else {
                    return Err(format!("unsupported type for \"{key}\", expected array of strings"))
                }
            } else {
                return Err("expected table with \"normal\" and \"debug\"".into())
            }
        } else {
            return Err("expected table with \"normal\" and \"debug\"".into())
        }
    }

    
}


/// Evaluates all variables, chooses parameters to use and substitutes the variables
/// - panics upon failure
/// 
/// 1. Checks if target specific config exists for this target
///     - uses the `default` if not
/// 2. Copies and evaluates built-in and user-defined variables
/// 3. Substitutes the variables into parameters
fn parse_params_for<'l>(arch: Arch, default: &'l Params, specific: &'l SpecificParams, vars: &'l Variables) -> Params {

    //  choose parameters
    let mut params = match specific.get(&arch) {
        Some(p) => p.clone(),
        None => default.clone(),
    };

    //  evaluate variables
    let mut vars = vars.clone();
    let iso = PATH.forged.iso_string(arch).unwrap();

    let reg = Regex::new(r"\{([^}]+)\}").expect("failed to build regex");

    for (_, var) in vars.iter_mut() {
        *var = reg.replace_all(var, |caps: &regex::Captures| -> &str {
            let name = &caps[1];
            match name {
                "ISO" => iso.as_str(),
                "ARCH" => arch.normalize(),
                _ => failln!(internal: "unknown built-in variable {}", name.blue()),
            }
        }).to_string();
    }

    //  evaluate parameters
    for param in params.iter_mut() {
        *param = reg.replace_all(param, |caps: &regex::Captures| -> &str {
            let name = &caps[1];
            match name {
                "ISO" => iso.as_str(),
                "ARCH" => arch.normalize(),
                _ => {      //  index user-defined varialbes
                    if let Some(var) = vars.get(name) {
                        var.as_str()
                    } else {
                        failln!(internal: "unknown user-defined variable {}", name.blue());
                    }
                }
            }
        }).to_string();
    }


    params

}

/// Runs the emulator for `arch`, panics upon failure
fn run_qemu(arch: Arch, params: &Params) {

    let cfg = match ArchConfig::load(arch) {
        Ok(cfg) => cfg,
        Err(e) => failln!(internal: "failed to load target specific config: {e}"),
    };

    let qemu = &cfg.emulator;
    if qemu.is_empty() {
        failln!(internal: "no emulator for {} is set", arch.triplet().blue());
    }

    println!("qemu: {qemu}");

    match cmd(qemu, Some(params)) {
        Ok(status) => if !status.success() {
            failln!(internal: "failed to run qemu for {}", arch.triplet().blue());
        },
        Err(e) => failln!(internal: "failed to run qemu due to IO error: {e}"),
    }


}