extern crate proc_macro;
use ethers_solc::{project_util::TempProject, ConfigurableArtifacts, ProjectCompileOutput};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn my_macro(_input: TokenStream) -> TokenStream {
    let error_msg = "My error message".to_string();
    let error_tokens = quote! {
        compile_error!(#error_msg);
    };

    error_tokens.into()
}

macro_rules! solme {
    ($source_code:expr) => {{
        /// Creates a temp project and compiles the files in it
        /// Note: returns the ownership of Project not to be dropped and deleted
        fn make_temp_project(// files: Vec<ProjectFile>,
        ) -> (TempProject<ConfigurableArtifacts>, ProjectCompileOutput) {
            let project = TempProject::<ConfigurableArtifacts>::dapptools().unwrap();
            project
                .add_source(String::from("solme"), $source_code)
                .unwrap();

            let compiled = project.compile().unwrap();

            if compiled.has_compiler_errors() {
                let errors = compiled.clone().output().errors;
                dbg!(&errors);
                quote!(::std::compile_error!("Compilation error"));
                return (project, compiled);
            } else {
                return (project, compiled);
            }
        }

        make_temp_project()
    }};
}

pub fn generate() {
    let (_project, output) = solme! {"pragma solidity 0.8.0;

contract Test {
    function hi() public {

    }
}"
    };

    let (_project, output) = solme! {"pragma solidity 0.8.0;

    contract Test {
        function
    }"
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        generate();
    }
}
