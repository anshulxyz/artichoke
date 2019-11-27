use artichoke_core::eval::Eval;

use crate::{Artichoke, ArtichokeError};

pub fn init(interp: &Artichoke) -> Result<(), ArtichokeError> {
    interp.0.borrow_mut().def_class::<Hash>("Hash", None, None);
    interp.eval(&include_bytes!("hash.rb")[..])?;
    Ok(())
}

pub struct Hash;
