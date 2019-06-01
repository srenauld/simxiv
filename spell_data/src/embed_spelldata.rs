use std::error::Error;
use std::fs::DirEntry;
use std::path::{PathBuf, Path};
use std::io::{ErrorKind, Error as IOError};
use crate::util::{get_option, get_options_from_input};
use crate::xiv_csv;

pub fn derive_embed_spelldata(input: &syn::LitStr) -> proc_macro2::TokenStream {

    fn bug() -> ! {
        panic!(
            "This is a bug. Please open a Github issue \
             with your invocation of `embed_spelldata!"
        );
    }

    let spelldata_expr = Ok(Path::new(&input.value())).and_then(|path| xiv_csv::path_to_actions(&path));
    let spelldata_expr = match spelldata_expr {
        Ok(v) => v,
        Err(e) => panic!("Error reading spelldata CSV: {:?}", e),
    };

    let output = quote! {
        {
            let mut entry = HashMap::new();
            #(#spelldata_expr)*
            entry
        }
    };
    //panic!(format!("{:?}", output));
    output.into()
}