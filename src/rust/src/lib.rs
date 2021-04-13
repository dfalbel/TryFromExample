use extendr_api::*;
use std::convert::TryFrom;

pub struct LinearModel {
    coefs : Vec<f64>
}

impl TryFrom<Robj> for LinearModel {
    type Error = Error;
    fn try_from (robj: Robj) -> Result<Self> {
        if !robj.inherits("lm") {
            return Err(Error::Other(String::from("Expected object with class 'lm'")));
        } if let Ok(coef) = robj.dollar("coef") {
            match coef.as_real_vector() {
                Some(v) => Ok(Self{coefs: v}),
                None => Err(Error::Other(String::from("Coef is not numeric.")))
            }
        } else {
            Err(Error::Other(String::from("Can't find the 'coef' attribute.")))
        }    
    }
}

#[extendr]
fn show_coefs (model: LinearModel) {
    for v in model.coefs {
        println!("coef is {}", v);
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn show_coefs;
}
